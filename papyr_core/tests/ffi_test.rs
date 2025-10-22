//
//  papyr_core
//  tests/ffi_test.rs - FFI layer integration tests
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use std::ffi::CString;
use std::ptr;

// Import the FFI functions
extern "C" {
    fn papyr_init() -> i32;
    fn papyr_cleanup();
    fn papyr_list_scanners() -> *mut papyr_core::ffi::CScannerInfoList;
    fn papyr_free_scanner_list(list: *mut papyr_core::ffi::CScannerInfoList);
    fn papyr_get_capabilities(device_id: *const i8) -> *mut papyr_core::ffi::CCapabilities;
    fn papyr_free_capabilities(caps: *mut papyr_core::ffi::CCapabilities);
    fn papyr_start_scan(device_id: *const i8, config: *const papyr_core::ffi::CScanConfig) -> i32;
    fn papyr_next_scan_event(session_id: i32) -> *mut papyr_core::ffi::CScanEvent;
    fn papyr_free_scan_event(event: *mut papyr_core::ffi::CScanEvent);
}

#[test]
fn test_ffi_init_and_cleanup() {
    unsafe {
        let result = papyr_init();
        assert_eq!(result, 0, "papyr_init should return 0 on success");
        papyr_cleanup();
    }
}

#[test]
fn test_ffi_list_scanners() {
    unsafe {
        papyr_init();

        let scanners = papyr_list_scanners();
        assert!(
            !scanners.is_null(),
            "papyr_list_scanners should not return NULL"
        );

        let list = &*scanners;
        println!("Found {} scanners via FFI", list.count);

        // Even if no scanners are found, the function should return a valid (empty) list
        // Clean up
        papyr_free_scanner_list(scanners);
        papyr_cleanup();
    }
}

#[test]
fn test_ffi_double_init() {
    unsafe {
        let result1 = papyr_init();
        assert_eq!(result1, 0);

        let result2 = papyr_init();
        assert_eq!(result2, 0, "Multiple init calls should be safe");

        papyr_cleanup();
    }
}

#[test]
fn test_ffi_cleanup_without_init() {
    unsafe {
        // This should not crash
        papyr_cleanup();
    }
}

#[test]
fn test_ffi_get_capabilities_invalid_device() {
    unsafe {
        papyr_init();

        let device_id = CString::new("invalid_device_id_12345").unwrap();
        let caps = papyr_get_capabilities(device_id.as_ptr());

        // Should return NULL for invalid device
        assert!(caps.is_null(), "Should return NULL for invalid device ID");

        papyr_cleanup();
    }
}

#[test]
fn test_ffi_start_scan_invalid_device() {
    unsafe {
        papyr_init();

        let device_id = CString::new("invalid_device_id_12345").unwrap();
        let config = papyr_core::ffi::CScanConfig {
            source: 0, // Flatbed
            duplex: 0, // false
            dpi: 300,
            color_mode: 0, // Color
            page_width_mm: 216,
            page_height_mm: 279,
        };

        let session_id = papyr_start_scan(device_id.as_ptr(), &config);

        // Should return negative value (error) for invalid device
        assert!(session_id < 0, "Should return error for invalid device ID");

        papyr_cleanup();
    }
}

#[test]
fn test_ffi_scan_event_invalid_session() {
    unsafe {
        papyr_init();

        let event = papyr_next_scan_event(99999); // Invalid session ID

        // Should return NULL for invalid session
        assert!(event.is_null(), "Should return NULL for invalid session ID");

        papyr_cleanup();
    }
}

#[test]
fn test_ffi_free_null_pointers() {
    unsafe {
        // These should not crash when passed NULL
        papyr_free_scanner_list(ptr::null_mut());
        papyr_free_capabilities(ptr::null_mut());
        papyr_free_scan_event(ptr::null_mut());
    }
}

#[test]
fn test_ffi_memory_safety() {
    unsafe {
        papyr_init();

        // Get scanner list
        let scanners = papyr_list_scanners();
        if !scanners.is_null() {
            let list = &*scanners;

            // If we have scanners, test capabilities
            if list.count > 0 {
                let first_scanner = &list.scanners.read();
                if !first_scanner.id.is_null() {
                    let caps = papyr_get_capabilities(first_scanner.id);
                    if !caps.is_null() {
                        // Verify capabilities structure is valid
                        let capabilities = &*caps;
                        assert!(capabilities.sources_count >= 0);
                        assert!(capabilities.dpis_count >= 0);
                        assert!(capabilities.color_modes_count >= 0);

                        papyr_free_capabilities(caps);
                    }
                }
            }

            papyr_free_scanner_list(scanners);
        }

        papyr_cleanup();
    }
}

#[cfg(test)]
mod integration {
    use super::*;

    /// Full workflow test: init -> list -> capabilities -> scan -> cleanup
    #[test]
    fn test_ffi_full_workflow() {
        unsafe {
            // 1. Initialize
            let result = papyr_init();
            assert_eq!(result, 0);

            // 2. List scanners
            let scanners = papyr_list_scanners();
            assert!(!scanners.is_null());

            let list = &*scanners;
            println!("Full workflow test: Found {} scanners", list.count);

            if list.count > 0 {
                // 3. Get first scanner
                let first_scanner = &list.scanners.read();
                println!(
                    "Testing scanner: {:?}",
                    std::ffi::CStr::from_ptr(first_scanner.name)
                );

                // 4. Get capabilities
                if !first_scanner.id.is_null() {
                    let caps = papyr_get_capabilities(first_scanner.id);
                    if !caps.is_null() {
                        let capabilities = &*caps;
                        println!("  Sources: {}", capabilities.sources_count);
                        println!("  DPIs: {}", capabilities.dpis_count);
                        println!("  Color modes: {}", capabilities.color_modes_count);

                        papyr_free_capabilities(caps);
                    }

                    // 5. Attempt to start scan
                    let config = papyr_core::ffi::CScanConfig {
                        source: 0,
                        duplex: 0,
                        dpi: 150,
                        color_mode: 0,
                        page_width_mm: 216,
                        page_height_mm: 279,
                    };

                    let session_id = papyr_start_scan(first_scanner.id, &config);
                    if session_id > 0 {
                        println!("  Scan session created: {}", session_id);

                        // 6. Get events (until completion or error)
                        loop {
                            let event = papyr_next_scan_event(session_id);
                            if event.is_null() {
                                break;
                            }

                            let scan_event = &*event;
                            println!("  Event type: {}", scan_event.event_type);

                            papyr_free_scan_event(event);

                            // Break on job complete (type 3)
                            if scan_event.event_type == 3 {
                                break;
                            }
                        }
                    } else {
                        println!("  Scan session failed (expected without hardware)");
                    }
                }
            } else {
                println!("  No scanners found (expected without hardware)");
            }

            // 7. Cleanup
            papyr_free_scanner_list(scanners);
            papyr_cleanup();
        }
    }
}
