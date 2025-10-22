//
//  papyr_core
//  test_c/test_ffi.c - C FFI test
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

#include <stdio.h>
#include <stdlib.h>
#include "../include/papyr_core.h"

int main() {
    printf("Testing papyr_core C FFI...\n");
    
    // Initialize the library
    int init_result = papyr_init();
    if (init_result != 0) {
        printf("Failed to initialize papyr_core: %d\n", init_result);
        return 1;
    }
    printf("✓ Library initialized successfully\n");
    
    // List available scanners
    PapyrScannerInfoList* scanners = papyr_list_scanners();
    if (scanners == NULL) {
        printf("Failed to get scanner list\n");
        papyr_cleanup();
        return 1;
    }
    
    printf("✓ Found %zu scanner(s):\n", scanners->count);
    
    for (size_t i = 0; i < scanners->count; i++) {
        PapyrScannerInfo* scanner = &scanners->scanners[i];
        printf("  - %s (%s): backend %d\n", scanner->name, scanner->id, scanner->backend);
        
        // Test capabilities for this scanner
        PapyrCapabilities* caps = papyr_get_capabilities(scanner->id);
        if (caps != NULL) {
            printf("    ✓ Capabilities retrieved\n");
            printf("      Sources: %zu available\n", caps->sources_count);
            printf("      DPIs: %zu available\n", caps->dpis_count);
            printf("      Color modes: %zu available\n", caps->color_modes_count);
            printf("      Duplex: %s\n", caps->supports_duplex ? "yes" : "no");
            
            papyr_free_capabilities(caps);
        } else {
            printf("    ✗ Failed to get capabilities\n");
        }
        
        // Test scan session creation
        PapyrScanConfig config = {
            .source = SOURCE_FLATBED,
            .duplex = 0,  // false
            .dpi = 300,
            .color_mode = COLOR_MODE_COLOR,
            .page_width_mm = 216,
            .page_height_mm = 279
        };
        
        int session_id = papyr_start_scan(scanner->id, &config);
        if (session_id > 0) {
            printf("    ✓ Scan session created (ID: %d)\n", session_id);
            
            // Get scan events
            PapyrScanEvent* event = papyr_next_scan_event(session_id);
            if (event != NULL) {
                printf("    ✓ Received scan event (type: %d)\n", event->event_type);
                papyr_free_scan_event(event);
            }
        } else {
            printf("    ✗ Failed to create scan session: %d\n", session_id);
        }
    }
    
    // Cleanup
    papyr_free_scanner_list(scanners);
    papyr_cleanup();
    
    printf("✓ C FFI test completed successfully\n");
    return 0;
}
