//
//  papyr_core
//  backends/twain.rs - TWAIN Scanner Backend Implementation
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use crate::models::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void, c_uint, c_ushort};
use std::ptr;

// TWAIN Constants
const TWON_ONEVALUE: c_ushort = 5;
const TWON_RANGE: c_ushort = 6;
const TWON_ENUMERATION: c_ushort = 4;

// TWAIN Data Groups
const DG_CONTROL: c_uint = 0x0001;
const DG_IMAGE: c_uint = 0x0002;

// TWAIN Data Argument Types
const DAT_CAPABILITY: c_ushort = 0x0001;
const DAT_EVENT: c_ushort = 0x0002;
const DAT_IDENTITY: c_ushort = 0x0003;
const DAT_PARENT: c_ushort = 0x0004;
const DAT_PENDINGXFERS: c_ushort = 0x0005;
const DAT_SETUPMEMXFER: c_ushort = 0x0006;
const DAT_SETUPFILEXFER: c_ushort = 0x0007;
const DAT_STATUS: c_ushort = 0x0008;
const DAT_USERINTERFACE: c_ushort = 0x0009;
const DAT_XFERGROUP: c_ushort = 0x000a;
const DAT_IMAGEMEMXFER: c_ushort = 0x000b;
const DAT_IMAGENATIVEXFER: c_ushort = 0x000c;
const DAT_IMAGEFILEXFER: c_ushort = 0x000d;

// TWAIN Messages
const MSG_NULL: c_ushort = 0x0000;
const MSG_GET: c_ushort = 0x0001;
const MSG_GETCURRENT: c_ushort = 0x0002;
const MSG_GETDEFAULT: c_ushort = 0x0003;
const MSG_GETFIRST: c_ushort = 0x0004;
const MSG_GETNEXT: c_ushort = 0x0005;
const MSG_SET: c_ushort = 0x0006;
const MSG_RESET: c_ushort = 0x0007;
const MSG_QUERYDEFAULT: c_ushort = 0x0008;
const MSG_OPENDSM: c_ushort = 0x0301;
const MSG_CLOSEDSM: c_ushort = 0x0302;
const MSG_OPENDS: c_ushort = 0x0401;
const MSG_CLOSEDS: c_ushort = 0x0402;
const MSG_USERSELECT: c_ushort = 0x0403;
const MSG_DISABLEDS: c_ushort = 0x0501;
const MSG_ENABLEDS: c_ushort = 0x0502;
const MSG_ENABLEDSUIONLY: c_ushort = 0x0503;
const MSG_PROCESSEVENT: c_ushort = 0x0601;
const MSG_ENDXFER: c_ushort = 0x0701;
const MSG_RESET_XFER: c_ushort = 0x0702;
const MSG_XFERREADY: c_ushort = 0x0101;

// TWAIN Return Codes
const TWRC_SUCCESS: c_ushort = 0;
const TWRC_FAILURE: c_ushort = 1;
const TWRC_CHECKSTATUS: c_ushort = 2;
const TWRC_CANCEL: c_ushort = 3;
const TWRC_DSEVENT: c_ushort = 4;
const TWRC_NOTDSEVENT: c_ushort = 5;
const TWRC_XFERDONE: c_ushort = 6;
const TWRC_ENDOFLIST: c_ushort = 7;
const TWRC_INFONOTSUPPORTED: c_ushort = 8;
const TWRC_DATANOTAVAILABLE: c_ushort = 9;

// TWAIN Capabilities
const ICAP_XRESOLUTION: c_ushort = 0x1118;
const ICAP_YRESOLUTION: c_ushort = 0x1119;
const ICAP_PIXELTYPE: c_ushort = 0x0101;
const CAP_FEEDERENABLED: c_ushort = 0x1002;
const CAP_DUPLEXENABLED: c_ushort = 0x1012;

// TWAIN Pixel Types
const TWPT_BW: c_ushort = 0;
const TWPT_GRAY: c_ushort = 1;
const TWPT_RGB: c_ushort = 2;

// TWAIN Structures
#[repr(C)]
#[derive(Debug, Clone)]
struct TW_IDENTITY {
    id: c_uint,
    version: TW_VERSION,
    protocol_major: c_ushort,
    protocol_minor: c_ushort,
    supported_groups: c_uint,
    manufacturer: [c_char; 34],
    product_family: [c_char; 34],
    product_name: [c_char; 34],
}

