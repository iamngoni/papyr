#![allow(clippy::not_unsafe_ptr_arg_deref)]
//
//  papyr_core
//  ffi.rs - C FFI interface for Dart integration
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::sync::{Arc, Mutex};

use crate::models::*;
use crate::registry::BackendRegistry;

// Global registry instance
static mut REGISTRY: Option<Arc<Mutex<BackendRegistry>>> = None;
static mut SCAN_SESSIONS: Option<Arc<Mutex<HashMap<u32, Box<dyn ScanSession + Send>>>>> = None;
static mut NEXT_SESSION_ID: u32 = 1;

#[repr(C)]
pub struct CScannerInfo {
    pub id: *mut c_char,
    pub name: *mut c_char,
    pub backend: c_int, // Backend enum as int
}

#[repr(C)]
pub struct CScannerInfoList {
    pub scanners: *mut CScannerInfo,
    pub count: usize,
}

#[repr(C)]
pub struct CCapabilities {
    pub sources: *mut c_int,
    pub sources_count: usize,
    pub dpis: *mut c_int,
    pub dpis_count: usize,
    pub color_modes: *mut c_int,
    pub color_modes_count: usize,
    pub supports_duplex: c_int, // bool as int
}

#[repr(C)]
pub struct CScanConfig {
    pub source: c_int,
    pub duplex: c_int, // bool as int
    pub dpi: c_int,
    pub color_mode: c_int,
    pub page_width_mm: c_int,
    pub page_height_mm: c_int,
}

#[repr(C)]
pub struct CScanEvent {
    pub event_type: c_int, // ScanEvent type as int
    pub data: *mut c_void,
    pub data_size: usize,
}

// Initialize the papyr core library
#[no_mangle]
pub extern "C" fn papyr_init() -> c_int {
    unsafe {
        // BackendRegistry::new() automatically registers all available backends
        let registry = BackendRegistry::new();

        REGISTRY = Some(Arc::new(Mutex::new(registry)));
        SCAN_SESSIONS = Some(Arc::new(Mutex::new(HashMap::new())));

        0 // Success
    }
}

// Get list of available scanners
#[no_mangle]
pub extern "C" fn papyr_list_scanners() -> *mut CScannerInfoList {
    unsafe {
        if let Some(registry) = &REGISTRY {
            if let Ok(guard) = registry.lock() {
                match guard.list_devices() {
                    Ok(scanners) => {
                        let c_scanners: Vec<CScannerInfo> = scanners
                            .into_iter()
                            .map(|scanner| CScannerInfo {
                                id: CString::new(scanner.id).unwrap().into_raw(),
                                name: CString::new(scanner.name).unwrap().into_raw(),
                                backend: backend_to_int(scanner.backend),
                            })
                            .collect();

                        let c_scanners_ptr = Box::into_raw(c_scanners.into_boxed_slice());
                        let list = Box::new(CScannerInfoList {
                            scanners: c_scanners_ptr.cast(),
                            count: (*c_scanners_ptr).len(),
                        });

                        Box::into_raw(list)
                    }
                    Err(_) => std::ptr::null_mut(),
                }
            } else {
                std::ptr::null_mut()
            }
        } else {
            std::ptr::null_mut()
        }
    }
}

// Get scanner capabilities
#[no_mangle]
pub extern "C" fn papyr_get_capabilities(device_id: *const c_char) -> *mut CCapabilities {
    unsafe {
        if device_id.is_null() {
            return std::ptr::null_mut();
        }

        let device_id_str = match CStr::from_ptr(device_id).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        if let Some(registry) = &REGISTRY {
            if let Ok(guard) = registry.lock() {
                match guard.capabilities(device_id_str) {
                    Ok(caps) => {
                        let sources: Vec<c_int> = caps
                            .sources
                            .iter()
                            .map(|&s| scan_source_to_int(s))
                            .collect();
                        let dpis: Vec<c_int> = caps.dpis.iter().map(|&d| d as c_int).collect();
                        let color_modes: Vec<c_int> = caps
                            .color_modes
                            .iter()
                            .map(|&c| color_mode_to_int(c))
                            .collect();

                        let c_caps = Box::new(CCapabilities {
                            sources: Box::into_raw(sources.into_boxed_slice()).cast(),
                            sources_count: caps.sources.len(),
                            dpis: Box::into_raw(dpis.into_boxed_slice()).cast(),
                            dpis_count: caps.dpis.len(),
                            color_modes: Box::into_raw(color_modes.into_boxed_slice()).cast(),
                            color_modes_count: caps.color_modes.len(),
                            supports_duplex: if caps.supports_duplex { 1 } else { 0 },
                        });

                        Box::into_raw(c_caps)
                    }
                    Err(_) => std::ptr::null_mut(),
                }
            } else {
                std::ptr::null_mut()
            }
        } else {
            std::ptr::null_mut()
        }
    }
}

