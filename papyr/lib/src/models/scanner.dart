//
//  papyr
//  scanner.dart
//
//  Created by Ngonidzashe Mangudya on 23/01/2025.
//  Copyright (c) 2025 Codecraft Solutions
//

import 'package:freezed_annotation/freezed_annotation.dart';

part 'scanner.freezed.dart';

/// Represents a discovered scanner device
///
/// Contains identifying information and capabilities
/// for a physical or network scanner.
@freezed
class Scanner with _$Scanner {
  const factory Scanner({
    /// Unique identifier for this scanner
    required String id,
    
    /// Human-readable name of the scanner
    required String name,
    
    /// Backend protocol used to communicate with this scanner
    required ScannerBackend backend,
    
    /// Scanner capabilities (resolution, color modes, etc)
    required Capabilities capabilities,
  }) = _Scanner;
}

/// Scanner backend/protocol types
enum ScannerBackend {
  /// Windows Image Acquisition (Windows)
  wia,
  
  /// Scanner Access Now Easy (Linux/BSD)
  sane,
  
  /// Image Capture Architecture (macOS)
  ica,
  
  /// eSCL/AirScan (Network scanners, all platforms)
  escl,
  
  /// Unknown/unsupported backend
  unknown,
}

/// Scanner capabilities
///
/// Describes what a scanner can do (supported resolutions,
/// color modes, paper sources, etc).
@freezed
class Capabilities with _$Capabilities {
  const factory Capabilities({
    /// Available scan sources (flatbed, ADF, etc)
    required List<ScanSource> sources,
    
    /// Supported DPI/resolution values
    required List<int> dpis,
    
    /// Supported color modes
    required List<ColorMode> colorModes,
    
    /// Whether the scanner supports duplex (two-sided) scanning
    required bool supportsDuplex,
  }) = _Capabilities;
}

/// Scan source/input types
enum ScanSource {
  /// Flatbed scanner (place document on glass)
  flatbed,
  
  /// Automatic Document Feeder (feed multiple pages)
  adf,
  
  /// ADF with duplex (scan both sides automatically)
  adfDuplex,
}

/// Color modes for scanning
enum ColorMode {
  /// Full color (RGB)
  color,
  
  /// Grayscale (black and white shades)
  grayscale,
  
  /// Black and white only (no shades)
  blackAndWhite,
}
