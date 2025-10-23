//
//  papyr_core
//  backends/escl.rs - eSCL (AirPrint/AirScan) backend
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use crate::models::*;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const ESCL_SERVICE: &str = "_uscan._tcp.local.";
const ESCL_SECURE_SERVICE: &str = "_uscans._tcp.local.";

pub struct EsclBackend {
    discovered_scanners: Arc<Mutex<HashMap<String, EsclDevice>>>,
    client: Client,
}

#[derive(Clone)]
struct EsclDevice {
    id: String,
    name: String,
    host: String,
    port: u16,
    use_https: bool,
}

impl EsclDevice {
    fn base_url(&self) -> String {
        let protocol = if self.use_https { "https" } else { "http" };
        format!("{}://{}:{}/eSCL", protocol, self.host, self.port)
    }
}

impl EsclBackend {
    pub fn new() -> Self {
        Self {
            discovered_scanners: Arc::new(Mutex::new(HashMap::new())),
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .danger_accept_invalid_certs(true) // Many printers use self-signed certs
                .build()
                .unwrap(),
        }
    }

    fn discover_scanners(&self) -> Result<Vec<ScannerInfo>> {
        // Use a simple blocking approach with a timeout
        // This avoids nested runtime issues
        use std::sync::mpsc::channel;
        use std::thread;

        let (tx, rx) = channel();
        let discovered = Arc::clone(&self.discovered_scanners);

        // Spawn a separate thread to handle mDNS discovery
        thread::spawn(move || {
            // Create a minimal runtime just for this thread
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(_) => {
                    let _ = tx.send(Vec::new());
                    return;
                }
            };

            let result = rt.block_on(async {
                let mdns = match ServiceDaemon::new() {
                    Ok(mdns) => mdns,
                    Err(_) => return Vec::new(),
                };

                let receiver = match mdns.browse(ESCL_SERVICE) {
                    Ok(r) => r,
                    Err(_) => return Vec::new(),
                };

                let mut scanners = Vec::new();
                let timeout = tokio::time::sleep(Duration::from_secs(3));
                tokio::pin!(timeout);

                loop {
                    tokio::select! {
                        event = receiver.recv_async() => {
                            match event {
                                Ok(ServiceEvent::ServiceResolved(info)) => {
                                    let name = info.get_fullname().trim_end_matches('.').to_string();
                                    let host = info.get_addresses().iter().next()
                                        .map(|addr| addr.to_string())
                                        .unwrap_or_default();
                                    let port = info.get_port();

                                    if !host.is_empty() {
                                        let device = EsclDevice {
                                            id: format!("escl_{}", host.replace('.', "_")),
                                            name: name.clone(),
                                            host,
                                            port,
                                            use_https: false,
                                        };

                                        scanners.push(ScannerInfo {
                                            id: device.id.clone(),
                                            name: device.name.clone(),
                                            backend: Backend::Escl,
                                        });

                                        if let Ok(mut disc) = discovered.lock() {
                                            disc.insert(device.id.clone(), device);
                                        }
                                    }
                                }
                                Ok(_) => {},
                                Err(_) => break,
                            }
                        }
                        _ = &mut timeout => {
                            break;
                        }
                    }
                }

                let _ = mdns.shutdown();
                scanners
            });

            let _ = tx.send(result);
        });

