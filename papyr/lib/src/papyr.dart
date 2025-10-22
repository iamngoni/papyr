import 'dart:async';
import 'dart:ffi';
import 'dart:typed_data';
import 'package:ffi/ffi.dart';

import 'package:papyr/src/ffi_bindings.dart';
import 'package:papyr/src/models/models.dart';

/// {@template papyr}
/// Cross-platform document scanner library.
///
/// Supports scanning documents from WIA (Windows), SANE (Linux),
/// ICA (macOS), and eSCL/AirScan (network) scanners.
/// {@endtemplate}
class Papyr {
  final PapyrCoreFFI _ffi;
  bool _initialized = false;

  /// Creates a new Papyr instance.
  Papyr() : _ffi = PapyrCoreFFI.instance;

  /// Initializes the papyr library.
  ///
  /// Must be called before using other methods.
  ///
  /// Throws [PapyrException] if initialization fails.
  Future<void> initialize() async {
    if (_initialized) return;

    final result = _ffi.papyrInit();
    if (result != 0) {
      throw PapyrException('Failed to initialize papyr', result);
    }
    _initialized = true;
  }

  /// Checks if the papyr library is available on the current platform.
  ///
  /// Returns `true` if at least one scanning backend is available,
  /// `false` otherwise.
  bool get isAvailable {
    if (!_initialized) return false;
    return true; // If we initialized successfully, we have backends
  }

  /// Discovers available scanners on the system.
  ///
  /// Returns a list of [Scanner] instances representing all detected scanners.
  /// Returns an empty list if no scanners are found.
  ///
  /// Throws [StateError] if the library is not initialized.
  Future<List<Scanner>> discoverScanners() async {
    _ensureInitialized();

    final listPtr = _ffi.papyrListScanners();
    if (listPtr == nullptr) {
      return [];
    }

    final scannerList = listPtr.ref;
    final scanners = <Scanner>[];

    for (int i = 0; i < scannerList.count; i++) {
      final scannerPtr = scannerList.scanners.elementAt(i);
      final scanner = scannerPtr.ref;

      final capsPtr = _ffi.papyrGetCapabilities(scanner.id);
      final capabilities = capsPtr != nullptr
          ? _convertCapabilities(capsPtr.ref)
          : const Capabilities(
              sources: [ScanSource.flatbed],
              dpis: [300],
              colorModes: [ColorMode.color],
              supportsDuplex: false,
            );

      if (capsPtr != nullptr) {
        _ffi.papyrFreeCapabilities(capsPtr);
      }

      scanners.add(Scanner(
        id: scanner.id.toDartString(),
        name: scanner.name.toDartString(),
        backend: _convertBackend(scanner.backend),
        capabilities: capabilities,
      ));
    }

    _ffi.papyrFreeScannerList(listPtr);
    return scanners;
  }

  /// Scans a document using the specified configuration.
  ///
  /// Returns a [Stream] of [ScanEvent] that emits progress updates
  /// and the final result.
  ///
  /// The stream will emit:
  /// - [ScanStarted] when scanning begins
  /// - [ScanProgress] events during scanning (if supported)
  /// - [ScanCompleted] with the result when successful
  /// - [ScanError] if an error occurs
  /// - [ScanCancelled] if the scan is cancelled
  ///
  /// Example:
  /// ```dart
  /// final papyr = Papyr();
  /// await papyr.initialize();
  ///
  /// final scanners = await papyr.discoverScanners();
  /// final config = ScanConfig(scanner: scanners.first);
  ///
  /// await for (final event in papyr.scan(config)) {
  ///   event.when(
  ///     started: () => print('Scanning started'),
  ///     progress: (bytes, total) => print('Progress: $bytes/$total'),
  ///     completed: (result) => print('Done: ${result.filePath}'),
  ///     error: (msg, code) => print('Error: $msg'),
  ///     cancelled: () => print('Cancelled'),
  ///   );
  /// }
  /// ```
  ///
  /// Throws [StateError] if the library is not initialized.
  Stream<ScanEvent> scan(ScanConfig config) async* {
    _ensureInitialized();

    final deviceIdPtr = config.scanner.id.toNativeUtf8();
    final configPtr = malloc<PapyrScanConfig>();

    configPtr.ref.source = _scanSourceToInt(config.source);
    configPtr.ref.duplex = config.useDuplex ? 1 : 0;
    configPtr.ref.dpi = config.dpi;
    configPtr.ref.colorMode = _colorModeToInt(config.colorMode);
    configPtr.ref.pageWidthMm = 216; // Letter width
    configPtr.ref.pageHeightMm = 279; // Letter height

    final sessionId = _ffi.papyrStartScan(deviceIdPtr, configPtr);

    malloc.free(deviceIdPtr);
    malloc.free(configPtr);

    if (sessionId <= 0) {
      yield ScanEvent.error(
        message: 'Failed to start scan session',
        code: sessionId,
      );
      return;
    }

    yield const ScanEvent.started();

    // Poll for events
    int totalBytes = 0;
    final pageData = <List<int>>[];

    while (true) {
      final eventPtr = _ffi.papyrNextScanEvent(sessionId);
      if (eventPtr == nullptr) {
        break;
      }

      final event = eventPtr.ref;

      switch (event.eventType) {
        case PapyrScanEventType.pageStarted:
          // Page started, continue
          break;

        case PapyrScanEventType.pageData:
          final data = event.data.cast<Uint8>().asTypedList(event.dataSize);
          pageData.add(List<int>.from(data));
          totalBytes += event.dataSize;
          yield ScanEvent.progress(bytesScanned: totalBytes);
          break;

        case PapyrScanEventType.pageComplete:
          // Page completed, continue to next page or job complete
          break;

        case PapyrScanEventType.jobComplete:
          // Flatten all page data
          final allData = pageData.expand((page) => page).toList();

          // Save to file if output path provided
          String? filePath;
          if (config.outputPath != null) {
            // TODO: Implement file saving based on format
            filePath = config.outputPath;
          }

          yield ScanEvent.completed(
            result: ScanResult(
              data: config.outputPath == null ? allData : null,
              filePath: filePath,
              pageCount: 1, // TODO: Track actual page count
              totalBytes: totalBytes,
            ),
          );
          _ffi.papyrFreeScanEvent(eventPtr);
          return;
      }

      _ffi.papyrFreeScanEvent(eventPtr);
    }

    yield ScanEvent.error(
      message: 'Scan session ended unexpectedly',
      code: -1,
    );
  }