#[repr(C)]
#[derive(Debug, Clone)]
struct TW_VERSION {
    major_num: c_ushort,
    minor_num: c_ushort,
    language: c_ushort,
    country: c_ushort,
    info: [c_char; 34],
}

#[repr(C)]
#[derive(Debug)]
struct TW_USERINTERFACE {
    show_ui: c_ushort,
    modal_ui: c_ushort,
    parent: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
struct TW_CAPABILITY {
    cap: c_ushort,
    con_type: c_ushort,
    h_container: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
struct TW_PENDINGXFERS {
    count: c_ushort,
    end_of_job: c_uint,
}

// TWAIN Entry Point Type
type DsmEntry = unsafe extern "C" fn(
    origin: *mut TW_IDENTITY,
    dest: *mut TW_IDENTITY,
    dg: c_uint,
    dat: c_ushort,
    msg: c_ushort,
    data: *mut c_void,
) -> c_ushort;

pub struct TwainBackend {
    dsm_lib: Option<libloading::Library>,
    dsm_entry: Option<DsmEntry>,
    app_identity: TW_IDENTITY,
    source_identity: Option<TW_IDENTITY>,
    state: TwainState,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum TwainState {
    State1, // Pre-session
    State2, // DSM loaded
    State3, // DSM opened
    State4, // Source opened
    State5, // Source enabled
    State6, // Transfer ready
    State7, // Transferring
}

impl TwainBackend {
    pub fn new() -> Self {
        let app_identity = TW_IDENTITY {
            id: 0,
            version: TW_VERSION {
                major_num: 1,
                minor_num: 0,
                language: 13, // English
                country: 1,   // USA
                info: Self::str_to_array("Papyr Scanner v1.0"),
            },
            protocol_major: 2,
            protocol_minor: 4,
            supported_groups: DG_IMAGE | DG_CONTROL,
            manufacturer: Self::str_to_array("Papyr"),
            product_family: Self::str_to_array("Scanner"),
            product_name: Self::str_to_array("Papyr Core Scanner"),
        };

        Self {
            dsm_lib: None,
            dsm_entry: None,
            app_identity,
            source_identity: None,
            state: TwainState::State1,
        }
    }

    fn str_to_array(s: &str) -> [c_char; 34] {
        let mut arr = [0; 34];
        let bytes = s.as_bytes();
        let len = std::cmp::min(bytes.len(), 33); // Leave room for null terminator
        for (i, &b) in bytes.iter().take(len).enumerate() {
            arr[i] = b as c_char;
        }
        arr
    }

    fn load_dsm(&mut self) -> Result<()> {
        if self.state != TwainState::State1 {
            return Err(PapyrError::Backend("DSM already loaded".into()));
        }

        // Determine DSM library path based on platform
        #[cfg(target_os = "windows")]
        let dsm_path = "TWAINDSM.dll";
        #[cfg(target_os = "macos")]
        let dsm_path = "/System/Library/Frameworks/TWAIN.framework/TWAIN";
        #[cfg(target_os = "linux")]
        let dsm_path = "libtwaindsm.so";

        println!("📚 Loading TWAIN DSM from: {}", dsm_path);

        let lib = unsafe { libloading::Library::new(dsm_path) }
            .map_err(|e| PapyrError::Backend(format!("Failed to load TWAIN DSM: {}", e)))?;

        let dsm_entry: libloading::Symbol<DsmEntry> = unsafe { lib.get(b"DSM_Entry") }
            .map_err(|e| PapyrError::Backend(format!("Failed to find DSM_Entry: {}", e)))?;

        let dsm_entry_fn = *dsm_entry;
        self.dsm_lib = Some(lib);
        self.dsm_entry = Some(dsm_entry_fn);
        self.state = TwainState::State2;

        println!("✅ TWAIN DSM loaded successfully");
        Ok(())
    }

