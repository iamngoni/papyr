//
//  papyr_core
//  bin/test_scanner.rs - Test binary to verify scanner backends
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use papyr_core::models::{ColorMode, PageSize, ScanConfig, ScanSource};
use papyr_core::registry::BackendRegistry;

#[tokio::main]
async fn main() {
    println!("🔍 Papyr Core - Scanner Backend Test\n");

    // Initialize registry (automatically loads all backends)
    let registry = BackendRegistry::new();

    // List all available scanners
    println!("📡 Discovering scanners...\n");
    match registry.list_devices() {
        Ok(scanners) => {
            if scanners.is_empty() {
                println!("❌ No scanners found");
                println!("\nTroubleshooting:");
                println!("  - Ensure scanner is turned on and connected");
                println!("  - For network scanners, ensure they're on the same network");
                println!("  - For USB scanners, check USB connection");
                return;
            }

            println!("✅ Found {} scanner(s):\n", scanners.len());
            for (i, scanner) in scanners.iter().enumerate() {
                println!("  {}. {} ({})", i + 1, scanner.name, scanner.id);
                println!("     Backend: {:?}", scanner.backend);

                // Get capabilities
                match registry.capabilities(&scanner.id) {
                    Ok(caps) => {
                        println!("     Capabilities:");
                        println!("       - Sources: {:?}", caps.sources);
                        println!("       - DPIs: {:?}", caps.dpis);
                        println!("       - Color modes: {:?}", caps.color_modes);
                        println!("       - Duplex: {}", caps.supports_duplex);
                    }
                    Err(e) => {
                        println!("     ⚠️  Failed to get capabilities: {:?}", e);
                    }
                }
                println!();
            }

            // Test scan with first scanner
            if let Some(first_scanner) = scanners.first() {
                println!("\n🖨️  Testing scan with: {}\n", first_scanner.name);

                let config = ScanConfig {
                    source: ScanSource::Flatbed,
                    duplex: false,
                    dpi: 150,
                    color_mode: ColorMode::Color,
                    page_size: PageSize {
                        width_mm: 216,
                        height_mm: 279,
                    },
                    area: None,
                    brightness: None,
                    contrast: None,
                    max_pages: Some(1),
                };

                println!("Configuration:");
                println!("  - Source: {:?}", config.source);
                println!("  - DPI: {}", config.dpi);
                println!("  - Color: {:?}", config.color_mode);
                println!(
                    "  - Size: {}x{}mm\n",
                    config.page_size.width_mm, config.page_size.height_mm
                );

                match registry.start_scan(&first_scanner.id, config) {
                    Ok(mut session) => {
                        println!("✅ Scan session started\n");
                        println!("📄 Waiting for scan events...\n");

                        let mut page_count = 0;
                        let mut total_data_size = 0;

                        loop {
                            match session.next_event() {
                                Ok(Some(event)) => {
                                    use papyr_core::models::ScanEvent;
                                    match event {
                                        ScanEvent::PageStarted(index) => {
                                            println!("  📄 Page {} started", index);
                                        }
                                        ScanEvent::PageData(data) => {
                                            if !data.is_empty() {
                                                println!("  📦 Received {} bytes", data.len());
                                                total_data_size += data.len();
                                            }
                                        }
                                        ScanEvent::PageComplete(meta) => {
                                            page_count += 1;
                                            println!("  ✅ Page {} complete", meta.index);
                                            println!(
                                                "     Size: {}x{} pixels",
                                                meta.width_px, meta.height_px
                                            );
                                            println!("     DPI: {}", meta.dpi);
                                            println!("     Color: {:?}", meta.color_mode);
                                        }
                                        ScanEvent::JobComplete => {
                                            println!("\n✅ Scan job complete!");
                                            println!("   Pages scanned: {}", page_count);
                                            println!("   Total data: {} bytes", total_data_size);
                                            break;
                                        }
                                    }
                                }
                                Ok(None) => {
                                    println!("\n✅ Scan session finished");
                                    println!("   Pages scanned: {}", page_count);
                                    println!("   Total data: {} bytes", total_data_size);
                                    break;
                                }
                                Err(e) => {
                                    println!("\n❌ Scan error: {:?}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to start scan: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to list devices: {:?}", e);
        }
    }

    println!("\n✨ Test complete");
}
