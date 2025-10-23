//
//  papyr_core
//  backends/escl.rs - eSCL (AirPrint/AirScan) backend (FIXED IMPLEMENTATION)
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use crate::models::*;
use mdns_sd::{ServiceDaemon, ServiceEvent, ScopedIp};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// Multiple eSCL service types
const ESCL_SERVICES: &[&str] = &[
    "_uscan._tcp.local.",    // Standard eSCL HTTP
    "_uscans._tcp.local.",   // Secure eSCL HTTPS
    "_airscan._tcp.local.",  // Apple's AirScan variant
];

// Extended discovery timeout
const DISCOVERY_TIMEOUT_SECS: u64 = 10;

pub struct EsclBackend {
    discovered_scanners: Arc<Mutex<HashMap<String, EsclDevice>>>,
    client: Client,
    mdns_daemon: Arc<Mutex<Option<ServiceDaemon>>>,
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
        format!("{}://{}/eSCL", protocol, if self.port == 443 || self.port == 80 { 
            host 
        } else { 
            format!("{}:{}", host, self.port) 
        })
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
            mdns_daemon: Arc::new(Mutex::new(None)),
        }
    }

    fn is_valid_address(&self, addr: &ScopedIp) -> bool {
        // For now, just accept all addresses 
        // TODO: Implement proper address filtering
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
    let devices = rx.recv()
            .map_err(|_| PapyrError::Backend("eSCL discovery thread failed".into()))?;

    // Store discovered devices
        if let Ok(mut discovered) = self.discovered_scanners.lock() {
        discovered.clear();
        for device in &devices {
        discovered.insert(device.name.clone(), device.clone());
            }
    }

    // Convert to ScannerInfo
    let scanners: Vec<ScannerInfo> = devices.into_iter().map(|device| {
    ScannerInfo {
    id: device.id.clone(),
        name: device.name.clone(),
            backend: Backend::Escl,
            }
    }).collect();

        Ok(scanners)
    }

    async fn get_scanner_capabilities(&self, device: &EsclDevice) -> Result<Capabilities> {
        let url = format!("{}/ScannerCapabilities", device.base_url());
        println!("🔍 Fetching capabilities from: {}", url);
        
        let response = self.client.get(&url).send().await
            .map_err(|e| PapyrError::Backend(format!("Failed to get capabilities: {}", e)))?;

        if !response.status().is_success() {
            return Err(PapyrError::Backend(format!("HTTP error: {}", response.status())));
        }

        let xml = response.text().await
            .map_err(|e| PapyrError::Backend(format!("Failed to read response: {}", e)))?;

        println!("📄 Capabilities XML received ({} bytes)", xml.len());
        
        // Parse XML to extract capabilities
        self.parse_capabilities(&xml)
    }

    fn parse_capabilities(&self, xml: &str) -> Result<Capabilities> {
        // Simple XML parsing for now - in production use proper XML parser
        let dpis = vec![75, 150, 300, 600]; // Default DPIs
        let color_modes = vec![ColorMode::Color, ColorMode::Gray, ColorMode::Bw];
        let mut sources = vec![ScanSource::Flatbed];

        // Look for ADF support
        if xml.contains("ADF") || xml.contains("DocumentFeeder") {
            sources.push(ScanSource::Adf);
        }

        // Look for duplex support
        let supports_duplex = xml.contains("Duplex") || xml.contains("TwoSided");

        Ok(Capabilities {
            sources,
            dpis,
            color_modes,
            page_sizes: vec![
                PageSize { width_mm: 216, height_mm: 279 }, // Letter
                PageSize { width_mm: 210, height_mm: 297 }, // A4
            ],
            supports_duplex,
        })
    }

    async fn start_scan_job(&self, device: &EsclDevice, config: &ScanConfig) -> Result<String> {
        let url = format!("{}/ScanJobs", device.base_url());
        
        // Create scan settings XML
        let scan_xml = self.create_scan_settings_xml(config);
        
        println!("🖨️ Starting scan job at: {}", url);
        println!("📄 Scan settings: {}", scan_xml);
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "text/xml")
            .body(scan_xml)
            .send()
            .await
            .map_err(|e| PapyrError::Backend(format!("Failed to start scan job: {}", e)))?;

        if response.status().as_u16() == 201 {
            // Job created, get location header
            if let Some(location) = response.headers().get("Location") {
                let job_url = location.to_str()
                    .map_err(|_| PapyrError::Backend("Invalid job location header".into()))?;
                Ok(job_url.to_string())
            } else {
                Err(PapyrError::Backend("No job location returned".into()))
            }
        } else {
            Err(PapyrError::Backend(format!("Scan job failed: HTTP {}", response.status())))
        }
    }

    fn create_scan_settings_xml(&self, config: &ScanConfig) -> String {
        let input_source = match config.source {
            ScanSource::Flatbed => "Platen",
            ScanSource::Adf | ScanSource::AdfDuplex => "Feeder",
        };

        let color_mode = match config.color_mode {
            ColorMode::Color => "RGB24",
            ColorMode::Gray => "Grayscale8", 
            ColorMode::Bw => "BlackAndWhite1",
        };

        format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<scan:ScanSettings xmlns:scan="http://schemas.hp.com/imaging/escl/2011/05/03">
    <pwg:Version xmlns:pwg="http://www.pwg.org/schemas/2010/12/sm">2.0</pwg:Version>
    <scan:Intent>Document</scan:Intent>
    <scan:InputSource>{}</scan:InputSource>
    <scan:ColorMode>{}</scan:ColorMode>
    <scan:XResolution>{}</scan:XResolution>
    <scan:YResolution>{}</scan:YResolution>
    <scan:DocumentFormat>image/jpeg</scan:DocumentFormat>
</scan:ScanSettings>"#, input_source, color_mode, config.dpi, config.dpi)
    }

    async fn get_next_document(&self, job_url: &str) -> Result<Option<Vec<u8>>> {
        let document_url = format!("{}/NextDocument", job_url);
        println!("📥 Fetching document from: {}", document_url);
        
        let response = self.client.get(&document_url).send().await
            .map_err(|e| PapyrError::Backend(format!("Failed to get document: {}", e)))?;

        match response.status().as_u16() {
            200 => {
                let bytes = response.bytes().await
                    .map_err(|e| PapyrError::Backend(format!("Failed to read document data: {}", e)))?;
                println!("📄 Downloaded document: {} bytes", bytes.len());
                Ok(Some(bytes.to_vec()))
            },
            404 => {
                println!("✅ No more documents (HTTP 404)");
                Ok(None) // No more documents
            },
            _ => {
                Err(PapyrError::Backend(format!("Document fetch failed: HTTP {}", response.status())))
            }
        }
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
        // For async operations in sync trait, we need to block
        let rt = tokio::runtime::Runtime::new()
            .map_err(|_| PapyrError::Backend("Failed to create tokio runtime".into()))?;

        let discovered = self.discovered_scanners.lock()
            .map_err(|_| PapyrError::Backend("Failed to lock discovered scanners".into()))?;

        let device = discovered.values()
            .find(|d| d.id == device_id)
            .ok_or_else(|| PapyrError::NotFound(format!("Device {} not found", device_id)))?
            .clone();

        rt.block_on(self.get_scanner_capabilities(&device))
    }

    fn start_scan(&self, device_id: &str, config: ScanConfig) -> Result<Box<dyn ScanSession>> {
        let discovered = self.discovered_scanners.lock()
            .map_err(|_| PapyrError::Backend("Failed to lock discovered scanners".into()))?;

        let device = discovered.values()
            .find(|d| d.id == device_id)
            .ok_or_else(|| PapyrError::NotFound(format!("Device {} not found", device_id)))?
            .clone();

        Ok(Box::new(EsclScanSession::new(device, config, self.client.clone())))
    }
}

