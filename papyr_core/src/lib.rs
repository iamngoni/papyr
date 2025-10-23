#![allow(static_mut_refs)]
#![allow(dead_code)]
#![allow(clippy::new_without_default)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::manual_unwrap_or_default)]
#![allow(clippy::type_complexity)]
#![allow(deprecated)]
//
//  papyr_core
//  lib.rs
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

pub mod backends;
pub mod ffi;
pub mod models;
pub mod registry;

pub use models::*;
