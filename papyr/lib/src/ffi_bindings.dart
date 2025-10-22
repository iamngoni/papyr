// papyr Dart FFI bindings
// Auto-generated bindings for papyr_core C interface

import 'dart:ffi';
import 'dart:io';

// Backend types
class PapyrBackend {
  static const int wia = 0;
  static const int sane = 1;
  static const int ica = 2;
  static const int unknown = 99;
}

// Scan source types
class PapyrScanSource {
  static const int flatbed = 0;
  static const int adf = 1;
  static const int adfDuplex = 2;
}

// Color mode types
class PapyrColorMode {
  static const int color = 0;
  static const int gray = 1;
  static const int bw = 2;
}

// Scan event types
class PapyrScanEventType {
  static const int pageStarted = 0;
  static const int pageData = 1;
  static const int pageComplete = 2;
  static const int jobComplete = 3;
}

// Native structures
final class PapyrScannerInfo extends Struct {
  external Pointer<Utf8> id;
  external Pointer<Utf8> name;
  @Int32()
  external int backend;
}

final class PapyrScannerInfoList extends Struct {
  external Pointer<PapyrScannerInfo> scanners;
  @Size()
  external int count;
}

final class PapyrCapabilities extends Struct {
  external Pointer<Int32> sources;
  @Size()
  external int sourcesCount;
  external Pointer<Int32> dpis;
  @Size()
  external int dpisCount;
  external Pointer<Int32> colorModes;
  @Size()
  external int colorModesCount;
  @Int32()
  external int supportsDuplex;
}

final class PapyrScanConfig extends Struct {
  @Int32()
  external int source;
  @Int32()
  external int duplex;
  @Int32()
  external int dpi;
  @Int32()
  external int colorMode;
  @Int32()
  external int pageWidthMm;
  @Int32()
  external int pageHeightMm;
}

final class PapyrScanEvent extends Struct {
  @Int32()
  external int eventType;
  external Pointer<Void> data;
  @Size()
  external int dataSize;
}

// Function signatures
typedef PapyrInitNative = Int32 Function();
typedef PapyrInit = int Function();

typedef PapyrListScannersNative = Pointer<PapyrScannerInfoList> Function();
typedef PapyrListScanners = Pointer<PapyrScannerInfoList> Function();

typedef PapyrGetCapabilitiesNative = Pointer<PapyrCapabilities> Function(
    Pointer<Utf8>);
typedef PapyrGetCapabilities = Pointer<PapyrCapabilities> Function(
    Pointer<Utf8>);

typedef PapyrStartScanNative = Int32 Function(
    Pointer<Utf8>, Pointer<PapyrScanConfig>);
typedef PapyrStartScan = int Function(Pointer<Utf8>, Pointer<PapyrScanConfig>);

typedef PapyrNextScanEventNative = Pointer<PapyrScanEvent> Function(Int32);
typedef PapyrNextScanEvent = Pointer<PapyrScanEvent> Function(int);

typedef PapyrFreeScannerListNative = Void Function(
    Pointer<PapyrScannerInfoList>);
typedef PapyrFreeScannerList = void Function(Pointer<PapyrScannerInfoList>);

typedef PapyrFreeCapabilitiesNative = Void Function(Pointer<PapyrCapabilities>);
typedef PapyrFreeCapabilities = void Function(Pointer<PapyrCapabilities>);

typedef PapyrFreeScanEventNative = Void Function(Pointer<PapyrScanEvent>);
typedef PapyrFreeScanEvent = void Function(Pointer<PapyrScanEvent>);

typedef PapyrCleanupNative = Void Function();
typedef PapyrCleanup = void Function();

// Library wrapper
class PapyrCoreFFI {
  static PapyrCoreFFI? _instance;
  late final DynamicLibrary _lib;
  
  // Function pointers
  late final PapyrInit papyrInit;
  late final PapyrListScanners papyrListScanners;
  late final PapyrGetCapabilities papyrGetCapabilities;
  late final PapyrStartScan papyrStartScan;
  late final PapyrNextScanEvent papyrNextScanEvent;
  late final PapyrFreeScannerList papyrFreeScannerList;
  late final PapyrFreeCapabilities papyrFreeCapabilities;
  late final PapyrFreeScanEvent papyrFreeScanEvent;
  late final PapyrCleanup papyrCleanup;

  PapyrCoreFFI._() {
    _lib = _loadLibrary();
    _bindFunctions();
  }

  static PapyrCoreFFI get instance {
    _instance ??= PapyrCoreFFI._();
    return _instance!;
  }

  DynamicLibrary _loadLibrary() {
    if (Platform.isWindows) {
      return DynamicLibrary.open('papyr_core.dll');
    } else if (Platform.isMacOS) {
      return DynamicLibrary.open('libpapyr_core.dylib');
    } else if (Platform.isLinux) {
      return DynamicLibrary.open('libpapyr_core.so');
    } else {
      throw UnsupportedError('Unsupported platform: ${Platform.operatingSystem}');
    }
  }

  void _bindFunctions() {
    papyrInit = _lib
        .lookup<NativeFunction<PapyrInitNative>>('papyr_init')
        .asFunction();
    
    papyrListScanners = _lib
        .lookup<NativeFunction<PapyrListScannersNative>>('papyr_list_scanners')
        .asFunction();
    
    papyrGetCapabilities = _lib
        .lookup<NativeFunction<PapyrGetCapabilitiesNative>>('papyr_get_capabilities')
        .asFunction();
    
    papyrStartScan = _lib
        .lookup<NativeFunction<PapyrStartScanNative>>('papyr_start_scan')
        .asFunction();
    
    papyrNextScanEvent = _lib
        .lookup<NativeFunction<PapyrNextScanEventNative>>('papyr_next_scan_event')
        .asFunction();
    
    papyrFreeScannerList = _lib
        .lookup<NativeFunction<PapyrFreeScannerListNative>>('papyr_free_scanner_list')
        .asFunction();
    
    papyrFreeCapabilities = _lib
        .lookup<NativeFunction<PapyrFreeCapabilitiesNative>>('papyr_free_capabilities')
        .asFunction();
    
    papyrFreeScanEvent = _lib
        .lookup<NativeFunction<PapyrFreeScanEventNative>>('papyr_free_scan_event')
        .asFunction();
    
    papyrCleanup = _lib
        .lookup<NativeFunction<PapyrCleanupNative>>('papyr_cleanup')
        .asFunction();
  }
}