pub struct EsclScanSession {
    device: EsclDevice,
    config: ScanConfig,
    client: Client,
    job_url: Option<String>,
    completed: bool,
    page_count: usize,
}

impl EsclScanSession {
    pub fn new(device: EsclDevice, config: ScanConfig, client: Client) -> Self {
        Self {
            device,
            config,
            client,
            job_url: None,
            completed: false,
            page_count: 0,
        }
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

        format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<scan:ScanSettings xmlns:scan="http://schemas.hp.com/imaging/escl/2011/05/03">
    <pwg:Version xmlns:pwg="http://www.pwg.org/schemas/2010/12/sm">2.0</pwg:Version>
    <scan:Intent>Document</scan:Intent>
    <scan:InputSource>{}</scan:InputSource>
    <scan:ColorMode>{}</scan:ColorMode>
    <scan:XResolution>{}</scan:XResolution>
    <scan:YResolution>{}</scan:YResolution>
    <scan:DocumentFormat>image/jpeg</scan:DocumentFormat>
</scan:ScanSettings>"#, input_source, color_mode, self.config.dpi, self.config.dpi)
    }
}

impl ScanSession for EsclScanSession {
    fn next_event(&mut self) -> Result<Option<ScanEvent>> {
        if self.completed {
            return Ok(None);
        }

        let rt = tokio::runtime::Runtime::new()
            .map_err(|_| PapyrError::Backend("Failed to create tokio runtime".into()))?;

        rt.block_on(async {
            // Start scan job if not started
            if self.job_url.is_none() {
                let url = format!("{}/ScanJobs", self.device.base_url());
                let scan_xml = self.create_scan_settings_xml();
                
                println!("🖨️ Starting eSCL scan job at: {}", url);
                
                let response = self.client
                    .post(&url)
                    .header("Content-Type", "text/xml")
                    .body(scan_xml)
                    .send()
                    .await
                    .map_err(|e| PapyrError::Backend(format!("Failed to start scan job: {}", e)))?;

                if response.status().as_u16() == 201 {
                    if let Some(location) = response.headers().get("Location") {
                        let job_url = location.to_str()
                            .map_err(|_| PapyrError::Backend("Invalid job location header".into()))?;
                        self.job_url = Some(job_url.to_string());
                        return Ok(Some(ScanEvent::PageStarted(0)));
                    } else {
                        return Err(PapyrError::Backend("No job location returned".into()));
                    }
                } else {
                    return Err(PapyrError::Backend(format!("Scan job failed: HTTP {}", response.status())));
                }
            }

            // Get next document
            if let Some(job_url) = &self.job_url {
                let document_url = format!("{}/NextDocument", job_url);
                
                let response = self.client.get(&document_url).send().await
                    .map_err(|e| PapyrError::Backend(format!("Failed to get document: {}", e)))?;

                match response.status().as_u16() {
                    200 => {
                        let bytes = response.bytes().await
                            .map_err(|e| PapyrError::Backend(format!("Failed to read document data: {}", e)))?;
                        
                        println!("📄 Downloaded page {}: {} bytes", self.page_count, bytes.len());
                        self.page_count += 1;
                        
                        // For single page scans, mark as complete
                        if self.config.source == ScanSource::Flatbed || self.config.max_pages == Some(1) {
                            self.completed = true;
                        }
                        
                        Ok(Some(ScanEvent::PageData(bytes.to_vec())))
                    },
                    404 => {
                        println!("✅ Scan complete - no more pages");
                        self.completed = true;
                        Ok(Some(ScanEvent::JobComplete))
                    },
                    _ => {
                        Err(PapyrError::Backend(format!("Document fetch failed: HTTP {}", response.status())))
                    }
                }
            } else {
                Err(PapyrError::Backend("No scan job started".into()))
            }
        })
    }
}
