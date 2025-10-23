//
//  papyr_core
//  backends/wia.rs - Windows Image Acquisition Backend (FIXED IMPLEMENTATION)
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

#[cfg(windows)]
use windows::{
    core::*,
    Win32::Devices::ImageAcquisition::*,
    Win32::System::Com::*,
    Win32::System::Variant::*,
    Win32::UI::Shell::PropertiesSystem::*,
};

use crate::models::{
    Backend, BackendProvider, Capabilities, PapyrError, Result, ScanConfig, ScanEvent, ScanSession,
    ScannerInfo, ColorMode, PageSize, ScanSource,
};

// Correct WIA 2.0 constants
#[cfg(windows)]
const WIA_DEVICETYPE_SCANNER: i32 = 0x00000001;

// WIA Property IDs
#[cfg(windows)]
const WIA_DIP_DEV_ID: u32 = 2;
#[cfg(windows)]
const WIA_DIP_DEV_NAME: u32 = 3;
#[cfg(windows)]
const WIA_DIP_DEV_TYPE: u32 = 4;

pub struct WiaBackend {
    // Don't store COM objects directly to avoid Send/Sync issues
}

impl WiaBackend {
    pub fn new() -> Self {
        WiaBackend {}
    }

    #[cfg(windows)]
    fn get_device_manager(&self) -> Result<IWiaDevMgr2> {
        unsafe {
            // Proper COM initialization for STA
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .map_err(|_| PapyrError::Backend("Failed to initialize COM".into()))?;

            // Use WIA 2.0 device manager
            let device_manager: IWiaDevMgr2 = CoCreateInstance(
                &CLSID_WiaDevMgr2,
                None,
                CLSCTX_LOCAL_SERVER,
            )?;
            
            Ok(device_manager)
        }
    }