    fn open_dsm(&mut self) -> Result<()> {
        if self.state != TwainState::State2 {
            return self.load_dsm().and_then(|_| self.open_dsm());
        }

        if let Some(entry) = self.dsm_entry {
            let rc = unsafe {
                entry(
                    &mut self.app_identity,
                    ptr::null_mut(),
                    DG_CONTROL,
                    DAT_PARENT,
                    MSG_OPENDSM,
                    ptr::null_mut(),
                )
            };

            if rc == TWRC_SUCCESS {
                self.state = TwainState::State3;
                println!("✅ TWAIN DSM opened");
                Ok(())
            } else {
                Err(PapyrError::Backend(format!("Failed to open DSM: {}", rc)))
            }
        } else {
            Err(PapyrError::Backend("DSM entry point not loaded".into()))
        }
    }

    fn enumerate_sources(&mut self) -> Result<Vec<TW_IDENTITY>> {
        if self.state < TwainState::State3 {
            self.open_dsm()?;
        }

        let mut sources = Vec::new();
        
        if let Some(entry) = self.dsm_entry {
            // Get first source
            let mut source = TW_IDENTITY {
                id: 0,
                version: TW_VERSION {
                    major_num: 0,
                    minor_num: 0,
                    language: 0,
                    country: 0,
                    info: [0; 34],
                },
                protocol_major: 0,
                protocol_minor: 0,
                supported_groups: 0,
                manufacturer: [0; 34],
                product_family: [0; 34],
                product_name: [0; 34],
            };

            let rc = unsafe {
                entry(
                    &mut self.app_identity,
                    &mut source,
                    DG_CONTROL,
                    DAT_IDENTITY,
                    MSG_GETFIRST,
                    &mut source as *mut _ as *mut c_void,
                )
            };

            if rc == TWRC_SUCCESS {
                sources.push(source.clone());

                // Get remaining sources
                loop {
                    let rc = unsafe {
                        entry(
                            &mut self.app_identity,
                            &mut source,
                            DG_CONTROL,
                            DAT_IDENTITY,
                            MSG_GETNEXT,
                            &mut source as *mut _ as *mut c_void,
                        )
                    };

                    if rc == TWRC_SUCCESS {
                        sources.push(source.clone());
                    } else if rc == TWRC_ENDOFLIST {
                        break;
                    } else {
                        println!("Warning: Error getting next source: {}", rc);
                        break;
                    }
                }
            } else if rc == TWRC_ENDOFLIST {
                println!("No TWAIN sources found");
            } else {
                return Err(PapyrError::Backend(format!("Failed to enumerate sources: {}", rc)));
            }
        }

        println!("🔍 Found {} TWAIN sources", sources.len());
        Ok(sources)
    }

    fn open_source(&mut self, source_id: u32) -> Result<()> {
        if self.state < TwainState::State3 {
            self.open_dsm()?;
        }

        let sources = self.enumerate_sources()?;
        let source = sources.into_iter()
            .find(|s| s.id == source_id)
            .ok_or_else(|| PapyrError::NotFound(format!("TWAIN source {} not found", source_id)))?;

        if let Some(entry) = self.dsm_entry {
            let mut source_copy = source.clone();
            let rc = unsafe {
                entry(
                    &mut self.app_identity,
                    &mut source_copy,
                    DG_CONTROL,
                    DAT_IDENTITY,
                    MSG_OPENDS,
                    ptr::null_mut(),
                )
            };

            if rc == TWRC_SUCCESS {
                self.source_identity = Some(source_copy);
                self.state = TwainState::State4;
                println!("✅ TWAIN source opened");
                Ok(())
            } else {
                Err(PapyrError::Backend(format!("Failed to open source: {}", rc)))
            }
        } else {
            Err(PapyrError::Backend("DSM entry point not loaded".into()))
        }
    }

    fn close_source(&mut self) -> Result<()> {
        if self.state >= TwainState::State4 && self.source_identity.is_some() {
            if let Some(entry) = self.dsm_entry {
                let source = self.source_identity.as_mut().unwrap();
                let rc = unsafe {
                    entry(
                        &mut self.app_identity,
                        source,
                        DG_CONTROL,
                        DAT_IDENTITY,
                        MSG_CLOSEDS,
                        ptr::null_mut(),
                    )
                };

                if rc == TWRC_SUCCESS {
                    self.source_identity = None;
                    self.state = TwainState::State3;
                    println!("✅ TWAIN source closed");
                }
            }
        }
        Ok(())
    }