// Start a scan session
#[no_mangle]
pub extern "C" fn papyr_start_scan(device_id: *const c_char, config: *const CScanConfig) -> c_int {
    unsafe {
        if device_id.is_null() || config.is_null() {
            return -1;
        }

        let device_id_str = match CStr::from_ptr(device_id).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };

        let c_config = &*config;
        let scan_config = ScanConfig {
            source: int_to_scan_source(c_config.source),
            duplex: c_config.duplex != 0,
            dpi: c_config.dpi as u32,
            color_mode: int_to_color_mode(c_config.color_mode),
            page_size: PageSize {
                width_mm: c_config.page_width_mm as u32,
                height_mm: c_config.page_height_mm as u32,
            },
            area: None,
            brightness: None,
            contrast: None,
            max_pages: None,
        };

        if let Some(registry) = &REGISTRY {
            if let Ok(guard) = registry.lock() {
                match guard.start_scan(device_id_str, scan_config) {
                    Ok(session) => {
                        if let Some(sessions) = &SCAN_SESSIONS {
                            if let Ok(mut sessions_guard) = sessions.lock() {
                                let session_id = NEXT_SESSION_ID;
                                NEXT_SESSION_ID += 1;
                                sessions_guard.insert(session_id, session);
                                return session_id as c_int;
                            }
                        }
                        -1
                    }
                    Err(_) => -1,
                }
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

// Get next scan event
#[no_mangle]
pub extern "C" fn papyr_next_scan_event(session_id: c_int) -> *mut CScanEvent {
    unsafe {
        if let Some(sessions) = &SCAN_SESSIONS {
            if let Ok(mut sessions_guard) = sessions.lock() {
                if let Some(session) = sessions_guard.get_mut(&(session_id as u32)) {
                    match session.next_event() {
                        Ok(Some(event)) => {
                            let c_event = Box::new(CScanEvent {
                                event_type: scan_event_to_int(&event),
                                data: std::ptr::null_mut(), // Simplified for now
                                data_size: 0,
                            });
                            Box::into_raw(c_event)
                        }
                        Ok(None) => std::ptr::null_mut(),
                        Err(_) => std::ptr::null_mut(),
                    }
                } else {
                    std::ptr::null_mut()
                }
            } else {
                std::ptr::null_mut()
            }
        } else {
            std::ptr::null_mut()
        }
    }
}

// Cleanup functions
#[no_mangle]
pub extern "C" fn papyr_free_scanner_list(list: *mut CScannerInfoList) {
    unsafe {
        if !list.is_null() {
            let list = Box::from_raw(list);
            let scanners = Box::from_raw(std::slice::from_raw_parts_mut(list.scanners, list.count));
            for scanner in scanners.iter() {
                if !scanner.id.is_null() {
                    drop(CString::from_raw(scanner.id));
                }
                if !scanner.name.is_null() {
                    drop(CString::from_raw(scanner.name));
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn papyr_free_capabilities(caps: *mut CCapabilities) {
    unsafe {
        if !caps.is_null() {
            let caps = Box::from_raw(caps);
            if !caps.sources.is_null() {
                drop(Box::from_raw(std::slice::from_raw_parts_mut(
                    caps.sources,
                    caps.sources_count,
                )));
            }
            if !caps.dpis.is_null() {
                drop(Box::from_raw(std::slice::from_raw_parts_mut(
                    caps.dpis,
                    caps.dpis_count,
                )));
            }
            if !caps.color_modes.is_null() {
                drop(Box::from_raw(std::slice::from_raw_parts_mut(
                    caps.color_modes,
                    caps.color_modes_count,
                )));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn papyr_free_scan_event(event: *mut CScanEvent) {
    unsafe {
        if !event.is_null() {
            drop(Box::from_raw(event));
        }
    }
}

#[no_mangle]
pub extern "C" fn papyr_cleanup() {
    unsafe {
        REGISTRY = None;
        SCAN_SESSIONS = None;
        NEXT_SESSION_ID = 1;
    }
}

// Helper conversion functions
fn backend_to_int(backend: Backend) -> c_int {
    match backend {
        Backend::Wia => 0,
        Backend::Sane => 1,
        Backend::Ica => 2,
        Backend::Escl => 3,
        Backend::Unknown => 99,
    }
}

fn scan_source_to_int(source: ScanSource) -> c_int {
    match source {
        ScanSource::Flatbed => 0,
        ScanSource::Adf => 1,
        ScanSource::AdfDuplex => 2,
    }
}

fn int_to_scan_source(val: c_int) -> ScanSource {
    match val {
        0 => ScanSource::Flatbed,
        1 => ScanSource::Adf,
        2 => ScanSource::AdfDuplex,
        _ => ScanSource::Flatbed,
    }
}

fn color_mode_to_int(mode: ColorMode) -> c_int {
    match mode {
        ColorMode::Color => 0,
        ColorMode::Gray => 1,
        ColorMode::Bw => 2,
    }
}

fn int_to_color_mode(val: c_int) -> ColorMode {
    match val {
        0 => ColorMode::Color,
        1 => ColorMode::Gray,
        2 => ColorMode::Bw,
        _ => ColorMode::Color,
    }
}

fn scan_event_to_int(event: &ScanEvent) -> c_int {
    match event {
        ScanEvent::PageStarted(_) => 0,
        ScanEvent::PageData(_) => 1,
        ScanEvent::PageComplete(_) => 2,
        ScanEvent::JobComplete => 3,
    }
}
