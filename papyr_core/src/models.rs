//
//  papyr_core
//  models.rs
//
//  Created by Ngonidzashe Mangudya on 2025/10/22.
//  Copyright (c) 2025 Codecraft Solutions. All rights reserved.
//

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Backend {
    Wia,  // Windows Image Acquisition
    Sane, // Scanner Access Now Easy (Linux)
    Ica,  // Image Capture Architecture (macOS)
    Escl, // eSCL/AirScan (network scanners, cross-platform)
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScanSource {
    Flatbed,
    Adf,
    AdfDuplex,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorMode {
    Color,
    Gray,
    Bw,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageSize {
    pub width_mm: u32,
    pub height_mm: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerInfo {
    pub id: String,
    pub name: String,
    pub backend: Backend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub sources: Vec<ScanSource>,
    pub dpis: Vec<u32>,
    pub color_modes: Vec<ColorMode>,
    pub page_sizes: Vec<PageSize>,
    pub supports_duplex: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScanArea {
    pub x_mm: u32,
    pub y_mm: u32,
    pub width_mm: u32,
    pub height_mm: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMeta {
    pub index: u32,
    pub width_px: u32,
    pub height_px: u32,
    pub dpi: u32,
    pub color_mode: ColorMode,
}

#[derive(Debug)]
pub enum ScanEvent {
    PageStarted(u32),
    PageData(Vec<u8>),
    PageComplete(PageMeta),
    JobComplete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub source: ScanSource,
    pub duplex: bool,
    pub dpi: u32,
    pub color_mode: ColorMode,
    pub page_size: PageSize,
    pub area: Option<ScanArea>,
    pub brightness: Option<i32>, // device-specific range
    pub contrast: Option<i32>,   // device-specific range
    /// Optional safety: stop after N pages even if feeder keeps going.
    pub max_pages: Option<u32>,
}

#[derive(Debug, Error)]
pub enum PapyrError {
    #[error("scanner not found: {0}")]
    NotFound(String),

    #[error("invalid config: {0}")]
    InvalidConfig(String),

    #[error("backend error: {0}")]
    Backend(String),

    #[error("not implemented")]
    NotImplemented,

    #[error("other: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, PapyrError>;

#[cfg(windows)]
impl From<windows::core::Error> for PapyrError {
    fn from(err: windows::core::Error) -> Self {
        PapyrError::Backend(format!("Windows error: {}", err))
    }
}

pub trait BackendProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn kind(&self) -> Backend;

    fn enumerate(&self) -> Vec<ScannerInfo>;

    fn capabilities(&self, device_id: &str) -> Result<Capabilities>;

    fn start_scan(&self, device_id: &str, cfg: ScanConfig) -> Result<Box<dyn ScanSession>>;
}

pub trait ScanSession: Send {
    /// Returns next event, or Ok(None) when finished.
    fn next_event(&mut self) -> Result<Option<ScanEvent>>;
}
