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

#[cfg(feature = "wia")]
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
        registry.register(Box::new(EsclBackend::new()));

        // Platform-specific backends
        #[cfg(feature = "wia")]
        registry.register(Box::new(WiaBackend::new()));

        #[cfg(feature = "ica")]
        registry.register(Box::new(IcaBackend::new()));

        #[cfg(feature = "sane")]
        registry.register(Box::new(SaneBackend::new()));

        registry
    }

    pub fn register(&mut self, provider: Box<dyn BackendProvider>) {
        self.providers.push(provider);
    }

    pub fn list_devices(&self) -> Result<Vec<ScannerInfo>> {
        let mut scanners_info = Vec::new();
        for provider in &self.providers {
            scanners_info.extend(provider.enumerate());
        }

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
        for provider in &self.providers {
            match provider.start_scan(device_id, config.clone()) {
                Ok(session) => return Ok(session),
                Err(PapyrError::NotFound(_)) => continue,
                Err(e) => return Err(e),
            }
        }

        Err(PapyrError::NotFound(device_id.to_string()))
    }
}