        // Wait for discovery to complete (with timeout)
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(scanners) => Ok(scanners),
            Err(_) => Ok(Vec::new()), // Timeout, return empty list
        }
    }

    async fn get_capabilities_async(&self, device: &EsclDevice) -> Result<Capabilities> {
        let url = format!("{}/ScannerCapabilities", device.base_url());

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| PapyrError::Backend(format!("Failed to fetch capabilities: {}", e)))?;

        if !response.status().is_success() {
            return Err(PapyrError::Backend(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let xml_text = response
            .text()
            .await
            .map_err(|e| PapyrError::Backend(format!("Failed to read response: {}", e)))?;

        self.parse_capabilities(&xml_text)
    }

    fn parse_capabilities(&self, xml: &str) -> Result<Capabilities> {
        use quick_xml::de::from_str;

        let caps: ScannerCapabilities = from_str(xml)
            .map_err(|e| PapyrError::Backend(format!("Failed to parse capabilities: {}", e)))?;

        let mut sources = Vec::new();
        let mut dpis = Vec::new();
        let mut color_modes = Vec::new();
        let mut _max_width = 0;
        let mut _max_height = 0;

        // Parse platen (flatbed) capabilities
        if let Some(platen) = caps.platen {
            sources.push(ScanSource::Flatbed);

            if let Some(input_caps) = platen.platen_input_caps {
                _max_width = input_caps.max_width.unwrap_or(0);
                _max_height = input_caps.max_height.unwrap_or(0);

                if let Some(profiles) = input_caps.setting_profiles {
                    if let Some(profile) = profiles.setting_profile.first() {
                        // Parse resolutions
                        if let Some(resolutions) = &profile.supported_resolutions {
                            if let Some(discrete) = &resolutions.discrete_resolutions {
                                for res in &discrete.discrete_resolution {
                                    if !dpis.contains(&res.x_resolution) {
                                        dpis.push(res.x_resolution);
                                    }
                                }
                            }
                        }

                        // Parse color modes
                        if let Some(modes) = &profile.color_modes {
                            for mode in &modes.color_mode {
                                match mode.as_str() {
                                    "RGB24" | "RGB48" => {
                                        if !color_modes.contains(&ColorMode::Color) {
                                            color_modes.push(ColorMode::Color);
                                        }
                                    }
                                    "Grayscale8" | "Grayscale16" => {
                                        if !color_modes.contains(&ColorMode::Gray) {
                                            color_modes.push(ColorMode::Gray);
                                        }
                                    }
                                    "BlackAndWhite1" => {
                                        if !color_modes.contains(&ColorMode::Bw) {
                                            color_modes.push(ColorMode::Bw);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        // Parse ADF capabilities
        let supports_duplex = if let Some(adf) = caps.adf {
            sources.push(ScanSource::Adf);

            if let Some(simplex) = adf.adf_simplex {
                simplex.adf_simplex_input_caps.is_some()
            } else {
                false
            }
        } else {
            false
        };

        if supports_duplex {
            sources.push(ScanSource::AdfDuplex);
        }

        // Ensure we have reasonable defaults
        if dpis.is_empty() {
            dpis = vec![75, 150, 300, 600];
        }
        if color_modes.is_empty() {
            color_modes = vec![ColorMode::Color, ColorMode::Gray, ColorMode::Bw];
        }

        Ok(Capabilities {
            sources,
            dpis,
            color_modes,
            page_sizes: vec![
                PageSize {
                    width_mm: 216,
                    height_mm: 279,
                }, // Letter
                PageSize {
                    width_mm: 210,
                    height_mm: 297,
                }, // A4
            ],
            supports_duplex,
        })
    }

    async fn start_scan_async(
        &self,
        device: &EsclDevice,
        config: ScanConfig,
    ) -> Result<Box<dyn ScanSession>> {
        let url = format!("{}/ScanJobs", device.base_url());
        let scan_settings = create_scan_settings_xml(&config);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "text/xml")
            .body(scan_settings)
            .send()
            .await
            .map_err(|e| PapyrError::Backend(format!("Failed to start scan: {}", e)))?;

        if response.status().as_u16() != 201 {
            return Err(PapyrError::Backend(format!(
                "Scan start failed: {}",
                response.status()
            )));
        }

        let location = response
            .headers()
            .get("Location")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| PapyrError::Backend("No Location header in scan response".into()))?
            .to_string();

        Ok(Box::new(EsclScanSession {
            device: device.clone(),
            job_location: location,
            client: self.client.clone(),
            state: ScanSessionState::Scanning,
            current_page: 0,
        }))
    }
}

impl BackendProvider for EsclBackend {
    fn name(&self) -> &'static str {
        "eSCL/AirScan"
    }

    fn kind(&self) -> Backend {
        Backend::Escl
    }

    fn enumerate(&self) -> Vec<ScannerInfo> {
        self.discover_scanners().unwrap_or_default()
    }

    fn capabilities(&self, device_id: &str) -> Result<Capabilities> {
        let discovered = self.discovered_scanners.lock().unwrap();
        let device = discovered
            .get(device_id)
            .ok_or_else(|| PapyrError::NotFound(device_id.to_string()))?
            .clone();
        drop(discovered);

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PapyrError::Backend(format!("Failed to create runtime: {}", e)))?;

        rt.block_on(self.get_capabilities_async(&device))
    }

    fn start_scan(&self, device_id: &str, cfg: ScanConfig) -> Result<Box<dyn ScanSession>> {
        let discovered = self.discovered_scanners.lock().unwrap();
        let device = discovered
            .get(device_id)
            .ok_or_else(|| PapyrError::NotFound(device_id.to_string()))?
            .clone();
        drop(discovered);

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PapyrError::Backend(format!("Failed to create runtime: {}", e)))?;

        rt.block_on(self.start_scan_async(&device, cfg))
    }
}

struct EsclScanSession {
    device: EsclDevice,
    job_location: String,
    client: Client,
    state: ScanSessionState,
    current_page: u32,
}

enum ScanSessionState {
    Scanning,
    Ready,
    Complete,
}

impl ScanSession for EsclScanSession {
    fn next_event(&mut self) -> Result<Option<ScanEvent>> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PapyrError::Backend(format!("Failed to create runtime: {}", e)))?;

        rt.block_on(async {
            match self.state {
                ScanSessionState::Scanning => {
                    // Poll for job completion
                    for _ in 0..30 {
                        let status_url = format!("{}/ScannerStatus", self.device.base_url());

                        if let Ok(response) = self.client.get(&status_url).send().await {
                            if let Ok(xml) = response.text().await {
                                if xml.contains("Completed")
                                    || xml.contains("JobCompletedSuccessfully")
                                {
                                    self.state = ScanSessionState::Ready;
                                    return Ok(Some(ScanEvent::PageStarted(self.current_page)));
                                }

                                if xml.contains("Error") || xml.contains("Stopped") {
                                    return Err(PapyrError::Backend("Scan job failed".into()));
                                }
                            }
                        }

                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }

                    Err(PapyrError::Backend("Scan timeout".into()))
                }
                ScanSessionState::Ready => {
                    // Download the page
                    let download_url = format!("{}/NextDocument", self.job_location);

                    let response =
                        self.client.get(&download_url).send().await.map_err(|e| {
                            PapyrError::Backend(format!("Failed to download: {}", e))
                        })?;

                    if response.status().as_u16() == 404 {
                        // No more pages
                        self.state = ScanSessionState::Complete;
                        return Ok(Some(ScanEvent::JobComplete));
                    }

                    if !response.status().is_success() {
                        return Err(PapyrError::Backend(format!(
                            "Download failed: {}",
                            response.status()
                        )));
                    }

                    let data = response
                        .bytes()
                        .await
                        .map_err(|e| PapyrError::Backend(format!("Failed to read data: {}", e)))?
                        .to_vec();

                    let _page_meta = PageMeta {
                        index: self.current_page,
                        width_px: 0, // Would need to parse image to get actual dimensions
                        height_px: 0,
                        dpi: 300, // Default, should come from config
                        color_mode: ColorMode::Color,
                    };

                    self.current_page += 1;
                    self.state = ScanSessionState::Scanning; // Check for more pages

                    Ok(Some(ScanEvent::PageData(data)))
                }
                ScanSessionState::Complete => Ok(None),
            }
        })
    }
}

fn create_scan_settings_xml(config: &ScanConfig) -> String {
    let color_mode = match config.color_mode {
        ColorMode::Color => "RGB24",
        ColorMode::Gray => "Grayscale8",
        ColorMode::Bw => "BlackAndWhite1",
    };

    let input_source = match config.source {
        ScanSource::Flatbed => "Platen",
        ScanSource::Adf | ScanSource::AdfDuplex => "Feeder",
    };

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<scan:ScanSettings xmlns:scan="http://schemas.hp.com/imaging/escl/2011/05/03" xmlns:pwg="http://www.pwg.org/schemas/2010/12/sm">
    <pwg:Version>2.6</pwg:Version>
    <pwg:ScanRegions>
        <pwg:ScanRegion>
            <pwg:Height>{}</pwg:Height>
            <pwg:Width>{}</pwg:Width>
            <pwg:XOffset>0</pwg:XOffset>
            <pwg:YOffset>0</pwg:YOffset>
        </pwg:ScanRegion>
    </pwg:ScanRegions>
    <scan:DocumentFormatExt>image/jpeg</scan:DocumentFormatExt>
    <pwg:InputSource>{}</pwg:InputSource>
    <scan:XResolution>{}</scan:XResolution>
    <scan:YResolution>{}</scan:YResolution>
    <scan:ColorMode>{}</scan:ColorMode>
</scan:ScanSettings>"#,
        (config.page_size.height_mm * config.dpi) / 25,
        (config.page_size.width_mm * config.dpi) / 25,
        input_source,
        config.dpi,
        config.dpi,
        color_mode
    )
}

// XML deserialization structures
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ScannerCapabilities {
    platen: Option<PlatenCaps>,
    adf: Option<AdfCaps>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PlatenCaps {
    platen_input_caps: Option<InputCaps>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AdfCaps {
    adf_simplex: Option<AdfSimplex>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AdfSimplex {
    adf_simplex_input_caps: Option<InputCaps>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct InputCaps {
    min_width: Option<u32>,
    max_width: Option<u32>,
    min_height: Option<u32>,
    max_height: Option<u32>,
    setting_profiles: Option<SettingProfiles>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SettingProfiles {
    #[serde(rename = "SettingProfile", default)]
    setting_profile: Vec<SettingProfile>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SettingProfile {
    color_modes: Option<ColorModes>,
    supported_resolutions: Option<SupportedResolutions>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ColorModes {
    #[serde(rename = "ColorMode", default)]
    color_mode: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SupportedResolutions {
    discrete_resolutions: Option<DiscreteResolutions>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DiscreteResolutions {
    #[serde(rename = "DiscreteResolution", default)]
    discrete_resolution: Vec<DiscreteResolution>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DiscreteResolution {
    #[serde(rename = "XResolution")]
    x_resolution: u32,
    #[serde(rename = "YResolution")]
    y_resolution: u32,
}
