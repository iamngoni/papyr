import 'dart:ffi';
import 'dart:typed_data';
import 'ffi_bindings.dart';

/// Scanner backend types
enum ScannerBackend {
  wia,    // Windows Image Acquisition
  sane,   // Scanner Access Now Easy (Linux)
  ica,    // Image Capture Architecture (macOS)
  unknown,
}

/// Scan source types
enum ScanSource {
  flatbed,
  adf,
  adfDuplex,
}

/// Color modes
enum ColorMode {
  color,
  grayscale,
  blackAndWhite,
}

/// Scanner information
class ScannerInfo {
  final String id;
  final String name;
  final ScannerBackend backend;

  const ScannerInfo({
    required this.id,
    required this.name,
    required this.backend,
  });

  @override
  String toString() => 'ScannerInfo($id: $name, $backend)';
}

/// Scanner capabilities
class ScannerCapabilities {
  final List<ScanSource> sources;
  final List<int> dpis;
  final List<ColorMode> colorModes;
  final bool supportsDuplex;

  const ScannerCapabilities({
    required this.sources,
    required this.dpis,
    required this.colorModes,
    required this.supportsDuplex,
  });

  @override
  String toString() => 
    'ScannerCapabilities(sources: $sources, dpis: $dpis, colors: $colorModes, duplex: $supportsDuplex)';
}

/// Scan configuration
class ScanConfig {
  final ScanSource source;
  final bool duplex;
  final int dpi;
  final ColorMode colorMode;
  final int pageWidthMm;
  final int pageHeightMm;

  const ScanConfig({
    required this.source,
    this.duplex = false,
    this.dpi = 300,
    this.colorMode = ColorMode.color,
    this.pageWidthMm = 216,  // Letter width
    this.pageHeightMm = 279, // Letter height
  });
}

/// Scan events
abstract class ScanEvent {}

class PageStartedEvent extends ScanEvent {
  final int pageIndex;
  PageStartedEvent(this.pageIndex);
}

class PageDataEvent extends ScanEvent {
  final Uint8List data;
  PageDataEvent(this.data);
}

class PageCompleteEvent extends ScanEvent {
  final int pageIndex;
  final int widthPixels;
  final int heightPixels;
  final int dpi;
  final ColorMode colorMode;
  
  PageCompleteEvent({
    required this.pageIndex,
    required this.widthPixels,
    required this.heightPixels,
    required this.dpi,
    required this.colorMode,
  });
}

class JobCompleteEvent extends ScanEvent {}

/// Scan session for managing ongoing scans
class ScanSession {
  final int _sessionId;
  final PapyrCoreFFI _ffi;
  
  ScanSession._(this._sessionId, this._ffi);
  
  /// Get the next scan event, or null if scanning is complete
  ScanEvent? nextEvent() {
    final eventPtr = _ffi.papyrNextScanEvent(_sessionId);
    if (eventPtr == nullptr) {
      return null;
    }
    
    final event = eventPtr.ref;
    final scanEvent = _convertEvent(event);
    
    _ffi.papyrFreeScanEvent(eventPtr);
    return scanEvent;
  }
  
  ScanEvent _convertEvent(PapyrScanEvent event) {
    switch (event.eventType) {
      case PapyrScanEventType.pageStarted:
        return PageStartedEvent(0); // Simplified
      case PapyrScanEventType.pageData:
        return PageDataEvent(Uint8List(0)); // Simplified
      case PapyrScanEventType.pageComplete:
        return PageCompleteEvent(
          pageIndex: 0,
          widthPixels: 2550,
          heightPixels: 3300,
          dpi: 300,
          colorMode: ColorMode.color,
        );
      case PapyrScanEventType.jobComplete:
      default:
        return JobCompleteEvent();
    }
  }
}

/// {@template papyr}
/// Cross-platform document scanner library
/// {@endtemplate}
class Papyr {
  static Papyr? _instance;
  final PapyrCoreFFI _ffi;
  bool _initialized = false;

  Papyr._() : _ffi = PapyrCoreFFI.instance;

  /// Get the singleton instance
  static Papyr get instance {
    _instance ??= Papyr._();
    return _instance!;
  }

  /// Initialize the papyr library
  /// Must be called before using other methods
  Future<void> initialize() async {
    if (_initialized) return;
    
    final result = _ffi.papyrInit();
    if (result != 0) {
      throw Exception('Failed to initialize papyr: $result');
    }
    _initialized = true;
  }

  /// Get list of available scanners
  Future<List<ScannerInfo>> listScanners() async {
    _ensureInitialized();
    
    final listPtr = _ffi.papyrListScanners();
    if (listPtr == nullptr) {
      return [];
    }

    final scannerList = listPtr.ref;
    final scanners = <ScannerInfo>[];
    
    for (int i = 0; i < scannerList.count; i++) {
      final scannerPtr = scannerList.scanners.elementAt(i);
      final scanner = scannerPtr.ref;
      
      scanners.add(ScannerInfo(
        id: scanner.id.toDartString(),
        name: scanner.name.toDartString(),
        backend: _convertBackend(scanner.backend),
      ));
    }
    
    _ffi.papyrFreeScannerList(listPtr);
    return scanners;
  }

  /// Get capabilities of a specific scanner
  Future<ScannerCapabilities?> getCapabilities(String deviceId) async {
    _ensureInitialized();
    
    final deviceIdPtr = deviceId.toNativeUtf8();
    final capsPtr = _ffi.papyrGetCapabilities(deviceIdPtr);
    
    malloc.free(deviceIdPtr);
    
    if (capsPtr == nullptr) {
      return null;
    }

    final caps = capsPtr.ref;
    
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
    
    final capabilities = ScannerCapabilities(
      sources: sources,
      dpis: dpis,
      colorModes: colorModes,
      supportsDuplex: caps.supportsDuplex != 0,
    );
    
    _ffi.papyrFreeCapabilities(capsPtr);
    return capabilities;
  }

  /// Start a scan session
  Future<ScanSession?> startScan(String deviceId, ScanConfig config) async {
    _ensureInitialized();
    
    final deviceIdPtr = deviceId.toNativeUtf8();
    final configPtr = malloc<PapyrScanConfig>();
    
    configPtr.ref.source = _scanSourceToInt(config.source);
    configPtr.ref.duplex = config.duplex ? 1 : 0;
    configPtr.ref.dpi = config.dpi;
    configPtr.ref.colorMode = _colorModeToInt(config.colorMode);
    configPtr.ref.pageWidthMm = config.pageWidthMm;
    configPtr.ref.pageHeightMm = config.pageHeightMm;
    
    final sessionId = _ffi.papyrStartScan(deviceIdPtr, configPtr);
    
    malloc.free(deviceIdPtr);
    malloc.free(configPtr);
    
    if (sessionId <= 0) {
      return null;
    }
    
    return ScanSession._(sessionId, _ffi);
  }

  /// Cleanup the papyr library
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
      default:
        return ScannerBackend.unknown;
    }
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
