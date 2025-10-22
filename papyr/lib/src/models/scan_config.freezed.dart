// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'scan_config.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$ScanConfig {
  /// The scanner to use for scanning.
  Scanner get scanner => throw _privateConstructorUsedError;

  /// The source to scan from.
  ScanSource get source => throw _privateConstructorUsedError;

  /// The resolution in DPI.
  int get dpi => throw _privateConstructorUsedError;

  /// The color mode to use.
  ColorMode get colorMode => throw _privateConstructorUsedError;

  /// The file format for the scanned document.
  ScanFormat get format => throw _privateConstructorUsedError;

  /// The output file path (optional, if null returns bytes).
  String? get outputPath => throw _privateConstructorUsedError;

  /// Whether to use duplex scanning (both sides).
  bool get useDuplex => throw _privateConstructorUsedError;

  /// Create a copy of ScanConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $ScanConfigCopyWith<ScanConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ScanConfigCopyWith<$Res> {
  factory $ScanConfigCopyWith(
    ScanConfig value,
    $Res Function(ScanConfig) then,
  ) = _$ScanConfigCopyWithImpl<$Res, ScanConfig>;
  @useResult
  $Res call({
    Scanner scanner,
    ScanSource source,
    int dpi,
    ColorMode colorMode,
    ScanFormat format,
    String? outputPath,
    bool useDuplex,
  });

  $ScannerCopyWith<$Res> get scanner;
}

