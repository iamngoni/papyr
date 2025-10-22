# Papyr Core - Implementation Notes & Resources

> Comprehensive guide for implementing cross-platform document scanning in Rust with FFI bindings for Dart

## Table of Contents

1. [Project Overview](#project-overview)
2. [Protocol Specifications](#protocol-specifications)
3. [Implementation Strategy](#implementation-strategy)
4. [Backend Details](#backend-details)
5. [Architecture Notes](#architecture-notes)
6. [FFI Design](#ffi-design)
7. [Testing Strategy](#testing-strategy)
8. [References & Resources](#references--resources)

---

## Project Overview

Building a Rust core library to support document scanning across platforms (Windows, macOS, Linux) with:

- **Primary Use**: Dart/Flutter integration via FFI
- **Backends**: WIA, TWAIN, SANE, eSCL, ICA
- **Platform Strategy**: Feature flags for platform-specific backends
- **Cross-platform**: eSCL (network scanners) works on all platforms

### Current Status

| Protocol | Platform | Status | Priority |
|----------|----------|--------|----------|
| WIA      | Windows  | ✅ In Progress | High |
| eSCL     | All      | 🚧 Planned | High |
| ICA      | macOS    | 🚧 Planned | Medium |
| SANE     | Linux    | 🚧 Planned | Medium |
| TWAIN    | All      | 🔮 Future | Low |

---

## Protocol Specifications

### 1. Windows Image Acquisition (WIA)

#### Official Resources

- **Driver Development**: [WIA Driver Development Overview](https://learn.microsoft.com/en-us/windows-hardware/drivers/image/wia-driver-development)
- **API Reference**: [WIA Programming Guide](https://learn.microsoft.com/en-us/windows/win32/wia/-wia-startpage)
- **COM Interfaces**: [IWiaDevMgr2](https://learn.microsoft.com/en-us/windows/win32/api/wia_xp/nn-wia_xp-iwiadevmgr2), [IWiaItem2](https://learn.microsoft.com/en-us/windows/win32/api/wia_xp/nn-wia_xp-iwiaitem2)

#### Key Concepts

- **COM-based API**: Uses WIA 2.0 COM interfaces
- **Item Tree**: Devices expose hierarchical item trees (root → scanners/cameras → pages)
- **Property Bags**: Device capabilities exposed as properties
- **Transfer Modes**: Memory, Stream, File
- **Events**: Device arrival/removal, scan button press

#### Rust Implementation Strategy

```rust
// Use windows crate for COM bindings
use windows::Win32::Devices::ImageAcquisition::*;
use windows::Win32::System::Com::*;

// Key flow:
// 1. CoInitialize() → COM initialization
// 2. CoCreateInstance(CLSID_WiaDevMgr2) → Get device manager
// 3. IWiaDevMgr2::EnumDeviceInfo() → List devices
// 4. IWiaDevMgr2::CreateDevice() → Open device
// 5. IWiaItem2::EnumChildItems() → Get scanner items
// 6. Set properties (DPI, ColorMode, Format)
// 7. IWiaTransfer::Download() → Acquire image
// 8. Release COM objects
```

#### Critical Properties

```rust
// Common WIA properties to expose
const WIA_DPS_HORIZONTAL_BED_SIZE: u32 = 3074;
const WIA_DPS_VERTICAL_BED_SIZE: u32 = 3075;
const WIA_IPS_XRES: u32 = 6147; // DPI X
const WIA_IPS_YRES: u32 = 6148; // DPI Y
const WIA_IPA_DATATYPE: u32 = 4103; // Color/Grayscale/BW
const WIA_IPA_DEPTH: u32 = 4104; // Bits per pixel
const WIA_IPS_PAGES: u32 = 3096; // ADF page count
const WIA_IPS_DOCUMENT_HANDLING_SELECT: u32 = 3088; // Flatbed/ADF
```

---

### 2. TWAIN (Legacy Cross-Platform)

#### Official Resources

- **Specification**: [TWAIN Working Group Spec + Headers](https://github.com/twain/twain-specification)
- **Reference DSM**: [Open-source TWAIN DSM](https://github.com/twain/twain-dsm)

#### Key Concepts

- **DSM**: Data Source Manager (middleware between app and driver)
- **Data Source**: Scanner/camera driver
- **State Machine**: 7 states (1=Pre-session → 7=Transfer ready)
- **Capabilities**: ICAP_* (image caps), CAP_* (general caps)
- **Transfer Mechanisms**:
  - **Native**: Driver-allocated memory
  - **Memory**: App-allocated buffers
  - **File**: Direct to file

#### Rust Implementation Strategy

```rust
// Use bindgen to generate twain.h bindings
// Load DSM dynamically
#[cfg(windows)]
const DSM_LIB: &str = "TWAINDSM.dll";
#[cfg(target_os = "macos")]
const DSM_LIB: &str = "/Library/Frameworks/TWAINDSM.framework/TWAINDSM";
#[cfg(target_os = "linux")]
const DSM_LIB: &str = "libtwaindsm.so";

// State machine flow:
// State 1-2: Load DSM
// State 3: DSM_Open
// State 4: DSM_OpenDS (open specific scanner)
// State 5: Set capabilities (ICAP_XRESOLUTION, etc.)
// State 6: Enable source (UI or without UI)
// State 7: Transfer image
// Unwind states to close
```

#### Capability Negotiation

```rust
// Example: Set DPI
let mut cap = TW_CAPABILITY {
    cap: ICAP_XRESOLUTION,
    con_type: TWON_ONEVALUE,
    h_container: /* TW_ONEVALUE with 300 DPI */,
};
DSM_Entry(&app_id, &source, DG_CONTROL, DAT_CAPABILITY, MSG_SET, &mut cap);
```

---

### 3. SANE (Scanner Access Now Easy - Linux)

#### Official Resources

- **Standard**: [SANE Frontend/Backend API](https://gitlab.com/sane-project/sane-backends/-/blob/master/doc/sane.tex)
- **Backend Writing**: [Backend Development Guide](https://sane-project.org/docs.html)

#### Key Concepts

- **Frontend/Backend Separation**: App (frontend) talks to `libsane`, which loads device backends
- **Discovery**: `sane_get_devices()` lists all available scanners
- **Options**: Backend-specific capabilities (resolution, mode, source, etc.)
- **Frame-based Transfer**: Read scan data in frames/chunks

#### Rust Implementation Strategy

```rust
// FFI to libsane (C library)
#[link(name = "sane")]
extern "C" {
    fn sane_init(version_code: *mut i32, auth: Option<AuthCallback>) -> Status;
    fn sane_get_devices(device_list: *mut *const *const Device, local_only: bool) -> Status;
    fn sane_open(device_name: *const c_char, handle: *mut Handle) -> Status;
    fn sane_get_option_descriptor(handle: Handle, option: i32) -> *const OptionDescriptor;
    fn sane_control_option(handle: Handle, option: i32, action: i32, value: *mut c_void, info: *mut i32) -> Status;
    fn sane_start(handle: Handle) -> Status;
    fn sane_read(handle: Handle, data: *mut u8, max_length: i32, length: *mut i32) -> Status;
    fn sane_close(handle: Handle);
}

// Flow:
// 1. sane_init() → Initialize library
// 2. sane_get_devices() → List scanners
// 3. sane_open(name) → Open device
// 4. Loop sane_get_option_descriptor() → Enumerate capabilities
// 5. sane_control_option() → Set resolution, color mode, etc.
// 6. sane_start() → Begin scan
// 7. Loop sane_read() → Stream image data
// 8. sane_close() → Release device
```

#### Option Mapping

```rust
// Map SANE options to unified capability model
match option.name {
    "resolution" => Capability::Resolution,
    "mode" => Capability::ColorMode, // "Color", "Gray", "Lineart"
    "source" => Capability::Source, // "Flatbed", "ADF", "ADF Duplex"
    "tl-x" | "tl-y" | "br-x" | "br-y" => Capability::ScanArea,
    _ => Capability::Unknown(option.name.to_owned()),
}
```

---

### 4. eSCL / AirScan (Driverless Network Scanning)

#### Official Resources

- **Specification**: [Mopria eSCL Spec (PDF)](https://mopria.org/spec-download)
- **Reference Implementation**: [sane-airscan (C)](https://github.com/alexpevzner/sane-airscan)
- **Background**: [OpenPrinting eSCL Article](https://openprinting.github.io/achievements/#the-new-architecture-for-printing-and-scanning)

#### Key Concepts

- **HTTP/HTTPS + XML**: REST-like API for scanning
- **mDNS Discovery**: Devices advertise via `_uscan._tcp` (HTTP) / `_uscans._tcp` (HTTPS)
- **Workflow**:
  1. DNS-SD discovery → find scanner IP + port
  2. GET `/eSCL/ScannerCapabilities` → XML with supported settings
  3. POST `/eSCL/ScanJobs` with XML settings → Returns job URL
  4. GET `/eSCL/ScanJobs/{uuid}/NextDocument` → Binary image data
  5. DELETE `/eSCL/ScanJobs/{uuid}` → Cancel job

#### Rust Implementation Strategy

```rust
// Pure Rust implementation (works on all platforms!)
// Dependencies: reqwest, quick-xml, mdns-sd

// Discovery
use mdns_sd::{ServiceDaemon, ServiceEvent};

let mdns = ServiceDaemon::new()?;
let receiver = mdns.browse("_uscan._tcp.local.")?;

for event in receiver {
    if let ServiceEvent::ServiceResolved(info) = event {
        let capabilities_url = format!("http://{}:{}/eSCL/ScannerCapabilities",
            info.get_addresses().iter().next().unwrap(),
            info.get_port()
        );
        // Fetch capabilities...
    }
}

// Capabilities
#[derive(Deserialize)]
struct ScannerCapabilities {
    #[serde(rename = "MakeAndModel")]
    make_model: String,
    #[serde(rename = "SettingProfiles")]
    profiles: SettingProfiles,
}

let caps: ScannerCapabilities = quick_xml::de::from_str(&xml)?;

// Scan job
#[derive(Serialize)]
struct ScanSettings {
    #[serde(rename = "InputSource")]
    input_source: String, // "Platen", "Feeder"
    #[serde(rename = "XResolution")]
    x_resolution: u32,
    #[serde(rename = "YResolution")]
    y_resolution: u32,
    #[serde(rename = "ColorMode")]
    color_mode: String, // "RGB24", "Grayscale8", "BlackAndWhite1"
}

let body = quick_xml::se::to_string(&ScanSettings { /* ... */ })?;
let resp = client.post(format!("{}/eSCL/ScanJobs", base_url))
    .header("Content-Type", "text/xml")
    .body(body)
    .send()
    .await?;

let location = resp.headers().get("Location").unwrap().to_str()?;
// GET {location}/NextDocument to retrieve image
```

#### Common Quirks (from sane-airscan)

- **TXT Record Parsing**: `rs=` key contains relative path to capabilities
- **Multi-page**: Poll `/NextDocument` until HTTP 404
- **ADF/Duplex**: InputSource = "Feeder" + DocumentFormat can specify duplex
- **HTTPS**: Many devices use self-signed certs; may need to disable verification

---

### 5. Image Capture Architecture (ICA - macOS)

#### Official Resources

- **Modern API**: [ImageCaptureCore Framework](https://developer.apple.com/documentation/imagecapturecore)
- **Historical Context**: [Archived ICA Programming Guide](https://developer.apple.com/library/archive/documentation/Carbon/Conceptual/ICA_Guide/)
- **eSCL on macOS**: Apple's Image Capture app supports AirScan; [HP Documentation](https://support.hp.com/us-en/document/ish_1841315-1659297-16)

#### Key Concepts

- **ImageCaptureCore**: Objective-C framework for user-facing scanner/camera apps
- **Device Discovery**: ICDeviceBrowser for USB/network devices
- **ICScannerDevice**: Represents scanner, exposes functional units (flatbed, ADF)
- **ICScannerFunctionalUnit**: Per-unit capabilities (resolution, color, pixel type)
- **Delegate Pattern**: Callbacks for device discovery, scan progress, completion

#### Rust Implementation Strategy

**Option A: Objective-C FFI** (for USB/native devices)

```rust
// Use objc2 crate for Objective-C runtime
use objc2::runtime::*;
use objc2::*;

// Roughly:
// 1. [ICDeviceBrowser new]
// 2. browser.delegate = our Rust object (via msg_send!)
// 3. [browser start]
// 4. Delegate receives deviceDidBecomeReady: callback
// 5. Cast to ICScannerDevice
// 6. Configure scannerFunctionalUnits[0] (set resolution, etc.)
// 7. [device requestScan] → delegate receives didCompleteWithImage:

// Challenge: Bridging Objective-C blocks/delegates to Rust
```

**Option B: Use eSCL** (simpler, network devices only)

```rust
// On macOS, prefer eSCL for network scanners
// Fall back to ImageCaptureCore only for USB-only devices
// This reduces Objective-C FFI complexity
```

**Recommended Approach**: Start with eSCL (works everywhere), add ImageCaptureCore later if USB support needed.

---

## Implementation Strategy

### Phase 1: eSCL Foundation (High Priority)

**Why First?**
- Pure Rust, works on all platforms
- Modern driverless scanning
- No complex FFI to platform APIs

**Tasks:**
1. mDNS discovery (`mdns-sd` crate)
2. HTTP client for eSCL endpoints (`reqwest`)
3. XML parsing (`quick-xml` or `serde-xml-rs`)
4. Capability model mapping
5. Multi-page/ADF support
6. Error handling for network issues

**Deliverable**: eSCL backend in `src/backends/escl.rs` (already created!)

### Phase 2: WIA Polish (High Priority)

**Current Status**: `src/backends/wia.rs` exists

**Tasks:**
1. Complete COM interface bindings
2. Property bag parsing (DPI, color, ADF)
3. IWiaTransfer callback implementation
4. Memory transfer mode
5. Error mapping (COM HRESULT → Rust errors)
6. Test with real scanner hardware

### Phase 3: SANE (Medium Priority)

**Tasks:**
1. Generate `libsane` FFI bindings (`bindgen`)
2. Dynamic library loading (handle missing libsane gracefully)
3. Option descriptor parsing
4. Frame-based data transfer
5. Test with physical scanner on Linux

### Phase 4: ICA (Medium Priority)

**Decision Point**: ImageCaptureCore FFI vs eSCL-only?

**If FFI**:
1. Set up Objective-C bridge (`objc2`)
2. ICDeviceBrowser integration
3. Delegate pattern in Rust
4. Sandbox/entitlement handling

**If eSCL-only**:
- Document USB limitations
- Rely on eSCL backend

### Phase 5: TWAIN (Low Priority / Optional)

**Rationale**: WIA (Windows) and eSCL cover most use cases

**If Needed**:
1. Generate `twain.h` bindings
2. DSM dynamic loading
3. State machine implementation
4. Capability negotiation

---

## Architecture Notes

### Core Abstraction Layer

```rust
// src/models.rs

/// Unified scanner representation
pub trait ScannerBackend: Send + Sync {
    fn list_devices(&self) -> Result<Vec<ScannerInfo>>;
    fn get_capabilities(&self, device_id: &str) -> Result<Capabilities>;
    fn start_scan(&self, device_id: &str, config: ScanConfig) -> Result<ScanSession>;
    fn cancel_scan(&self, session_id: u64) -> Result<()>;
}

pub struct ScannerInfo {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub model: String,
    pub connection_type: ConnectionType, // USB, Network, etc.
}

pub struct Capabilities {
    pub resolutions: Vec<u32>,
    pub color_modes: Vec<ColorMode>,
    pub sources: Vec<Source>, // Flatbed, ADF, ADF Duplex
    pub max_width_mm: f32,
    pub max_height_mm: f32,
    pub formats: Vec<ImageFormat>, // PNG, JPEG, PDF
}

pub enum ColorMode {
    Color,
    Grayscale,
    BlackAndWhite,
}

pub enum Source {
    Flatbed,
    Feeder,
    FeederDuplex,
}

pub struct ScanConfig {
    pub resolution: u32,
    pub color_mode: ColorMode,
    pub source: Source,
    pub format: ImageFormat,
    pub area: Option<ScanArea>, // Crop region
}

pub struct ScanArea {
    pub x: f32,      // mm from left
    pub y: f32,      // mm from top
    pub width: f32,  // mm
    pub height: f32, // mm
}

pub trait ScanSession {
    fn next_page(&mut self) -> Result<Option<Vec<u8>>>; // Returns image bytes
    fn is_complete(&self) -> bool;
}
```

### Backend Registry

```rust
// src/registry.rs

pub struct BackendRegistry {
    backends: Vec<Box<dyn ScannerBackend>>,
}

impl BackendRegistry {
    pub fn new() -> Self {
        let mut backends: Vec<Box<dyn ScannerBackend>> = vec![];

        // Always available
        backends.push(Box::new(EsclBackend::new()));

        // Platform-specific
        #[cfg(feature = "wia")]
        backends.push(Box::new(WiaBackend::new()));

        #[cfg(feature = "sane")]
        backends.push(Box::new(SaneBackend::new()));

        #[cfg(feature = "ica")]
        backends.push(Box::new(IcaBackend::new()));

        Self { backends }
    }

    pub fn list_all_devices(&self) -> Result<Vec<ScannerInfo>> {
        // Merge devices from all backends, deduplicate
        let mut all_devices = vec![];
        for backend in &self.backends {
            all_devices.extend(backend.list_devices()?);
        }
        Ok(dedup_scanners(all_devices))
    }
}
```

---

## FFI Design

### C Header (`include/papyr_core.h`)

```c
#ifndef PAPYR_CORE_H
#define PAPYR_CORE_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Error codes
#define PAPYR_OK 0
#define PAPYR_ERROR_INIT -1
#define PAPYR_ERROR_DEVICE_NOT_FOUND -2
#define PAPYR_ERROR_SCAN_FAILED -3
#define PAPYR_ERROR_INVALID_CONFIG -4

// Initialize library (must call first)
int32_t papyr_init(void);

// Device discovery
typedef struct {
    char id[256];
    char name[256];
    char manufacturer[128];
    char model[128];
} PapyrDeviceInfo;

typedef struct {
    PapyrDeviceInfo* devices;
    uint32_t count;
} PapyrDeviceList;

PapyrDeviceList* papyr_list_devices(void);
void papyr_free_device_list(PapyrDeviceList* list);

// Capabilities
typedef struct {
    uint32_t* resolutions;
    uint32_t resolution_count;
    uint32_t color_modes; // Bitfield: 0x1=BW, 0x2=Gray, 0x4=Color
    uint32_t sources;     // Bitfield: 0x1=Flatbed, 0x2=ADF, 0x4=Duplex
    float max_width_mm;
    float max_height_mm;
} PapyrCapabilities;

PapyrCapabilities* papyr_get_capabilities(const char* device_id);
void papyr_free_capabilities(PapyrCapabilities* caps);

// Scanning
typedef struct {
    uint32_t resolution;
    uint32_t color_mode; // 1=BW, 2=Gray, 4=Color
    uint32_t source;     // 1=Flatbed, 2=ADF, 4=Duplex
    uint32_t format;     // 1=PNG, 2=JPEG, 3=PDF
} PapyrScanConfig;

int32_t papyr_start_scan(const char* device_id, const PapyrScanConfig* config);

// Scan events (polling model for FFI simplicity)
typedef enum {
    PAPYR_EVENT_PAGE_READY = 1,
    PAPYR_EVENT_SCAN_COMPLETE = 2,
    PAPYR_EVENT_ERROR = 3,
} PapyrEventType;

typedef struct {
    PapyrEventType type;
    uint8_t* image_data; // NULL unless type=PAGE_READY
    uint32_t data_len;
    char error_message[256]; // Set if type=ERROR
} PapyrScanEvent;

PapyrScanEvent* papyr_next_event(int32_t session_id);
void papyr_free_event(PapyrScanEvent* event);

// Cleanup
void papyr_cleanup(void);

#ifdef __cplusplus
}
#endif

#endif // PAPYR_CORE_H
```

### Rust FFI Implementation (`src/ffi.rs`)

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::sync::Mutex;

static REGISTRY: Mutex<Option<BackendRegistry>> = Mutex::new(None);
static SESSIONS: Mutex<HashMap<i32, Box<dyn ScanSession>>> = Mutex::new(HashMap::new());

#[no_mangle]
pub extern "C" fn papyr_init() -> i32 {
    let mut reg = REGISTRY.lock().unwrap();
    *reg = Some(BackendRegistry::new());
    0 // PAPYR_OK
}

#[repr(C)]
pub struct CPapyrDeviceInfo {
    id: [c_char; 256],
    name: [c_char; 256],
    manufacturer: [c_char; 128],
    model: [c_char; 128],
}

#[repr(C)]
pub struct CPapyrDeviceList {
    devices: *mut CPapyrDeviceInfo,
    count: u32,
}

#[no_mangle]
pub extern "C" fn papyr_list_devices() -> *mut CPapyrDeviceList {
    let reg = REGISTRY.lock().unwrap();
    let registry = match reg.as_ref() {
        Some(r) => r,
        None => return ptr::null_mut(),
    };

    let devices = match registry.list_all_devices() {
        Ok(d) => d,
        Err(_) => return ptr::null_mut(),
    };

    // Convert Vec<ScannerInfo> to C array...
    // (Omitted for brevity - see existing ffi.rs)
}

// ... other FFI functions
```

---

## Testing Strategy

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escl_discovery() {
        let backend = EsclBackend::new();
        let devices = backend.list_devices().unwrap();
        // Requires network scanner for integration test
    }

    #[test]
    fn test_capability_parsing() {
        let xml = r#"<ScannerCapabilities>...</ScannerCapabilities>"#;
        let caps = parse_escl_capabilities(xml).unwrap();
        assert!(caps.resolutions.contains(&300));
    }
}
```

### C FFI Test (`test_c/test_ffi.c`)

```c
#include "../include/papyr_core.h"
#include <stdio.h>

int main() {
    if (papyr_init() != PAPYR_OK) {
        printf("Init failed\n");
        return 1;
    }

    PapyrDeviceList* devices = papyr_list_devices();
    printf("Found %u devices\n", devices->count);

    for (uint32_t i = 0; i < devices->count; i++) {
        printf("  %s - %s\n", devices->devices[i].name, devices->devices[i].id);
    }

    papyr_free_device_list(devices);
    papyr_cleanup();
    return 0;
}
```

### Dart Test (Integration)

```dart
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:test/test.dart';

void main() {
  test('list scanners', () {
    final lib = DynamicLibrary.open('libpapyr_core.dylib');
    final init = lib.lookupFunction<Int32 Function(), int Function()>('papyr_init');

    expect(init(), equals(0));

    // Test device listing...
  });
}
```

---

## References & Resources

### WIA (Windows)

- [WIA Driver Development Overview](https://learn.microsoft.com/en-us/windows-hardware/drivers/image/wia-driver-development)
- [WIA Intro + Concepts](https://learn.microsoft.com/en-us/windows/win32/wia/-wia-startpage)
- [WIA API Reference](https://learn.microsoft.com/en-us/windows/win32/api/_wia/)
- [IWiaDevMgr2 Interface](https://learn.microsoft.com/en-us/windows/win32/api/wia_xp/nn-wia_xp-iwiadevmgr2)
- [IWiaItem2 Interface](https://learn.microsoft.com/en-us/windows/win32/api/wia_xp/nn-wia_xp-iwiaitem2)

### TWAIN

- [Official Spec + twain.h](https://github.com/twain/twain-specification)
- [Open-source DSM (code)](https://github.com/twain/twain-dsm)
- [TWAIN Working Group](https://www.twain.org/)

### SANE

- [SANE API (frontend spec)](https://gitlab.com/sane-project/sane-backends/-/blob/master/doc/sane.tex)
- [Backend Writing Guide](https://sane-project.org/docs.html)
- [SANE Project Homepage](http://www.sane-project.org/)

### eSCL / AirScan

- [Mopria eSCL Spec (download)](https://mopria.org/spec-download)
- [sane-airscan (eSCL + WS-Scan reference impl)](https://github.com/alexpevzner/sane-airscan)
- [OpenPrinting eSCL Article](https://openprinting.github.io/achievements/#the-new-architecture-for-printing-and-scanning)
- [Apple AirPrint/AirScan Overview](https://support.apple.com/en-us/HT201311)

### macOS (ICA / ImageCaptureCore)

- [ImageCaptureCore Framework](https://developer.apple.com/documentation/imagecapturecore)
- [Archived ICA Programming Guide](https://developer.apple.com/library/archive/documentation/Carbon/Conceptual/ICA_Guide/)
- [HP AirScan Documentation](https://support.hp.com/us-en/document/ish_1841315-1659297-16)

### Rust FFI & Bindings

- [The Rustonomicon (FFI Chapter)](https://doc.rust-lang.org/nomicon/ffi.html)
- [rust-bindgen User Guide](https://rust-lang.github.io/rust-bindgen/)
- [objc2 crate (Objective-C bindings)](https://crates.io/crates/objc2)
- [windows crate](https://crates.io/crates/windows)

---

## Next Steps

1. **Complete eSCL Backend** (`src/backends/escl.rs`)
  - mDNS discovery
  - HTTP client for eSCL endpoints
  - Capability parsing
  - Multi-page scanning

2. **Test WIA Backend** (`src/backends/wia.rs`)
  - Real hardware testing on Windows
  - COM error handling
  - Memory transfer implementation

3. **FFI Polish** (`src/ffi.rs`)
  - Memory safety audit
  - Error code standardization
  - Dart FFI bindings generator

4. **Documentation**
  - API reference (rustdoc)
  - Platform-specific notes
  - Troubleshooting guide

5. **CI/CD**
  - GitHub Actions for multi-platform builds
  - Automated FFI testing
  - Release binaries (dylib/dll/so)

---

## Implementation Gotchas & Tips

### General

- **Device Deduplication**: Same physical scanner may appear via multiple backends (e.g., eSCL + WIA). Use unique ID strategy (MAC address, serial number).
- **Async/Sync**: Scanning is inherently async; use `tokio` for eSCL HTTP, but FFI boundary is synchronous. Consider event polling model.
- **Memory Management**: FFI requires manual memory management. Always pair `papyr_foo()` with `papyr_free_foo()`.

### Platform-Specific

- **Windows (WIA)**: COM requires `CoInitialize`/`CoUninitialize` per thread. Use `windows::Win32::System::Com::CoInitializeEx`.
- **macOS (ICA)**: ImageCaptureCore requires main thread for UI-related operations. For CLI use, run on background thread with run loop.
- **Linux (SANE)**: `libsane` may not be installed by default. Detect missing library gracefully, suggest `apt install libsane`.

### eSCL

- **Self-Signed Certs**: Many printers use HTTPS with self-signed certs. Use `reqwest::Client::builder().danger_accept_invalid_certs(true)`.
- **Timeout Handling**: Network scanners can be slow. Set reasonable timeouts (30s for scan start, 5min for document transfer).
- **Multi-page Logic**: Poll `/NextDocument` in loop until HTTP 404 (no more pages).

### Dart FFI

- **String Encoding**: Always use UTF-8. Dart `Utf8.toNativeUtf8()` / `toDartString()`.
- **Callbacks**: FFI doesn't support callbacks easily. Use polling model (`papyr_next_event()`) instead of C callbacks.
- **Isolates**: Dart FFI calls must be on same isolate. For async Dart, use `compute()` or send ports.

---

**Document Version**: 1.0
**Last Updated**: 2025-01-22
**Maintainer**: Papyr Core Team
