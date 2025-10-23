//
//  papyr_core
//  registry.rs
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use crate::backends::escl::EsclBackend;
use crate::models::{
    BackendProvider, Capabilities, PapyrError, Result, ScanConfig, ScanSession, ScannerInfo,
};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::backends::twain::TwainBackend;

// WIA is default on Windows
#[cfg(target_os = "windows")]
use crate::backends::wia::WiaBackend;

#[cfg(feature = "ica")]
use crate::backends::ica::IcaBackend;

#[cfg(feature = "sane")]
use crate::backends::sane::SaneBackend;

pub struct BackendRegistry {
    providers: Vec<Box<dyn BackendProvider>>,
}

impl BackendRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            providers: Vec::new(),
        };

        // eSCL is always available (cross-platform network scanning)
        println!("🔧 Registering eSCL backend (network scanners)");
        registry.register(Box::new(EsclBackend::new()));

        // WIA is primary backend on Windows 
        #[cfg(target_os = "windows")]
        {
            println!("🔧 Registering WIA backend (Windows primary)");
            registry.register(Box::new(WiaBackend::new()));
        }

        // TWAIN as fallback for Windows and macOS
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("🔧 Registering TWAIN backend (fallback)");
            registry.register(Box::new(TwainBackend::new()));
        }

        // Other platform-specific backends
        #[cfg(feature = "ica")]
        {
            println!("🔧 Registering ICA backend (macOS)");
            registry.register(Box::new(IcaBackend::new()));
        }

        #[cfg(feature = "sane")]
        {
            println!("🔧 Registering SANE backend (Linux)");
            registry.register(Box::new(SaneBackend::new()));
        }

        registry
    }

    pub fn register(&mut self, provider: Box<dyn BackendProvider>) {
        self.providers.push(provider);
    }

    pub fn list_devices(&self) -> Result<Vec<ScannerInfo>> {
        println!("📡 Querying all registered backends for devices...");
        let mut scanners_info = Vec::new();
        
        for (i, provider) in self.providers.iter().enumerate() {
            println!("🔍 Backend {}: {} - discovering devices...", i + 1, provider.name());
            let devices = provider.enumerate();
            println!("   Found {} devices", devices.len());
            scanners_info.extend(devices);
        }

        println!("🎯 Total devices found across all backends: {}", scanners_info.len());
        Ok(scanners_info)
    }

    pub fn capabilities(&self, device_id: &str) -> Result<Capabilities> {
        for provider in &self.providers {
            if let Ok(capabilities) = provider.capabilities(device_id) {
                return Ok(capabilities);
            }
        }

        Err(PapyrError::NotFound(device_id.to_string()))
    }

    pub fn start_scan(&self, device_id: &str, config: ScanConfig) -> Result<Box<dyn ScanSession>> {
        println!("🚀 Starting scan for device: {}", device_id);
        
        // First, find which backend owns this device
        for provider in &self.providers {
            let devices = provider.enumerate();
            if devices.iter().any(|d| d.id == device_id) {
                println!("📍 Found device in backend: {}", provider.name());
                return provider.start_scan(device_id, config);
            }
        }

        Err(PapyrError::NotFound(format!("Device {} not found in any backend", device_id)))
    }
}
