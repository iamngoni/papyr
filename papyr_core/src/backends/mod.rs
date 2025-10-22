//
//  papyr_core
//  backends/mod.rs
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

// eSCL is always available (cross-platform network scanning)
pub mod escl;

#[cfg(feature = "wia")]
pub mod wia;

#[cfg(feature = "ica")]
pub mod ica;

#[cfg(feature = "sane")]
pub mod sane;
