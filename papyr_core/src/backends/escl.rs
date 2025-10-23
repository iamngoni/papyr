//
//  papyr_core
//  backends/escl.rs - eSCL (AirPrint/AirScan) backend - WORKING IMPLEMENTATION
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use crate::models::*;
use mdns_sd::{ScopedIp, ServiceDaemon, ServiceEvent};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// Multiple eSCL service types
const ESCL_SERVICES: &[&str] = &[
    "_uscan._tcp.local.",   // Standard eSCL HTTP
    "_uscans._tcp.local.",  // Secure eSCL HTTPS
    "_airscan._tcp.local.", // Apple's AirScan variant
];

// Extended discovery timeout
const DISCOVERY_TIMEOUT_SECS: u64 = 10;

pub struct EsclBackend {
    discovered_scanners: Arc<Mutex<HashMap<String, EsclDevice>>>,
}

#[derive(Clone, Debug)]
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
        // Handle IPv6 addresses properly
        let host = if self.host.contains(':') && !self.host.starts_with('[') {
            format!("[{}]", self.host)
        } else {
            self.host.clone()
        };
        format!(
            "{}://{}/eSCL",
            protocol,
            if self.port == 443 || self.port == 80 {
                host
            } else {
                format!("{}:{}", host, self.port)
            }
        )
    }
}

impl EsclBackend {
    pub fn new() -> Self {
        Self {
            discovered_scanners: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn is_valid_address(&self, addr: &ScopedIp) -> bool {
        // For now, just accept all addresses
        !addr.to_string().is_empty()
    }

    fn discover_scanners(&self) -> Result<Vec<ScannerInfo>> {
        use std::sync::mpsc::channel;
        use std::thread;

        let (tx, rx) = channel();

        // Spawn discovery thread without capturing self
        thread::spawn(move || {
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
                    Err(e) => {
                        println!("Failed to create mDNS daemon: {:?}", e);
                        return Vec::new();
                    }
                };

                println!("🌐 Starting eSCL discovery for multiple service types...");
                let mut all_scanners = Vec::new();

                // Discover each service type
                for service_type in ESCL_SERVICES {
                    println!("🔍 Browsing for service: {}", service_type);

                    match mdns.browse(service_type) {
                        Ok(receiver) => {
                            let timeout = tokio::time::sleep(Duration::from_secs(DISCOVERY_TIMEOUT_SECS));
                            tokio::pin!(timeout);

                            let mut service_scanners = Vec::new();

                            loop {
                                tokio::select! {
                                    event = receiver.recv_async() => {
                                        match event {
                                            Ok(ServiceEvent::ServiceResolved(info)) => {
                                                println!("📡 Found service: {}", info.get_fullname());

                                                let name = info.get_fullname()
                                                    .trim_end_matches('.').to_string();

                                                // Get the best available address
                                                let addresses: Vec<_> = info.get_addresses().iter()
                                                    .cloned()
                                                    .filter(|addr| !addr.to_string().is_empty())
                                                    .collect();

                                                println!("Available addresses: {:?}", addresses);

                                                if let Some(addr) = addresses.first() {
                                                    let host = addr.to_string();
                                                    let port = info.get_port();
                                                    let use_https = service_type.contains("uscans") || service_type.contains("airscan");

                                                    let device = EsclDevice {
                                                        id: format!("escl_{}", name.replace('.', "_")),
                                                        name: name.clone(),
                                                        host,
                                                        port,
                                                        use_https,
                                                    };

                                                    println!("✅ Added device: {} at {}", device.name, device.base_url());
                                                    service_scanners.push(device);
                                                }
                                            },
                                            Ok(_) => continue,
                                            Err(_) => break,
                                        }
                                    }
                                    _ = &mut timeout => {
                                        println!("🕒 Discovery timeout for {}", service_type);
                                        break;
                                    }
                                }
                            }

                            all_scanners.extend(service_scanners);
                        },
                        Err(e) => {
                            println!("Failed to browse {}: {:?}", service_type, e);
                        }
                    }
                }

                println!("🎯 Total eSCL devices discovered: {}", all_scanners.len());
                all_scanners
            });

            let _ = tx.send(result);
        });

        // Wait for discovery to complete
        let devices = rx
            .recv()
            .map_err(|_| PapyrError::Backend("eSCL discovery thread failed".into()))?;

        // Store discovered devices
        if let Ok(mut discovered) = self.discovered_scanners.lock() {
            discovered.clear();
            for device in &devices {
                discovered.insert(device.id.clone(), device.clone());
            }
        }

        // Convert to ScannerInfo
        let scanners: Vec<ScannerInfo> = devices
            .into_iter()
            .map(|device| ScannerInfo {
                id: device.id.clone(),
                name: device.name.clone(),
                backend: Backend::Escl,
            })
            .collect();

