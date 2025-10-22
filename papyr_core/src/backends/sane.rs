//
//  papyr_core
//  backends/sane.rs - SANE (Scanner Access Now Easy) backend for Linux
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use crate::models::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

// SANE FFI bindings
const SANE_STATUS_GOOD: c_int = 0;
const SANE_STATUS_EOF: c_int = 5;

const SANE_FRAME_GRAY: c_int = 0;
const SANE_FRAME_RGB: c_int = 1;

#[repr(C)]
struct SaneHandle(*mut c_void);

#[repr(C)]
struct SaneDevice {
    name: *const c_char,
    vendor: *const c_char,
    model: *const c_char,
    device_type: *const c_char,
}

#[repr(C)]
struct SaneParameters {
    format: c_int,
    last_frame: c_int,
    bytes_per_line: c_int,
    pixels_per_line: c_int,
    lines: c_int,
    depth: c_int,
}

#[repr(C)]
struct SaneOptionDescriptor {
    name: *const c_char,
    title: *const c_char,
    desc: *const c_char,
    option_type: c_int,
    unit: c_int,
    size: c_int,
    cap: c_int,
    constraint_type: c_int,
    // ... more fields would be needed for full implementation
}

#[link(name = "sane", kind = "dylib")]
extern "C" {
    fn sane_init(version_code: *mut c_int, authorize: *const c_void) -> c_int;
    fn sane_exit();
    fn sane_get_devices(device_list: *mut *const *const SaneDevice, local_only: c_int) -> c_int;
    fn sane_open(devicename: *const c_char, handle: *mut SaneHandle) -> c_int;
    fn sane_close(handle: SaneHandle);
    fn sane_get_option_descriptor(handle: SaneHandle, option: c_int)
        -> *const SaneOptionDescriptor;
    fn sane_control_option(
        handle: SaneHandle,
        option: c_int,
        action: c_int,
        value: *mut c_void,
        info: *mut c_int,
    ) -> c_int;
    fn sane_start(handle: SaneHandle) -> c_int;
    fn sane_get_parameters(handle: SaneHandle, params: *mut SaneParameters) -> c_int;
    fn sane_read(handle: SaneHandle, data: *mut u8, max_length: c_int, length: *mut c_int)
        -> c_int;
    fn sane_cancel(handle: SaneHandle);
}

pub struct SaneBackend {
    initialized: bool,
}

impl SaneBackend {
    pub fn new() -> Self {
        let mut version = 0;
        let initialized = unsafe { sane_init(&mut version, ptr::null()) == SANE_STATUS_GOOD };

        Self { initialized }
    }

    fn enumerate_devices(&self) -> Result<Vec<ScannerInfo>> {
        if !self.initialized {
            return Err(PapyrError::Backend("SANE not initialized".into()));
        }

        let mut device_list: *const *const SaneDevice = ptr::null();
        let status = unsafe { sane_get_devices(&mut device_list, 1) };

        if status != SANE_STATUS_GOOD {
            return Err(PapyrError::Backend(format!(
                "Failed to get devices: {}",
                status
            )));
        }

        let mut scanners = Vec::new();
        let mut i = 0;

        unsafe {
            while !(*device_list.offset(i)).is_null() {
                let device = &**device_list.offset(i);

                let name = if !device.name.is_null() {
                    CStr::from_ptr(device.name).to_string_lossy().into_owned()
                } else {
                    "Unknown".to_string()
                };

                let vendor = if !device.vendor.is_null() {
                    CStr::from_ptr(device.vendor).to_string_lossy().into_owned()
                } else {
                    "".to_string()
                };

                let model = if !device.model.is_null() {
                    CStr::from_ptr(device.model).to_string_lossy().into_owned()
                } else {
                    "".to_string()
                };

                scanners.push(ScannerInfo {
                    id: format!("sane_{}", name.replace(":", "_")),
                    name: format!("{} {}", vendor, model).trim().to_string(),
                    backend: Backend::Sane,
                });

                i += 1;
            }
        }

        Ok(scanners)
    }

    fn get_device_capabilities(&self, device_name: &str) -> Result<Capabilities> {
        let device_name_c = CString::new(device_name)
            .map_err(|_| PapyrError::InvalidConfig("Invalid device name".into()))?;

        let mut handle = SaneHandle(ptr::null_mut());
        let status = unsafe { sane_open(device_name_c.as_ptr(), &mut handle) };

        if status != SANE_STATUS_GOOD {
            return Err(PapyrError::Backend(format!(
                "Failed to open device: {}",
                status
            )));
        }

        // Parse SANE options to build capabilities
        let mut dpis = Vec::new();
        let mut color_modes = Vec::new();
        let mut sources = vec![ScanSource::Flatbed]; // Default to flatbed

        // Option indices vary by device, typically:
        // 0-3: Standard options (count, group, etc.)
        // 4+: Device-specific options
        for i in 0..100 {
            let desc = unsafe { sane_get_option_descriptor(handle, i) };
            if desc.is_null() {
                break;
            }

            unsafe {
                if !(*desc).name.is_null() {
                    let name = CStr::from_ptr((*desc).name).to_string_lossy();

                    match name.as_ref() {
                        "resolution" => {
                            // Typically supports 75, 150, 300, 600
                            dpis = vec![75, 150, 300, 600];
                        }
                        "mode" => {
                            color_modes = vec![ColorMode::Color, ColorMode::Gray, ColorMode::Bw];
                        }
                        "source" => {
                            sources = vec![ScanSource::Flatbed, ScanSource::Adf];
                        }
                        _ => {}
                    }
                }
            }
        }

        unsafe {
            sane_close(handle);
        }

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
            supports_duplex: false,
        })
    }
}

