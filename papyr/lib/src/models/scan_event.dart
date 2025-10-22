import 'package:freezed_annotation/freezed_annotation.dart';

part 'scan_event.freezed.dart';

/// Events emitted during a scan operation.
@freezed
class ScanEvent with _$ScanEvent {
  /// Scan operation started.
  const factory ScanEvent.started() = ScanStarted;

  /// Scan progress update.
  const factory ScanEvent.progress({
    required int bytesScanned,
    int? totalBytes,
  }) = ScanProgress;

  /// Scan completed successfully.
  const factory ScanEvent.completed({
    required ScanResult result,
  }) = ScanCompleted;

  /// Scan failed with an error.
  const factory ScanEvent.error({
    required String message,
    required int code,
  }) = ScanError;

  /// Scan was cancelled.
  const factory ScanEvent.cancelled() = ScanCancelled;
}

/// Result of a completed scan operation.
@freezed
class ScanResult with _$ScanResult {
  const factory ScanResult({
    /// The scanned data as bytes (if outputPath was null).
    List<int>? data,

    /// The output file path (if outputPath was provided).
    String? filePath,

    /// Number of pages scanned.
    required int pageCount,

    /// Total bytes scanned.
    required int totalBytes,
  }) = _ScanResult;
}