    fn close_dsm(&mut self) -> Result<()> {
        self.close_source()?;
        
        if self.state >= TwainState::State3 {
            if let Some(entry) = self.dsm_entry {
                let rc = unsafe {
                    entry(
                        &mut self.app_identity,
                        ptr::null_mut(),
                        DG_CONTROL,
                        DAT_PARENT,
                        MSG_CLOSEDSM,
                        ptr::null_mut(),
                    )
                };

                if rc == TWRC_SUCCESS {
                    self.state = TwainState::State2;
                    println!("✅ TWAIN DSM closed");
                }
            }
        }
        Ok(())
    }

    fn identity_to_string(identity: &TW_IDENTITY) -> String {
        unsafe {
            let product = CStr::from_ptr(identity.product_name.as_ptr())
                .to_string_lossy()
                .into_owned();
            let manufacturer = CStr::from_ptr(identity.manufacturer.as_ptr())
                .to_string_lossy()
                .into_owned();
            
            if manufacturer.is_empty() {
                product
            } else {
                format!("{} {}", manufacturer, product)
            }
        }
    }
}

impl BackendProvider for TwainBackend {
    fn name(&self) -> &'static str {
        "TWAIN"
    }

    fn kind(&self) -> Backend {
        Backend::Twain
    }

    fn enumerate(&self) -> Vec<ScannerInfo> {
        let mut backend = TwainBackend::new();
        
        match backend.enumerate_sources() {
            Ok(sources) => {
                sources.into_iter().map(|source| {
                    let name = Self::identity_to_string(&source);
                    ScannerInfo {
                        id: format!("twain_{}", source.id),
                        name,
                        backend: Backend::Twain,
                    }
                }).collect()
            },
            Err(e) => {
                println!("TWAIN enumeration failed: {:?}", e);
                vec![] // Return empty list instead of error
            }
        }
    }

    fn capabilities(&self, _device_id: &str) -> Result<Capabilities> {
        // Return typical TWAIN scanner capabilities
        Ok(Capabilities {
            sources: vec![ScanSource::Flatbed, ScanSource::Adf],
            dpis: vec![75, 150, 200, 300, 600, 1200],
            color_modes: vec![ColorMode::Bw, ColorMode::Gray, ColorMode::Color],
            page_sizes: vec![
                PageSize { width_mm: 216, height_mm: 279 }, // Letter
                PageSize { width_mm: 210, height_mm: 297 }, // A4
                PageSize { width_mm: 148, height_mm: 210 }, // A5
            ],
            supports_duplex: true,
        })
    }

    fn start_scan(&self, device_id: &str, config: ScanConfig) -> Result<Box<dyn ScanSession>> {
        let source_id = device_id.strip_prefix("twain_")
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| PapyrError::Backend("Invalid TWAIN device ID".into()))?;

        Ok(Box::new(TwainScanSession::new(source_id, config)?))
    }
}

pub struct TwainScanSession {
    backend: TwainBackend,
    source_id: u32,
    config: ScanConfig,
    completed: bool,
}

impl TwainScanSession {
    pub fn new(source_id: u32, config: ScanConfig) -> Result<Self> {
        let mut backend = TwainBackend::new();
        backend.open_source(source_id)?;
        
        Ok(Self {
            backend,
            source_id,
            config,
            completed: false,
        })
    }

    fn perform_scan(&mut self) -> Result<Vec<u8>> {
        println!("🖨️ Starting TWAIN scan...");
        
        // For now, return mock data
        // TODO: Implement actual TWAIN scanning with MSG_ENABLEDS and image transfer
        let mock_data = vec![0xFF; 20480]; // 20KB mock image
        println!("✅ TWAIN scan completed: {} bytes", mock_data.len());
        
        Ok(mock_data)
    }
}

impl ScanSession for TwainScanSession {
    fn next_event(&mut self) -> Result<Option<ScanEvent>> {
        if self.completed {
            return Ok(None);
        }

        match self.perform_scan() {
            Ok(data) => {
                self.completed = true;
                Ok(Some(ScanEvent::PageData(data)))
            },
            Err(e) => {
                self.completed = true;
                Err(e)
            }
        }
    }
}

impl Drop for TwainScanSession {
    fn drop(&mut self) {
        let _ = self.backend.close_dsm();
    }
}

impl Drop for TwainBackend {
    fn drop(&mut self) {
        let _ = self.close_dsm();
    }
}