  /// Cleans up the papyr library and releases resources.
  Future<void> dispose() async {
    if (!_initialized) return;

    _ffi.papyrCleanup();
    _initialized = false;
  }

  void _ensureInitialized() {
    if (!_initialized) {
      throw StateError('Papyr not initialized. Call initialize() first.');
    }
  }

  // Conversion helpers

  ScannerBackend _convertBackend(int backend) {
    switch (backend) {
      case PapyrBackend.wia:
        return ScannerBackend.wia;
      case PapyrBackend.sane:
        return ScannerBackend.sane;
      case PapyrBackend.ica:
        return ScannerBackend.ica;
      case 3: // eSCL
        return ScannerBackend.escl;
      default:
        return ScannerBackend.unknown;
    }
  }

  Capabilities _convertCapabilities(PapyrCapabilities caps) {
    final sources = <ScanSource>[];
    for (int i = 0; i < caps.sourcesCount; i++) {
      sources.add(_convertScanSource(caps.sources.elementAt(i).value));
    }

    final dpis = <int>[];
    for (int i = 0; i < caps.dpisCount; i++) {
      dpis.add(caps.dpis.elementAt(i).value);
    }

    final colorModes = <ColorMode>[];
    for (int i = 0; i < caps.colorModesCount; i++) {
      colorModes.add(_convertColorMode(caps.colorModes.elementAt(i).value));
    }

    return Capabilities(
      sources: sources,
      dpis: dpis,
      colorModes: colorModes,
      supportsDuplex: caps.supportsDuplex != 0,
    );
  }

  ScanSource _convertScanSource(int source) {
    switch (source) {
      case PapyrScanSource.flatbed:
        return ScanSource.flatbed;
      case PapyrScanSource.adf:
        return ScanSource.adf;
      case PapyrScanSource.adfDuplex:
        return ScanSource.adfDuplex;
      default:
        return ScanSource.flatbed;
    }
  }

  ColorMode _convertColorMode(int mode) {
    switch (mode) {
      case PapyrColorMode.color:
        return ColorMode.color;
      case PapyrColorMode.gray:
        return ColorMode.grayscale;
      case PapyrColorMode.bw:
        return ColorMode.blackAndWhite;
      default:
        return ColorMode.color;
    }
  }

  int _scanSourceToInt(ScanSource source) {
    switch (source) {
      case ScanSource.flatbed:
        return PapyrScanSource.flatbed;
      case ScanSource.adf:
        return PapyrScanSource.adf;
      case ScanSource.adfDuplex:
        return PapyrScanSource.adfDuplex;
    }
  }

  int _colorModeToInt(ColorMode mode) {
    switch (mode) {
      case ColorMode.color:
        return PapyrColorMode.color;
      case ColorMode.grayscale:
        return PapyrColorMode.gray;
      case ColorMode.blackAndWhite:
        return PapyrColorMode.bw;
    }
  }
}

/// Exception thrown by papyr operations.
class PapyrException implements Exception {
  /// The error message.
  final String message;

  /// The error code from the underlying library.
  final int code;

  /// Creates a new papyr exception.
  const PapyrException(this.message, this.code);

  @override
  String toString() => 'PapyrException($code): $message';
}
