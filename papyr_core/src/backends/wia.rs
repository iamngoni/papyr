//
//  papyr_core
//  backends/wia.rs - Windows Image Acquisition Backend
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

#[cfg(windows)]
use windows::{
    core::*, 
    Win32::Devices::ImageAcquisition::*, 
    Win32::System::Com::*,
};

use crate::models::{
    Backend, BackendProvider, Capabilities, PapyrError, Result, ScanConfig, ScanEvent, ScanSession,
    ScannerInfo,
};

#[cfg(windows)]
const WIA_DEVICE_TYPE_SCANNER: u32 = 2;

#[cfg(windows)]
const PRSPEC_PROPID: u32 = 1;

pub struct WiaBackend {
    // Don't store COM objects directly to avoid Send/Sync issues
}

impl WiaBackend {
    pub fn new() -> Self {
        #[cfg(windows)]
        {
            // Initialize COM for this thread
            unsafe {
                let _ = CoInitialize(None);
            }
        }

        WiaBackend {}
    }

    #[cfg(windows)]
    fn get_device_manager(&self) -> Result<IWiaDevMgr> {
        unsafe {
            let device_manager: IWiaDevMgr =
                CoCreateInstance(&WiaDevMgr, None, CLSCTX_LOCAL_SERVER)?;
            Ok(device_manager)
        }
    }

    #[cfg(windows)]
    fn enumerate_devices(&self) -> Result<Vec<ScannerInfo>> {
        let device_manager = self.get_device_manager()?;
        let mut devices = Vec::new();

        unsafe {
            match device_manager.EnumDeviceInfo(WIA_DEVICE_TYPE_SCANNER) {
                Ok(device_enum) => loop {
                    let mut device_info: Option<IWiaPropertyStorage> = None;
                    let mut fetched = 0u32;

                    if device_enum
                        .Next(1, &mut device_info as *mut _, &mut fetched as *mut _)
                        .is_ok()
                        && fetched > 0
                    {
                        if let Some(info) = device_info {
                            match self.extract_device_info(&info) {
                                Ok(scanner_info) => devices.push(scanner_info),
                                Err(_) => continue,
                            }
                        }
                    } else {
                        break;
                    }
                },
                Err(_) => return Ok(vec![]),
            }
        }

        Ok(devices)
    }

    #[cfg(windows)]
    fn extract_device_info(&self, device_info: &IWiaPropertyStorage) -> Result<ScannerInfo> {
        unsafe {
            let device_id = self
                .read_device_property(device_info, 0) // WIA_DIP_DEV_ID
                .unwrap_or_else(|_| {
                    format!(
                        "wia_device_{}",
                        std::ptr::addr_of!(device_info) as usize
                    )
                });

            let device_name = self
                .read_device_property(device_info, 1) // WIA_DIP_DEV_NAME
                .unwrap_or_else(|_| "Unknown WIA Scanner".to_string());

            Ok(ScannerInfo {
                id: device_id,
                name: device_name,
                backend: Backend::Wia,
            })
        }
    }

    #[cfg(windows)]
    fn read_device_property(
        &self,
        storage: &IWiaPropertyStorage,
        _prop_id: u32, // Simplified to just use the property ID directly
    ) -> Result<String> {
        // For now, return a generic result to get the compilation working
        // TODO: Implement proper property reading using IWiaPropertyStorage::ReadMultiple
        Ok("WIA Scanner Device".to_string())
    }
}

impl BackendProvider for WiaBackend {
    fn name(&self) -> &'static str {
        "Windows Image Acquisition"
    }

    fn kind(&self) -> Backend {
        Backend::Wia
    }

    fn enumerate(&self) -> Vec<ScannerInfo> {
        #[cfg(windows)]
        {
            match self.enumerate_devices() {
                Ok(devices) => devices,
                Err(_) => vec![],
            }
        }

        #[cfg(not(windows))]
        vec![]
    }

    fn capabilities(&self, _device_id: &str) -> Result<Capabilities> {
        #[cfg(windows)]
        {
            use crate::models::{ColorMode, PageSize, ScanSource};

            // Return typical scanner capabilities - in real implementation, query actual device
            Ok(Capabilities {
                sources: vec![ScanSource::Flatbed, ScanSource::Adf],
                dpis: vec![75, 150, 300, 600, 1200],
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
                supports_duplex: true,
            })
        }

        #[cfg(not(windows))]
        Err(PapyrError::Backend("WIA only supported on Windows".into()))
    }

    fn start_scan(&self, _device_id: &str, _cfg: ScanConfig) -> Result<Box<dyn ScanSession>> {
        #[cfg(windows)]
        {
            let session = WiaScanSession::new(device_id, cfg)?;
            Ok(Box::new(session))
        }

        #[cfg(not(windows))]
        Err(PapyrError::Backend("WIA only supported on Windows".into()))
    }
}

pub struct WiaScanSession {
    #[cfg(windows)]
    device_id: String,
    #[cfg(windows)]
    config: ScanConfig,
    #[cfg(windows)]
    completed: bool,
}

impl WiaScanSession {
    #[cfg(windows)]
    pub fn new(_device_id: &str, config: ScanConfig) -> Result<Self> {
        Ok(WiaScanSession {
            device_id: device_id.to_string(),
            config,
            completed: false,
        })
    }

    #[cfg(not(windows))]
    pub fn new(__device_id: &str, _config: ScanConfig) -> Result<Self> {
        Err(PapyrError::Backend("WIA only supported on Windows".into()))
    }
}

impl ScanSession for WiaScanSession {
    fn next_event(&mut self) -> Result<Option<ScanEvent>> {
        #[cfg(windows)]
        {
            if self.completed {
                return Ok(None);
            }

            // Simulate a simple scan - in real implementation, this would:
            // 1. Create WIA device connection
            // 2. Configure scan parameters
            // 3. Start scanning
            // 4. Return scan events as they occur

            use crate::models::PageMeta;

            self.completed = true;

            // Mock scan result
            let _page_data = vec![0xFF; 1024]; // Mock image data
            let _page_meta = PageMeta {
                index: 0,
                width_px: 2550,  // 8.5" * 300 DPI
                height_px: 3300, // 11" * 300 DPI
                dpi: self.config.dpi,
                color_mode: self.config.color_mode,
            };

            // In a real implementation, you'd return multiple events:
            // - PageStarted
            // - PageData (potentially multiple chunks)
            // - PageComplete
            // - JobComplete

            Ok(Some(ScanEvent::JobComplete))
        }

        #[cfg(not(windows))]
        Ok(None)
    }
}

impl Drop for WiaBackend {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            CoUninitialize();
        }
    }
}
