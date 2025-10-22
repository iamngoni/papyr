import 'package:papyr/src/models/models.dart';

/// Extensions on [Scanner] for convenience methods.
extension ScannerX on Scanner {
  /// Whether this scanner supports color scanning.
  bool get supportsColor => capabilities.colorModes.contains(ColorMode.color);

  /// Whether this scanner supports grayscale scanning.
  bool get supportsGrayscale =>
      capabilities.colorModes.contains(ColorMode.grayscale);

  /// Whether this scanner supports black and white scanning.
  bool get supportsBlackAndWhite =>
      capabilities.colorModes.contains(ColorMode.blackAndWhite);

  /// Whether this scanner has an Automatic Document Feeder (ADF).
  bool get hasAdf =>
      capabilities.sources.contains(ScanSource.adf) ||
      capabilities.sources.contains(ScanSource.adfDuplex);

  /// Whether this scanner supports duplex scanning.
  bool get supportsDuplex => capabilities.supportsDuplex;

  /// Whether this scanner has a flatbed.
  bool get hasFlatbed => capabilities.sources.contains(ScanSource.flatbed);

  /// The maximum DPI supported by this scanner.
  int get maxDpi =>
      capabilities.dpis.isEmpty ? 0 : capabilities.dpis.reduce((a, b) => a > b ? a : b);

  /// The minimum DPI supported by this scanner.
  int get minDpi =>
      capabilities.dpis.isEmpty ? 0 : capabilities.dpis.reduce((a, b) => a < b ? a : b);

  /// Whether this scanner is a network scanner (eSCL/AirScan).
  bool get isNetworkScanner => backend == ScannerBackend.escl;

  /// Whether this scanner is a local scanner (WIA, SANE, or ICA).
  bool get isLocalScanner =>
      backend == ScannerBackend.wia ||
      backend == ScannerBackend.sane ||
      backend == ScannerBackend.ica;
}

/// Extensions on [ScanConfig] for convenience methods.
extension ScanConfigX on ScanConfig {
  /// Creates a copy of this config with modified values.
  ScanConfig copyWith({
    Scanner? scanner,
    ScanSource? source,
    int? dpi,
    ColorMode? colorMode,
    ScanFormat? format,
    String? outputPath,
    bool? useDuplex,
  }) {
    return ScanConfig(
      scanner: scanner ?? this.scanner,
      source: source ?? this.source,
      dpi: dpi ?? this.dpi,
      colorMode: colorMode ?? this.colorMode,
      format: format ?? this.format,
      outputPath: outputPath ?? this.outputPath,
      useDuplex: useDuplex ?? this.useDuplex,
    );
  }

  /// Validates this configuration against the scanner's capabilities.
  ///
  /// Returns `null` if valid, or an error message if invalid.
  String? validate() {
    if (!scanner.capabilities.sources.contains(source)) {
      return 'Scanner does not support source: $source';
    }

    if (!scanner.capabilities.dpis.contains(dpi)) {
      return 'Scanner does not support DPI: $dpi';
    }

    if (!scanner.capabilities.colorModes.contains(colorMode)) {
      return 'Scanner does not support color mode: $colorMode';
    }

    if (useDuplex && !scanner.supportsDuplex) {
      return 'Scanner does not support duplex scanning';
    }

    return null;
  }

  /// Whether this configuration is valid for the scanner.
  bool get isValid => validate() == null;
}