        Ok(scanners)
    }

    fn parse_capabilities(&self, xml: &str) -> Result<Capabilities> {
        // Simple XML parsing for now
        let dpis = vec![75, 150, 300, 600];
        let color_modes = vec![ColorMode::Color, ColorMode::Gray, ColorMode::Bw];
        let mut sources = vec![ScanSource::Flatbed];

        // Look for ADF support
        if xml.contains("ADF") || xml.contains("DocumentFeeder") || xml.contains("Feeder") {
            sources.push(ScanSource::Adf);
        }

        // Look for duplex support
        let supports_duplex = xml.contains("Duplex") || xml.contains("TwoSided");

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
}

impl BackendProvider for EsclBackend {
    fn name(&self) -> &'static str {
        "eSCL (AirPrint/AirScan)"
    }

    fn kind(&self) -> Backend {
        Backend::Escl
    }

    fn enumerate(&self) -> Vec<ScannerInfo> {
        self.discover_scanners().unwrap_or_default()
    }

    fn capabilities(&self, device_id: &str) -> Result<Capabilities> {
        let discovered = self
            .discovered_scanners
            .lock()
            .map_err(|_| PapyrError::Backend("Failed to lock discovered scanners".into()))?;

        let device = discovered
            .values()
            .find(|d| d.id == device_id)
            .ok_or_else(|| PapyrError::NotFound(format!("Device {} not found", device_id)))?;

        // Use blocking client to fetch actual capabilities
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| PapyrError::Backend(format!("Failed to create HTTP client: {}", e)))?;

        let url = format!("{}/ScannerCapabilities", device.base_url());
        println!("🔍 Fetching capabilities from: {}", url);

        match client.get(&url).send() {
            Ok(response) if response.status().is_success() => match response.text() {
                Ok(xml) => {
                    println!("📄 Capabilities XML received ({} bytes)", xml.len());
                    self.parse_capabilities(&xml)
                }
                Err(_) => {
                    println!("⚠️  Failed to read capabilities, using defaults");
                    Ok(self.default_capabilities())
                }
            },
            _ => {
                println!("⚠️  Failed to fetch capabilities, using defaults");
                Ok(self.default_capabilities())
            }
        }
    }

    fn start_scan(&self, device_id: &str, config: ScanConfig) -> Result<Box<dyn ScanSession>> {
        let discovered = self
            .discovered_scanners
            .lock()
            .map_err(|_| PapyrError::Backend("Failed to lock discovered scanners".into()))?;

        let device = discovered
            .get(device_id)
            .ok_or_else(|| PapyrError::NotFound(format!("Device {} not found", device_id)))?
            .clone();

        Ok(Box::new(EsclScanSession::new(device, config)?))
    }
}

impl EsclBackend {
    fn default_capabilities(&self) -> Capabilities {
        Capabilities {
            sources: vec![ScanSource::Flatbed, ScanSource::Adf],
            dpis: vec![75, 150, 300, 600],
            color_modes: vec![ColorMode::Color, ColorMode::Gray, ColorMode::Bw],
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
            supports_duplex: false,
        }
    }
}

pub struct EsclScanSession {
    device: EsclDevice,
    config: ScanConfig,
    client: reqwest::blocking::Client,
    job_url: Option<String>,
    page_index: u32,
    state: ScanState,
}

#[derive(Debug, PartialEq)]
enum ScanState {
    NotStarted,
    JobCreated,
    Scanning,
    Completed,
}

