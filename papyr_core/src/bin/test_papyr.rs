//
//  papyr_core
//  bin/test_papyr.rs - Test executable
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use papyr_core::*;
use papyr_core::registry::BackendRegistry;

#[cfg(feature = "wia")]
use papyr_core::backends::wia::WiaBackend;

#[cfg(feature = "ica")]
use papyr_core::backends::ica::IcaBackend;

#[tokio::main]
async fn main() {
    println!("Testing papyr_core...");
    
    let mut registry = BackendRegistry::new();
    
    // Register WIA backend if available
    #[cfg(feature = "wia")]
    {
        match WiaBackend::new() {
            Ok(wia_backend) => {
                println!("✓ WIA Backend initialized successfully");
                registry.register(Box::new(wia_backend));
            }
            Err(e) => {
                println!("✗ Failed to initialize WIA Backend: {:?}", e);
            }
        }
    }

    // Register ICA backend if available
    #[cfg(feature = "ica")]
    let ica_backend = {
        match IcaBackend::new() {
            Ok(ica_backend) => {
                println!("✓ ICA Backend initialized successfully");
                Some(ica_backend)
            }
            Err(e) => {
                println!("✗ Failed to initialize ICA Backend: {:?}", e);
                None
            }
        }
    };

    // Test device enumeration with async ICA backend
    #[cfg(feature = "ica")]
    if let Some(ica_backend) = ica_backend {
        // Test async enumeration first
        match ica_backend.enumerate_devices_async().await {
            Ok(devices) => {
                println!("✓ Async enumeration found {} scanner(s):", devices.len());
                
                // Now register the backend and test it
                registry.register(Box::new(ica_backend));
                
                for (i, device) in devices.iter().enumerate() {
                    println!("  - {} ({}): {:?}", device.name, device.id, device.backend);
                    
                    // Test capabilities
                    match registry.capabilities(&device.id) {
                        Ok(caps) => {
                            println!("    Sources: {:?}", caps.sources);
                            println!("    DPIs: {:?}", caps.dpis);
                            println!("    Color modes: {:?}", caps.color_modes);
                            println!("    Duplex: {}", caps.supports_duplex);
                        }
                        Err(e) => {
                            println!("    ✗ Failed to get capabilities: {:?}", e);
                        }
                    }
                    
                    // Test scan configuration (only for the first device to avoid hanging)
                    if i == 0 {
                        println!("    🔧 Testing scan session creation...");
                        test_scan_session(&mut registry, &device.id);
                    }
                }
            }
            Err(e) => {
                println!("✗ Failed to enumerate devices: {:?}", e);
            }
        }
    }
    
    // Check final registry state
    match registry.list_devices() {
        Ok(devices) => {
            if devices.is_empty() {
                println!("ℹ️ No devices found in final registry");
            } else {
                println!("ℹ️ Final registry contains {} device(s)", devices.len());
            }
        }
        Err(e) => {
            println!("⚠️ Error checking final registry: {:?}", e);
        }
    }
    
    println!("Test completed.");
}

fn test_scan_session(registry: &mut BackendRegistry, device_id: &str) {
    let config = ScanConfig {
        source: ScanSource::Flatbed,
        duplex: false,
        dpi: 300,
        color_mode: ColorMode::Color,
        page_size: PageSize { width_mm: 216, height_mm: 279 },
        area: None,
        brightness: None,
        contrast: None,
        max_pages: Some(1),
    };
    
    // Test scan session creation (but don't actually scan to avoid hanging)
    match registry.start_scan(device_id, config) {
        Ok(mut session) => {
            println!("    ✓ Scan session created successfully");
            
            // Just try to get the first event, don't loop indefinitely
            match session.next_event() {
                Ok(Some(event)) => {
                    println!("    Scan event: {:?}", event);
                }
                Ok(None) => {
                    println!("    No scan events available");
                }
                Err(e) => {
                    println!("    ⚠️  Scan event error (expected): {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("    ✗ Failed to start scan: {:?}", e);
        }
    }
}