/// @nodoc
class _$ScanConfigCopyWithImpl<$Res, $Val extends ScanConfig>
    implements $ScanConfigCopyWith<$Res> {
  _$ScanConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ScanConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? scanner = null,
    Object? source = null,
    Object? dpi = null,
    Object? colorMode = null,
    Object? format = null,
    Object? outputPath = freezed,
    Object? useDuplex = null,
  }) {
    return _then(
      _value.copyWith(
            scanner: null == scanner
                ? _value.scanner
                : scanner // ignore: cast_nullable_to_non_nullable
                      as Scanner,
            source: null == source
                ? _value.source
                : source // ignore: cast_nullable_to_non_nullable
                      as ScanSource,
            dpi: null == dpi
                ? _value.dpi
                : dpi // ignore: cast_nullable_to_non_nullable
                      as int,
            colorMode: null == colorMode
                ? _value.colorMode
                : colorMode // ignore: cast_nullable_to_non_nullable
                      as ColorMode,
            format: null == format
                ? _value.format
                : format // ignore: cast_nullable_to_non_nullable
                      as ScanFormat,
            outputPath: freezed == outputPath
                ? _value.outputPath
                : outputPath // ignore: cast_nullable_to_non_nullable
                      as String?,
            useDuplex: null == useDuplex
                ? _value.useDuplex
                : useDuplex // ignore: cast_nullable_to_non_nullable
                      as bool,
          )
          as $Val,
    );
  }

  /// Create a copy of ScanConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $ScannerCopyWith<$Res> get scanner {
    return $ScannerCopyWith<$Res>(_value.scanner, (value) {
      return _then(_value.copyWith(scanner: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$ScanConfigImplCopyWith<$Res>
    implements $ScanConfigCopyWith<$Res> {
  factory _$$ScanConfigImplCopyWith(
    _$ScanConfigImpl value,
    $Res Function(_$ScanConfigImpl) then,
  ) = __$$ScanConfigImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    Scanner scanner,
    ScanSource source,
    int dpi,
    ColorMode colorMode,
    ScanFormat format,
    String? outputPath,
    bool useDuplex,
  });

  @override
  $ScannerCopyWith<$Res> get scanner;
}

/// @nodoc
class __$$ScanConfigImplCopyWithImpl<$Res>
    extends _$ScanConfigCopyWithImpl<$Res, _$ScanConfigImpl>
    implements _$$ScanConfigImplCopyWith<$Res> {
  __$$ScanConfigImplCopyWithImpl(
    _$ScanConfigImpl _value,
    $Res Function(_$ScanConfigImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ScanConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? scanner = null,
    Object? source = null,
    Object? dpi = null,
    Object? colorMode = null,
    Object? format = null,
    Object? outputPath = freezed,
    Object? useDuplex = null,
  }) {
    return _then(
      _$ScanConfigImpl(
        scanner: null == scanner
            ? _value.scanner
            : scanner // ignore: cast_nullable_to_non_nullable
                  as Scanner,
        source: null == source
            ? _value.source
            : source // ignore: cast_nullable_to_non_nullable
                  as ScanSource,
        dpi: null == dpi
            ? _value.dpi
            : dpi // ignore: cast_nullable_to_non_nullable
                  as int,
        colorMode: null == colorMode
            ? _value.colorMode
            : colorMode // ignore: cast_nullable_to_non_nullable
                  as ColorMode,
        format: null == format
            ? _value.format
            : format // ignore: cast_nullable_to_non_nullable
                  as ScanFormat,
        outputPath: freezed == outputPath
            ? _value.outputPath
            : outputPath // ignore: cast_nullable_to_non_nullable
                  as String?,
        useDuplex: null == useDuplex
            ? _value.useDuplex
            : useDuplex // ignore: cast_nullable_to_non_nullable
                  as bool,
      ),
    );
  }
}

/// @nodoc

class _$ScanConfigImpl implements _ScanConfig {
  const _$ScanConfigImpl({
    required this.scanner,
    this.source = ScanSource.flatbed,
    this.dpi = 300,
    this.colorMode = ColorMode.color,
    this.format = ScanFormat.pdf,
    this.outputPath,
    this.useDuplex = false,
  });

  /// The scanner to use for scanning.
  @override
  final Scanner scanner;

  /// The source to scan from.
  @override
  @JsonKey()
  final ScanSource source;

  /// The resolution in DPI.
  @override
  @JsonKey()
  final int dpi;

  /// The color mode to use.
  @override
  @JsonKey()
  final ColorMode colorMode;

  /// The file format for the scanned document.
  @override
  @JsonKey()
  final ScanFormat format;

  /// The output file path (optional, if null returns bytes).
  @override
  final String? outputPath;

  /// Whether to use duplex scanning (both sides).
  @override
  @JsonKey()
  final bool useDuplex;

  @override
  String toString() {
    return 'ScanConfig(scanner: $scanner, source: $source, dpi: $dpi, colorMode: $colorMode, format: $format, outputPath: $outputPath, useDuplex: $useDuplex)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ScanConfigImpl &&
            (identical(other.scanner, scanner) || other.scanner == scanner) &&
            (identical(other.source, source) || other.source == source) &&
            (identical(other.dpi, dpi) || other.dpi == dpi) &&
            (identical(other.colorMode, colorMode) ||
                other.colorMode == colorMode) &&
            (identical(other.format, format) || other.format == format) &&
            (identical(other.outputPath, outputPath) ||
                other.outputPath == outputPath) &&
            (identical(other.useDuplex, useDuplex) ||
                other.useDuplex == useDuplex));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    scanner,
    source,
    dpi,
    colorMode,
    format,
    outputPath,
    useDuplex,
  );

  /// Create a copy of ScanConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ScanConfigImplCopyWith<_$ScanConfigImpl> get copyWith =>
      __$$ScanConfigImplCopyWithImpl<_$ScanConfigImpl>(this, _$identity);
}

abstract class _ScanConfig implements ScanConfig {
  const factory _ScanConfig({
    required final Scanner scanner,
    final ScanSource source,
    final int dpi,
    final ColorMode colorMode,
    final ScanFormat format,
    final String? outputPath,
    final bool useDuplex,
  }) = _$ScanConfigImpl;

  /// The scanner to use for scanning.
  @override
  Scanner get scanner;

  /// The source to scan from.
  @override
  ScanSource get source;

  /// The resolution in DPI.
  @override
  int get dpi;

  /// The color mode to use.
  @override
  ColorMode get colorMode;

  /// The file format for the scanned document.
  @override
  ScanFormat get format;

  /// The output file path (optional, if null returns bytes).
  @override
  String? get outputPath;

  /// Whether to use duplex scanning (both sides).
  @override
  bool get useDuplex;

  /// Create a copy of ScanConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ScanConfigImplCopyWith<_$ScanConfigImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