impl BackendProvider for SaneBackend {
    fn name(&self) -> &'static str {
        "SANE"
    }

    fn kind(&self) -> Backend {
        Backend::Sane
    }

    fn enumerate(&self) -> Vec<ScannerInfo> {
        self.enumerate_devices().unwrap_or_default()
    }

    fn capabilities(&self, device_id: &str) -> Result<Capabilities> {
        // Extract device name from ID (remove "sane_" prefix)
        let device_name = device_id
            .strip_prefix("sane_")
            .unwrap_or(device_id)
            .replace("_", ":");

        self.get_device_capabilities(&device_name)
    }

    fn start_scan(&self, device_id: &str, cfg: ScanConfig) -> Result<Box<dyn ScanSession>> {
        let device_name = device_id
            .strip_prefix("sane_")
            .unwrap_or(device_id)
            .replace("_", ":");

        let device_name_c = CString::new(device_name.clone())
            .map_err(|_| PapyrError::InvalidConfig("Invalid device name".into()))?;

        let mut handle = SaneHandle(ptr::null_mut());
        let status = unsafe { sane_open(device_name_c.as_ptr(), &mut handle) };

        if status != SANE_STATUS_GOOD {
            return Err(PapyrError::Backend(format!(
                "Failed to open device: {}",
                status
            )));
        }

        // TODO: Set options based on cfg (resolution, color mode, etc.)
        // This requires finding the option indices and setting values

        // Start scanning
        let status = unsafe { sane_start(handle) };
        if status != SANE_STATUS_GOOD {
            unsafe {
                sane_close(handle);
            }
            return Err(PapyrError::Backend(format!(
                "Failed to start scan: {}",
                status
            )));
        }

        Ok(Box::new(SaneScanSession {
            handle,
            buffer: vec![0u8; 32 * 1024], // 32KB buffer
            state: SaneScanState::Scanning,
            accumulated_data: Vec::new(),
        }))
    }
}

impl Drop for SaneBackend {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                sane_exit();
            }
        }
    }
}

struct SaneScanSession {
    handle: SaneHandle,
    buffer: Vec<u8>,
    state: SaneScanState,
    accumulated_data: Vec<u8>,
}

enum SaneScanState {
    Scanning,
    Complete,
}

impl ScanSession for SaneScanSession {
    fn next_event(&mut self) -> Result<Option<ScanEvent>> {
        match self.state {
            SaneScanState::Scanning => {
                let mut len: c_int = 0;
                let status = unsafe {
                    sane_read(
                        self.handle,
                        self.buffer.as_mut_ptr(),
                        self.buffer.len() as c_int,
                        &mut len,
                    )
                };

                match status {
                    SANE_STATUS_GOOD => {
                        if len > 0 {
                            self.accumulated_data
                                .extend_from_slice(&self.buffer[..len as usize]);
                        }
                        // Continue reading
                        Ok(Some(ScanEvent::PageData(vec![]))) // Empty event to signal progress
                    }
                    SANE_STATUS_EOF => {
                        self.state = SaneScanState::Complete;

                        // Get final parameters
                        let mut params = SaneParameters {
                            format: 0,
                            last_frame: 0,
                            bytes_per_line: 0,
                            pixels_per_line: 0,
                            lines: 0,
                            depth: 0,
                        };
                        unsafe {
                            sane_get_parameters(self.handle, &mut params);
                        }

                        let color_mode = match params.format {
                            SANE_FRAME_GRAY => ColorMode::Gray,
                            SANE_FRAME_RGB => ColorMode::Color,
                            _ => ColorMode::Bw,
                        };

                        let page_meta = PageMeta {
                            index: 0,
                            width_px: params.pixels_per_line as u32,
                            height_px: params.lines as u32,
                            dpi: 300, // Would need to query from options
                            color_mode,
                        };

                        let data = std::mem::take(&mut self.accumulated_data);

                        Ok(Some(ScanEvent::PageComplete(page_meta)))
                    }
                    _ => Err(PapyrError::Backend(format!("SANE read error: {}", status))),
                }
            }
            SaneScanState::Complete => Ok(None),
        }
    }
}

impl Drop for SaneScanSession {
    fn drop(&mut self) {
        unsafe {
            sane_cancel(self.handle);
            sane_close(self.handle);
        }
    }
}
