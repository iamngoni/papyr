import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:papyr/src/models/scanner.dart';

part 'scan_config.freezed.dart';

/// Configuration for a scan operation.
@freezed
class ScanConfig with _$ScanConfig {
  const factory ScanConfig({
    /// The scanner to use for scanning.
    required Scanner scanner,

    /// The source to scan from.
    @Default(ScanSource.flatbed) ScanSource source,

    /// The resolution in DPI.
    @Default(300) int dpi,

    /// The color mode to use.
    @Default(ColorMode.color) ColorMode colorMode,

    /// The file format for the scanned document.
    @Default(ScanFormat.pdf) ScanFormat format,

    /// The output file path (optional, if null returns bytes).
    String? outputPath,

    /// Whether to use duplex scanning (both sides).
    @Default(false) bool useDuplex,
  }) = _ScanConfig;
}

/// Supported output formats for scanned documents.
enum ScanFormat {
  /// Portable Document Format.
  pdf,

  /// Joint Photographic Experts Group.
  jpeg,

  /// Portable Network Graphics.
  png,

  /// Tagged Image File Format.
  tiff,
}
