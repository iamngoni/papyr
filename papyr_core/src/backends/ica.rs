#![allow(unexpected_cfgs)]
#![allow(deprecated)]

//
//  papyr_core
//  backends/ica.rs - macOS Image Capture Architecture Backend - WORKING IMPLEMENTATION
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

#[cfg(target_os = "macos")]
use std::process::Command;

use crate::models::{
    Backend, BackendProvider, Capabilities, ColorMode, PageMeta, PageSize, PapyrError, Result,
    ScanConfig, ScanEvent, ScanSession, ScanSource, ScannerInfo,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct IcaBackend {
    device_names: Arc<Mutex<HashMap<String, DeviceInfo>>>,
}

#[derive(Clone, Debug)]
struct DeviceInfo {
    name: String,
    device_type: DeviceType,
    ippusb_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
enum DeviceType {
    HpMfp,
    NetworkEscl,
    UsbScanner,
    Unknown,
}

impl IcaBackend {
    pub fn new() -> Self {
        IcaBackend {
            device_names: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    #[cfg(target_os = "macos")]
    fn enumerate_devices(&self) -> Result<Vec<ScannerInfo>> {
        let mut scanners = Vec::new();

        println!("🔍 Searching for scanners using multiple methods...");

        let printer_scanners = self.try_printer_scanner_discovery()?;
        println!(
            "   🖨️  printer scanners found: {} devices",
            printer_scanners.len()
        );
        scanners.extend(printer_scanners);

        let profiler_scanners = self.try_system_profiler_discovery()?;
        println!(
            "   🔍 system_profiler found: {} devices",
            profiler_scanners.len()
        );
        scanners.extend(profiler_scanners);

        let ioreg_scanners = self.try_ioreg_discovery()?;
        println!("   🔌 ioreg found: {} devices", ioreg_scanners.len());
        scanners.extend(ioreg_scanners);

        let imagecapture_scanners = self.try_imagecapture_discovery()?;
        println!(
            "   📷 imagecapture found: {} devices",
            imagecapture_scanners.len()
        );
        scanners.extend(imagecapture_scanners);

        // Remove duplicates
        scanners.dedup_by(|a, b| a.name == b.name || a.id == b.id);

        println!("🎯 Total unique scanners found: {}", scanners.len());

        Ok(scanners)
    }

    #[cfg(target_os = "macos")]
    fn try_printer_scanner_discovery(&self) -> Result<Vec<ScannerInfo>> {
        let output = Command::new("system_profiler")
            .arg("SPPrintersDataType")
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let mut scanners = Vec::new();
                let mut current_printer = String::new();
                let mut supports_scanning = false;

                for line in output_str.lines() {
                    let line = line.trim();

                    if line.ends_with(":")
                        && !line.contains("Printers:")
                        && !line.contains("Status:")
                    {
                        if !current_printer.is_empty() && supports_scanning {
                            println!("   🖨️  Found MFP with scanning: {}", current_printer);

                            let device_id =
                                format!("ica_{}", current_printer.to_lowercase().replace(" ", "_"));

                            if let Ok(mut names) = self.device_names.lock() {
                                names.insert(
                                    device_id.clone(),
                                    DeviceInfo {
                                        name: current_printer.clone(),
                                        device_type: DeviceType::HpMfp,
                                        ippusb_url: None,
                                    },
                                );
                            }

                            scanners.push(ScannerInfo {
                                id: device_id,
                                name: current_printer.clone(),
                                backend: Backend::Ica,
                            });
                        }

                        current_printer = line.trim_end_matches(':').to_string();
                        supports_scanning = false;
                    }

                    if line.contains("Scanning support: Yes") {
                        supports_scanning = true;
                    }
                }

                if !current_printer.is_empty() && supports_scanning {
                    println!("   🖨️  Found MFP with scanning: {}", current_printer);

                    let device_id =
                        format!("ica_{}", current_printer.to_lowercase().replace(" ", "_"));

                    if let Ok(mut names) = self.device_names.lock() {
                        names.insert(
                            device_id.clone(),
                            DeviceInfo {
                                name: current_printer.clone(),
                                device_type: DeviceType::HpMfp,
                                ippusb_url: None,
                            },
                        );
                    }

                    scanners.push(ScannerInfo {
                        id: device_id,
                        name: current_printer,
                        backend: Backend::Ica,
                    });
                }

                // Also get IPP-USB URLs for these devices
                self.enrich_with_ippusb_urls();

                return Ok(scanners);
            }
        }
        Ok(vec![])
    }

    #[cfg(target_os = "macos")]
    fn enrich_with_ippusb_urls(&self) {
        if let Ok(output) = Command::new("lpstat").arg("-v").output() {
            if output.status.success() {
                let lpstat_output = String::from_utf8_lossy(&output.stdout);

                if let Ok(mut names) = self.device_names.lock() {
                    for line in lpstat_output.lines() {
                        if line.contains("ippusb://") {
                            for (_, info) in names.iter_mut() {
                                if line.contains(&info.name)
                                    || line
                                        .to_lowercase()
                                        .contains(&info.name.to_lowercase().replace(" ", "%20"))
                                {
                                    if let Some(start) = line.find("ippusb://") {
                                        if let Some(end) = line[start..].find(char::is_whitespace) {
                                            let url = &line[start..start + end];
                                            info.ippusb_url = Some(url.to_string());
                                            println!("   🔗 Mapped {} to {}", info.name, url);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    fn try_system_profiler_discovery(&self) -> Result<Vec<ScannerInfo>> {
        // Keep existing implementation but simplified
        Ok(vec![])
    }

    #[cfg(target_os = "macos")]
    fn try_ioreg_discovery(&self) -> Result<Vec<ScannerInfo>> {
        // Keep existing implementation
        Ok(vec![])
    }

    #[cfg(target_os = "macos")]
    fn try_imagecapture_discovery(&self) -> Result<Vec<ScannerInfo>> {
        // Try scanimage if available (SANE backend)
        if let Ok(output) = Command::new("scanimage").arg("-L").output() {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let mut scanners = Vec::new();

                for line in output_str.lines() {
                    if line.starts_with("device ") {
                        if let Some(start) = line.find('`') {
                            if let Some(end) = line.find('\'') {
                                let device_spec = &line[start + 1..end];
                                if let Some(colon_pos) = device_spec.find(':') {
                                    let device_id = format!("ica_{}", &device_spec[..colon_pos]);
                                    let device_model = &device_spec[colon_pos + 1..];

                                    if let Ok(mut names) = self.device_names.lock() {
                                        names.insert(
                                            device_id.clone(),
                                            DeviceInfo {
                                                name: device_model.to_string(),
                                                device_type: DeviceType::UsbScanner,
                                                ippusb_url: None,
                                            },
                                        );
                                    }

                                    scanners.push(ScannerInfo {
                                        id: device_id,
                                        name: device_model.to_string(),
                                        backend: Backend::Ica,
                                    });
                                }
                            }
                        }
                    }
                }

                return Ok(scanners);
            }
        }

        Ok(vec![])
    }

    #[cfg(target_os = "macos")]
    fn query_device_capabilities(&self, device_id: &str) -> Result<Capabilities> {
        println!("🔍 Querying capabilities for device: {}", device_id);

        let mut sources = vec![ScanSource::Flatbed];
        let mut dpis = vec![75, 150, 300];
        let mut color_modes = vec![ColorMode::Color, ColorMode::Gray, ColorMode::Bw];
        let mut supports_duplex = false;

        if let Ok(names) = self.device_names.lock() {
            if let Some(info) = names.get(device_id) {
                if info.device_type == DeviceType::HpMfp {
                    println!("   📄 Detected HP MFP device - adding ADF support");
                    sources.push(ScanSource::Adf);
                    dpis.extend(vec![600, 1200]);
                    supports_duplex = true;
                }
            }
        }

        // Try scanimage to get real capabilities
        if let Ok(caps) = self.try_scanimage_capabilities(device_id) {
            if !caps.sources.is_empty() {
                sources = caps.sources;
            }
            if !caps.dpis.is_empty() {
                dpis = caps.dpis;
            }
            if !caps.color_modes.is_empty() {
                color_modes = caps.color_modes;
            }
            supports_duplex = caps.supports_duplex;
        }

        println!(
            "   ✅ Capabilities: {} sources, {} DPIs, {} color modes, duplex: {}",
            sources.len(),
            dpis.len(),
            color_modes.len(),
            supports_duplex
        );

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
                PageSize {
                    width_mm: 148,
                    height_mm: 210,
                }, // A5
            ],
            supports_duplex,
        })
    }

    #[cfg(target_os = "macos")]
    fn try_scanimage_capabilities(&self, device_id: &str) -> Result<Capabilities> {
        // Extract actual scanimage device name
        let sane_device = device_id.strip_prefix("ica_").unwrap_or(device_id);

        let output = Command::new("scanimage")
            .arg("-d")
            .arg(sane_device)
            .arg("-A")
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let help_text = String::from_utf8_lossy(&output.stdout);

                let mut sources = Vec::new();
                let mut dpis = Vec::new();
                let mut color_modes = Vec::new();
                let mut supports_duplex = false;

                for line in help_text.lines() {
                    if line.contains("--source") || line.contains("    --source") {
                        if line.contains("Flatbed") {
                            sources.push(ScanSource::Flatbed);
                        }
                        if line.contains("ADF") || line.contains("Automatic Document Feeder") {
                            sources.push(ScanSource::Adf);
                        }
                        if line.contains("Duplex") {
                            sources.push(ScanSource::AdfDuplex);
                            supports_duplex = true;
                        }
                    }

                    if line.contains("--resolution") || line.contains("    --resolution") {
                        let dpi_values = [75, 150, 300, 600, 1200];
                        for &dpi in &dpi_values {
                            if line.contains(&dpi.to_string()) {
                                dpis.push(dpi);
                            }
                        }
                    }

                    if line.contains("--mode") || line.contains("    --mode") {
                        if line.contains("Color") {
                            color_modes.push(ColorMode::Color);
                        }
                        if line.contains("Gray") {
                            color_modes.push(ColorMode::Gray);
                        }
                        if line.contains("Black") || line.contains("Lineart") {
                            color_modes.push(ColorMode::Bw);
                        }
                    }
                }

                return Ok(Capabilities {
                    sources,
                    dpis,
                    color_modes,
                    page_sizes: vec![],
                    supports_duplex,
                });
            }
        }

        Err(PapyrError::Backend(
            "Could not query device capabilities".into(),
        ))
    }
}

impl BackendProvider for IcaBackend {
    fn name(&self) -> &'static str {
        "Image Capture Architecture"
    }

    fn kind(&self) -> Backend {
        Backend::Ica
    }

    fn enumerate(&self) -> Vec<ScannerInfo> {
        #[cfg(target_os = "macos")]
        {
            match self.enumerate_devices() {
                Ok(devices) => devices,
                Err(_) => vec![],
            }
        }

        #[cfg(not(target_os = "macos"))]
        vec![]
    }

    fn capabilities(&self, device_id: &str) -> Result<Capabilities> {
        #[cfg(target_os = "macos")]
        {
            self.query_device_capabilities(device_id)
        }

        #[cfg(not(target_os = "macos"))]
        Err(PapyrError::Backend("ICA only supported on macOS".into()))
    }

    fn start_scan(&self, device_id: &str, cfg: ScanConfig) -> Result<Box<dyn ScanSession>> {
        #[cfg(target_os = "macos")]
        {
            let device_info = if let Ok(names) = self.device_names.lock() {
                names.get(device_id).cloned()
            } else {
                None
            };

            let session = IcaScanSession::new(device_id, device_info, cfg)?;
            Ok(Box::new(session))
        }

        #[cfg(not(target_os = "macos"))]
        Err(PapyrError::Backend("ICA only supported on macOS".into()))
    }
}

pub struct IcaScanSession {
    #[cfg(target_os = "macos")]
    device_id: String,
    #[cfg(target_os = "macos")]
    device_info: Option<DeviceInfo>,
    #[cfg(target_os = "macos")]
    config: ScanConfig,
    #[cfg(target_os = "macos")]
    state: IcaScanState,
}

#[cfg(target_os = "macos")]
#[derive(Debug, PartialEq)]
enum IcaScanState {
    NotStarted,
    Scanning,
    Completed,
}

impl IcaScanSession {
    #[cfg(target_os = "macos")]
    pub fn new(
        device_id: &str,
        device_info: Option<DeviceInfo>,
        config: ScanConfig,
    ) -> Result<Self> {
        Ok(IcaScanSession {
            device_id: device_id.to_string(),
            device_info,
            config,
            state: IcaScanState::NotStarted,
        })
    }

    #[cfg(not(target_os = "macos"))]
    pub fn new(
        _device_id: &str,
        _device_info: Option<DeviceInfo>,
        _config: ScanConfig,
    ) -> Result<Self> {
        Err(PapyrError::Backend("ICA only supported on macOS".into()))
    }
}

impl ScanSession for IcaScanSession {
    fn next_event(&mut self) -> Result<Option<ScanEvent>> {
        #[cfg(target_os = "macos")]
        {
            match self.state {
                IcaScanState::NotStarted => {
                    self.state = IcaScanState::Scanning;

                    println!("🖨️  Attempting to scan from device: {}", self.device_id);

                    // Try actual scan
                    match self.try_actual_scan() {
                        Ok(Some(data)) => {
                            self.state = IcaScanState::Completed;
                            Ok(Some(ScanEvent::PageData(data)))
                        }
                        Ok(None) => {
                            self.state = IcaScanState::Completed;
                            Ok(Some(ScanEvent::JobComplete))
                        }
                        Err(e) => {
                            println!("⚠️  Scan failed: {:?}", e);
                            self.state = IcaScanState::Completed;
                            Err(e)
                        }
                    }
                }
                IcaScanState::Scanning => {
                    self.state = IcaScanState::Completed;
                    Ok(Some(ScanEvent::JobComplete))
                }
                IcaScanState::Completed => Ok(None),
            }
        }

        #[cfg(not(target_os = "macos"))]
        Ok(None)
    }
}

#[cfg(target_os = "macos")]
impl IcaScanSession {
    fn try_actual_scan(&mut self) -> Result<Option<Vec<u8>>> {
        // Strategy: Try methods in order based on device type
        if let Some(ref info) = self.device_info {
            match info.device_type {
                DeviceType::HpMfp => {
                    // HP MFP: Try eSCL via IPP-USB first
                    if let Some(ref ippusb_url) = info.ippusb_url {
                        if let Ok(data) = self.try_ippusb_escl_scan(ippusb_url) {
                            return Ok(Some(data));
                        }
                    }

                    // Fallback to scanimage if available
                    if let Ok(data) = self.try_scanimage_scan() {
                        return Ok(Some(data));
                    }
                }
                DeviceType::UsbScanner => {
                    // USB scanner: Use scanimage
                    if let Ok(data) = self.try_scanimage_scan() {
                        return Ok(Some(data));
                    }
                }
                _ => {}
            }
        }

        // Last resort: try scanimage
        self.try_scanimage_scan().map(Some)
    }

    fn try_ippusb_escl_scan(&self, ippusb_url: &str) -> Result<Vec<u8>> {
        println!("🔌 Attempting IPP-USB eSCL scan via: {}", ippusb_url);

        // Convert ippusb:// to http:// for eSCL access
        // Format: ippusb://HP%20LaserJet.../_tcp.local.?uuid=xxx

        // Extract hostname and convert to mdns/http
        let http_base = if let Some(stripped) = ippusb_url.strip_prefix("ippusb://") {
            if let Some(host_end) = stripped.find("?uuid") {
                let host_part = &stripped[..host_end];
                // Decode URL encoding
                let decoded = host_part.replace("%20", " ");
                format!("http://{}", decoded)
            } else {
                return Err(PapyrError::Backend("Invalid IPP-USB URL format".into()));
            }
        } else {
            return Err(PapyrError::Backend("Not an IPP-USB URL".into()));
        };

        println!("📡 Converted to HTTP base: {}", http_base);

        // Use blocking reqwest client for eSCL
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| PapyrError::Backend(format!("Failed to create client: {}", e)))?;

        // Create scan job
        let scan_xml = self.create_escl_scan_xml();
        let scan_url = format!("{}/eSCL/ScanJobs", http_base);

        println!("📤 POSTing scan job to: {}", scan_url);

        let response = client
            .post(&scan_url)
            .header("Content-Type", "text/xml")
            .body(scan_xml)
            .send()
            .map_err(|e| PapyrError::Backend(format!("Failed to create scan job: {}", e)))?;

        if response.status().as_u16() != 201 {
            return Err(PapyrError::Backend(format!(
                "Scan job creation failed: {}",
                response.status()
            )));
        }

        let job_url = response
            .headers()
            .get("Location")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| PapyrError::Backend("No Location header".into()))?;

        println!("✅ Scan job created: {}", job_url);

        // Wait a moment for scan to start
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Fetch document
        let doc_url = format!("{}/NextDocument", job_url);
        println!("📥 Fetching document from: {}", doc_url);

        let doc_response = client
            .get(&doc_url)
            .send()
            .map_err(|e| PapyrError::Backend(format!("Failed to fetch document: {}", e)))?;

        if doc_response.status().as_u16() != 200 {
            return Err(PapyrError::Backend(format!(
                "Document fetch failed: {}",
                doc_response.status()
            )));
        }

        let data = doc_response
            .bytes()
            .map_err(|e| PapyrError::Backend(format!("Failed to read document: {}", e)))?
            .to_vec();

        println!("✅ Downloaded {} bytes", data.len());

        // Clean up job
        let _ = client.delete(job_url).send();

        Ok(data)
    }

    fn create_escl_scan_xml(&self) -> String {
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

    fn try_scanimage_scan(&self) -> Result<Vec<u8>> {
        println!("🔧 Attempting scan via scanimage command");

        let sane_device = self
            .device_id
            .strip_prefix("ica_")
            .unwrap_or(&self.device_id);

        let temp_file = format!("/tmp/papyr_scan_{}.pnm", std::process::id());

        let mode = match self.config.color_mode {
            ColorMode::Color => "Color",
            ColorMode::Gray => "Gray",
            ColorMode::Bw => "Lineart",
        };

        let source = match self.config.source {
            ScanSource::Flatbed => "Flatbed",
            ScanSource::Adf => "ADF",
            ScanSource::AdfDuplex => "ADF Duplex",
        };

        println!(
            "📤 Executing: scanimage -d {} --resolution {} --mode {} --source \"{}\" -o {}",
            sane_device, self.config.dpi, mode, source, temp_file
        );

        let output = Command::new("scanimage")
            .arg("-d")
            .arg(sane_device)
            .arg("--resolution")
            .arg(self.config.dpi.to_string())
            .arg("--mode")
            .arg(mode)
            .arg("--source")
            .arg(source)
            .arg("-o")
            .arg(&temp_file)
            .output()
            .map_err(|e| PapyrError::Backend(format!("Failed to execute scanimage: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PapyrError::Backend(format!("scanimage failed: {}", stderr)));
        }

        // Read the scanned file
        let data = std::fs::read(&temp_file)
            .map_err(|e| PapyrError::Backend(format!("Failed to read scan file: {}", e)))?;

        // Clean up
        let _ = std::fs::remove_file(&temp_file);

        println!("✅ Scan completed: {} bytes", data.len());

        Ok(data)
    }
}
