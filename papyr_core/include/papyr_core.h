//
//  papyr_core
//  include/papyr_core.h - C header for FFI interface
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

#ifndef PAPYR_CORE_H
#define PAPYR_CORE_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>

// Backend types
typedef enum {
    BACKEND_WIA = 0,      // Windows Image Acquisition
    BACKEND_SANE = 1,     // Scanner Access Now Easy (Linux)
    BACKEND_ICA = 2,      // Image Capture Architecture (macOS)
    BACKEND_UNKNOWN = 99
} PapyrBackend;

// Scan source types
typedef enum {
    SOURCE_FLATBED = 0,
    SOURCE_ADF = 1,
    SOURCE_ADF_DUPLEX = 2
} PapyrScanSource;

// Color mode types
typedef enum {
    COLOR_MODE_COLOR = 0,
    COLOR_MODE_GRAY = 1,
    COLOR_MODE_BW = 2
} PapyrColorMode;

// Scan event types
typedef enum {
    SCAN_EVENT_PAGE_STARTED = 0,
    SCAN_EVENT_PAGE_DATA = 1,
    SCAN_EVENT_PAGE_COMPLETE = 2,
    SCAN_EVENT_JOB_COMPLETE = 3
} PapyrScanEventType;

// Structures
typedef struct {
    char* id;
    char* name;
    int backend;
} PapyrScannerInfo;

typedef struct {
    PapyrScannerInfo* scanners;
    size_t count;
} PapyrScannerInfoList;

typedef struct {
    int* sources;
    size_t sources_count;
    int* dpis;
    size_t dpis_count;
    int* color_modes;
    size_t color_modes_count;
    int supports_duplex; // 0 = false, 1 = true
} PapyrCapabilities;

typedef struct {
    int source;
    int duplex;            // 0 = false, 1 = true
    int dpi;
    int color_mode;
    int page_width_mm;
    int page_height_mm;
} PapyrScanConfig;

typedef struct {
    int event_type;
    void* data;
    size_t data_size;
} PapyrScanEvent;

// Function declarations

/**
 * Initialize the papyr core library.
 * Must be called before any other papyr functions.
 * @return 0 on success, negative on error
 */
int papyr_init(void);

/**
 * Get list of available scanners.
 * @return Pointer to scanner list, or NULL on error.
 *         Must be freed with papyr_free_scanner_list()
 */
PapyrScannerInfoList* papyr_list_scanners(void);

/**
 * Get capabilities of a specific scanner.
 * @param device_id Scanner device ID
 * @return Pointer to capabilities, or NULL on error.
 *         Must be freed with papyr_free_capabilities()
 */
PapyrCapabilities* papyr_get_capabilities(const char* device_id);

/**
 * Start a scan session.
 * @param device_id Scanner device ID
 * @param config Scan configuration
 * @return Session ID (positive integer) on success, negative on error
 */
int papyr_start_scan(const char* device_id, const PapyrScanConfig* config);

/**
 * Get next scan event from a session.
 * @param session_id Session ID from papyr_start_scan()
 * @return Pointer to scan event, or NULL when session is complete or on error.
 *         Must be freed with papyr_free_scan_event()
 */
PapyrScanEvent* papyr_next_scan_event(int session_id);

/**
 * Free scanner list memory.
 * @param list Scanner list to free
 */
void papyr_free_scanner_list(PapyrScannerInfoList* list);

/**
 * Free capabilities memory.
 * @param caps Capabilities to free
 */
void papyr_free_capabilities(PapyrCapabilities* caps);

/**
 * Free scan event memory.
 * @param event Scan event to free
 */
void papyr_free_scan_event(PapyrScanEvent* event);

/**
 * Cleanup the papyr core library.
 * Should be called when done using the library.
 */
void papyr_cleanup(void);

#ifdef __cplusplus
}
#endif

#endif // PAPYR_CORE_H