impl EsclScanSession {
    pub fn new(device: EsclDevice, config: ScanConfig) -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(120)) // Long timeout for scanning
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| PapyrError::Backend(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            device,
            config,
            client,
            job_url: None,
            page_index: 0,
            state: ScanState::NotStarted,
        })
    }

    fn create_scan_settings_xml(&self) -> String {
        let input_source = match self.config.source {
            ScanSource::Flatbed => "Platen",
            ScanSource::Adf | ScanSource::AdfDuplex => "Feeder",
        };

        let color_mode = match self.config.color_mode {
            ColorMode::Color => "RGB24",
            ColorMode::Gray => "Grayscale8",
            ColorMode::Bw => "BlackAndWhite1",
        };

        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<scan:ScanSettings xmlns:scan="http://schemas.hp.com/imaging/escl/2011/05/03" xmlns:pwg="http://www.pwg.org/schemas/2010/12/sm">
    <pwg:Version>2.1</pwg:Version>
    <scan:Intent>Document</scan:Intent>
    <scan:InputSource>{}</scan:InputSource>
    <scan:ColorMode>{}</scan:ColorMode>
    <scan:XResolution>{}</scan:XResolution>
    <scan:YResolution>{}</scan:YResolution>
    <scan:DocumentFormat>image/jpeg</scan:DocumentFormat>
</scan:ScanSettings>"#,
            input_source, color_mode, self.config.dpi, self.config.dpi
        )
    }

    fn create_job(&mut self) -> Result<()> {
        let url = format!("{}/ScanJobs", self.device.base_url());
        let scan_xml = self.create_scan_settings_xml();

        println!("🖨️  Creating scan job at: {}", url);
        println!("📄 Settings:\n{}", scan_xml);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "text/xml")
            .body(scan_xml)
            .send()
            .map_err(|e| PapyrError::Backend(format!("Failed to create scan job: {}", e)))?;

        let status = response.status();
        println!("📥 Response status: {}", status);

        if status.as_u16() == 201 {
            if let Some(location) = response.headers().get("Location") {
                let job_url = location
                    .to_str()
                    .map_err(|_| PapyrError::Backend("Invalid job location header".into()))?
                    .to_string();

                println!("✅ Scan job created: {}", job_url);
                self.job_url = Some(job_url);
                self.state = ScanState::JobCreated;
                Ok(())
            } else {
                Err(PapyrError::Backend("No Location header in response".into()))
            }
        } else {
            let body = response.text().unwrap_or_default();
            Err(PapyrError::Backend(format!(
                "Scan job creation failed: HTTP {} - {}",
                status, body
            )))
        }
    }

    fn fetch_next_document(&mut self) -> Result<Option<Vec<u8>>> {
        let job_url = self
            .job_url
            .as_ref()
            .ok_or_else(|| PapyrError::Backend("No active scan job".into()))?;

        let document_url = format!("{}/NextDocument", job_url);
        println!("📥 Fetching document from: {}", document_url);

        let response = self
            .client
            .get(&document_url)
            .header("Accept", "image/jpeg,image/png,application/pdf")
            .send()
            .map_err(|e| PapyrError::Backend(format!("Failed to fetch document: {}", e)))?;

        let status = response.status();
        println!("📥 Document response status: {}", status);

        match status.as_u16() {
            200 => {
                let bytes = response
                    .bytes()
                    .map_err(|e| {
                        PapyrError::Backend(format!("Failed to read document data: {}", e))
                    })?
                    .to_vec();

                println!("✅ Downloaded document: {} bytes", bytes.len());
                Ok(Some(bytes))
            }
            404 => {
                println!("✅ No more documents (HTTP 404)");
                Ok(None)
            }
            _ => {
                let body = response.text().unwrap_or_default();
                Err(PapyrError::Backend(format!(
                    "Document fetch failed: HTTP {} - {}",
                    status, body
                )))
            }
        }
    }

    fn delete_job(&mut self) -> Result<()> {
        if let Some(job_url) = &self.job_url {
            println!("🗑️  Deleting scan job: {}", job_url);

            match self.client.delete(job_url).send() {
                Ok(response) => {
                    println!("✅ Job deleted: HTTP {}", response.status());
                }
                Err(e) => {
                    println!("⚠️  Failed to delete job: {}", e);
                }
            }
        }
        Ok(())
    }
}

impl ScanSession for EsclScanSession {
    fn next_event(&mut self) -> Result<Option<ScanEvent>> {
        match self.state {
            ScanState::NotStarted => {
                // Create the scan job
                self.create_job()?;
                self.state = ScanState::Scanning;

                // Return PageStarted for first page
                Ok(Some(ScanEvent::PageStarted(self.page_index)))
            }

            ScanState::JobCreated | ScanState::Scanning => {
                // Fetch next document
                match self.fetch_next_document()? {
                    Some(data) => {
                        let page_meta = PageMeta {
                            index: self.page_index,
                            width_px: (8.5 * self.config.dpi as f32) as u32,
                            height_px: (11.0 * self.config.dpi as f32) as u32,
                            dpi: self.config.dpi,
                            color_mode: self.config.color_mode,
                        };

                        self.page_index += 1;

                        // Return page data, then complete, then start next page on next call
                        if data.is_empty() {
                            Ok(Some(ScanEvent::PageComplete(page_meta)))
                        } else {
                            Ok(Some(ScanEvent::PageData(data)))
                        }
                    }
                    None => {
                        // No more documents
                        self.delete_job()?;
                        self.state = ScanState::Completed;
                        Ok(Some(ScanEvent::JobComplete))
                    }
                }
            }

            ScanState::Completed => Ok(None),
        }
    }
}

impl Drop for EsclScanSession {
    fn drop(&mut self) {
        if self.state != ScanState::Completed {
            let _ = self.delete_job();
        }
    }
}
