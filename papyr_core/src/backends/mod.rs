//
//  papyr_core
//  backends/mod.rs
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

// eSCL is always available (cross-platform network scanning)
pub mod escl;

// TWAIN is available on Windows and macOS
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub mod twain;

// WIA is available by default on Windows (not behind feature flag)
#[cfg(target_os = "windows")]
pub mod wia;

#[cfg(target_os = "macos")]
pub mod ica;

#[cfg(feature = "sane")]
pub mod sane;