    #[cfg(windows)]
    fn enumerate_devices(&self) -> Result<Vec<ScannerInfo>> {
        let device_manager = self.get_device_manager()?;
        let mut devices = Vec::new();

        unsafe {
            match device_manager.EnumDeviceInfo(WIA_DEVICETYPE_SCANNER) {
                Ok(device_enum) => {
                    // Reset enumeration cursor
                    let _ = device_enum.Reset();
                    
                    loop {
                        let mut device_info: Option<IWiaPropertyStorage> = None;
                        let mut fetched = 0u32;

                        if device_enum
                            .Next(1, &mut device_info as *mut _, &mut fetched as *mut _)
                            .is_ok()
                            && fetched > 0
                        {
                            if let Some(info) = device_info {
                                match self.extract_device_info(&info) {
                                    Ok(scanner_info) => {
                                        println!("Found WIA scanner: {}", scanner_info.name);
                                        devices.push(scanner_info);
                                    },
                                    Err(e) => {
                                        println!("Failed to extract device info: {:?}", e);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    }
                },
                Err(e) => {
                    println!("WIA device enumeration failed: {:?}", e);
                    return Ok(vec![]);
                }
            }
        }

        Ok(devices)
    }

    #[cfg(windows)]
    fn extract_device_info(&self, device_info: &IWiaPropertyStorage) -> Result<ScannerInfo> {
        let device_id = self
            .read_device_property(device_info, WIA_DIP_DEV_ID)
            .unwrap_or_else(|_| {
                format!(
                    "wia_device_{}",
                    std::ptr::addr_of!(device_info) as usize
                )
            });

        let device_name = self
            .read_device_property(device_info, WIA_DIP_DEV_NAME)
            .unwrap_or_else(|_| "Unknown WIA Scanner".to_string());

        Ok(ScannerInfo {
            id: device_id,
            name: device_name,
            backend: Backend::Wia,
        })
    }

    #[cfg(windows)]
    fn read_device_property(&self, storage: &IWiaPropertyStorage, prop_id: u32) -> Result<String> {
        unsafe {
            // Create PROPSPEC for the property we want to read
            let mut prop_spec = PROPSPEC {
                ulKind: PRSPEC_PROPID,
                Anonymous: PROPSPEC_0 {
                    propid: prop_id,
                },
            };

            // Initialize PROPVARIANT to receive the value
            let mut prop_variant: PROPVARIANT = std::mem::zeroed();

            // Read the property
            match storage.ReadMultiple(1, &mut prop_spec, &mut prop_variant) {
                Ok(_) => {
                    let result = match prop_variant.vt {
                        VT_BSTR => {
                            // Handle BSTR (OLE string)
                            if let Some(bstr) = prop_variant.Anonymous.Anonymous.Anonymous.bstrVal.as_ref() {
                                bstr.to_string()
                            } else {
                                "".to_string()
                            }
                        },
                        VT_LPWSTR => {
                            // Handle wide string pointer
                            if !prop_variant.Anonymous.Anonymous.Anonymous.pwszVal.is_null() {
                                let wide_str = PWSTR::from_raw(prop_variant.Anonymous.Anonymous.Anonymous.pwszVal);
                                wide_str.to_string().unwrap_or_default()
                            } else {
                                "".to_string()
                            }
                        },
                        _ => {
                            format!("Unknown property type: {}", prop_variant.vt.0)
                        }
                    };

                    // Clean up the variant
                    VariantClear(&mut prop_variant as *mut _ as *mut _).ok();
                    
                    Ok(result)
                },
                Err(e) => {
                    Err(PapyrError::Backend(format!("Failed to read WIA property {}: {:?}", prop_id, e)))
                }
            }
        }
    }

    #[cfg(windows)]
    fn create_device_connection(&self, device_id: &str) -> Result<IWiaItem2> {
        let device_manager = self.get_device_manager()?;
        
        unsafe {
            let device_id_bstr = BSTR::from(device_id);
            let device: IWiaItem2 = device_manager.CreateDevice(0, &device_id_bstr)?;
            Ok(device)
        }
    }
}

impl BackendProvider for WiaBackend {
    fn name(&self) -> &'static str {
        "Windows Image Acquisition (WIA)"
    }

    fn kind(&self) -> Backend {
        Backend::Wia
    }

    fn list_devices(&self) -> Result<Vec<ScannerInfo>> {
        #[cfg(windows)]
        {
            self.enumerate_devices()
        }

        #[cfg(not(windows))]
        Ok(vec![])
    }

    fn capabilities(&self, _device_id: &str) -> Result<Capabilities> {
        #[cfg(windows)]
        {
            // For now, return typical scanner capabilities
            // TODO: Query actual device capabilities using IWiaItem2
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

    fn start_scan(&self, device_id: &str, config: ScanConfig) -> Result<Box<dyn ScanSession>> {
        #[cfg(windows)]
        {
            let session = WiaScanSession::new(device_id, config)?;
            Ok(Box::new(session))
        }

        #[cfg(not(windows))]
        {
            let _ = (device_id, config);
            Err(PapyrError::Backend("WIA only supported on Windows".into()))
        }
    }
}

pub struct WiaScanSession {
    device_id: String,
    config: ScanConfig,
    completed: bool,
    #[cfg(windows)]
    device: Option<IWiaItem2>,
}

impl WiaScanSession {
    #[cfg(windows)]
    pub fn new(device_id: &str, config: ScanConfig) -> Result<Self> {
        Ok(WiaScanSession {
            device_id: device_id.to_string(),
            config,
            completed: false,
            device: None,
        })
    }

    #[cfg(not(windows))]
    pub fn new(device_id: &str, config: ScanConfig) -> Result<Self> {
        let _ = (device_id, config);
        Err(PapyrError::Backend("WIA only supported on Windows".into()))
    }

    #[cfg(windows)]
    fn perform_actual_scan(&mut self) -> Result<Vec<u8>> {
        // TODO: Implement actual WIA scanning using IWiaTransfer
        // This is a placeholder that returns mock data
        println!("WIA: Starting actual scan of device: {}", self.device_id);
        
        // Mock scan data for now
        let mock_image_data = vec![0xFF; 10240]; // 10KB of mock image data
        Ok(mock_image_data)
    }
}

impl ScanSession for WiaScanSession {
    fn next_event(&mut self) -> Result<Option<ScanEvent>> {
        #[cfg(windows)]
        {
            if self.completed {
                return Ok(None);
            }

            println!("WIA: Processing scan for device {}", self.device_id);
            
            // For now, simulate a complete scan
            // TODO: Implement proper WIA scanning with real events
            match self.perform_actual_scan() {
                Ok(image_data) => {
                    self.completed = true;
                    
                    if !image_data.is_empty() {
                        Ok(Some(ScanEvent::PageData(image_data)))
                    } else {
                        Ok(Some(ScanEvent::JobComplete))
                    }
                },
                Err(e) => {
                    self.completed = true;
                    Err(e)
                }
            }
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
