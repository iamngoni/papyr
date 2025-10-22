// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'scanner.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$Scanner {
  /// Unique identifier for this scanner
  String get id => throw _privateConstructorUsedError;

  /// Human-readable name of the scanner
  String get name => throw _privateConstructorUsedError;

  /// Backend protocol used to communicate with this scanner
  ScannerBackend get backend => throw _privateConstructorUsedError;

  /// Scanner capabilities (resolution, color modes, etc)
  Capabilities get capabilities => throw _privateConstructorUsedError;

  /// Create a copy of Scanner
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $ScannerCopyWith<Scanner> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ScannerCopyWith<$Res> {
  factory $ScannerCopyWith(Scanner value, $Res Function(Scanner) then) =
      _$ScannerCopyWithImpl<$Res, Scanner>;
  @useResult
  $Res call({
    String id,
    String name,
    ScannerBackend backend,
    Capabilities capabilities,
  });

  $CapabilitiesCopyWith<$Res> get capabilities;
}

/// @nodoc
class _$ScannerCopyWithImpl<$Res, $Val extends Scanner>
    implements $ScannerCopyWith<$Res> {
  _$ScannerCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Scanner
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? backend = null,
    Object? capabilities = null,
  }) {
    return _then(
      _value.copyWith(
            id: null == id
                ? _value.id
                : id // ignore: cast_nullable_to_non_nullable
                      as String,
            name: null == name
                ? _value.name
                : name // ignore: cast_nullable_to_non_nullable
                      as String,
            backend: null == backend
                ? _value.backend
                : backend // ignore: cast_nullable_to_non_nullable
                      as ScannerBackend,
            capabilities: null == capabilities
                ? _value.capabilities
                : capabilities // ignore: cast_nullable_to_non_nullable
                      as Capabilities,
          )
          as $Val,
    );
  }

  /// Create a copy of Scanner
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $CapabilitiesCopyWith<$Res> get capabilities {
    return $CapabilitiesCopyWith<$Res>(_value.capabilities, (value) {
      return _then(_value.copyWith(capabilities: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$ScannerImplCopyWith<$Res> implements $ScannerCopyWith<$Res> {
  factory _$$ScannerImplCopyWith(
    _$ScannerImpl value,
    $Res Function(_$ScannerImpl) then,
  ) = __$$ScannerImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String id,
    String name,
    ScannerBackend backend,
    Capabilities capabilities,
  });

  @override
  $CapabilitiesCopyWith<$Res> get capabilities;
}

/// @nodoc
class __$$ScannerImplCopyWithImpl<$Res>
    extends _$ScannerCopyWithImpl<$Res, _$ScannerImpl>
    implements _$$ScannerImplCopyWith<$Res> {
  __$$ScannerImplCopyWithImpl(
    _$ScannerImpl _value,
    $Res Function(_$ScannerImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Scanner
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? backend = null,
    Object? capabilities = null,
  }) {
    return _then(
      _$ScannerImpl(
        id: null == id
            ? _value.id
            : id // ignore: cast_nullable_to_non_nullable
                  as String,
        name: null == name
            ? _value.name
            : name // ignore: cast_nullable_to_non_nullable
                  as String,
        backend: null == backend
            ? _value.backend
            : backend // ignore: cast_nullable_to_non_nullable
                  as ScannerBackend,
        capabilities: null == capabilities
            ? _value.capabilities
            : capabilities // ignore: cast_nullable_to_non_nullable
                  as Capabilities,
      ),
    );
  }
}

/// @nodoc

class _$ScannerImpl implements _Scanner {
  const _$ScannerImpl({
    required this.id,
    required this.name,
    required this.backend,
    required this.capabilities,
  });

  /// Unique identifier for this scanner
  @override
  final String id;

  /// Human-readable name of the scanner
  @override
  final String name;

  /// Backend protocol used to communicate with this scanner
  @override
  final ScannerBackend backend;

  /// Scanner capabilities (resolution, color modes, etc)
  @override
  final Capabilities capabilities;

  @override
  String toString() {
    return 'Scanner(id: $id, name: $name, backend: $backend, capabilities: $capabilities)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ScannerImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.backend, backend) || other.backend == backend) &&
            (identical(other.capabilities, capabilities) ||
                other.capabilities == capabilities));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id, name, backend, capabilities);

  /// Create a copy of Scanner
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ScannerImplCopyWith<_$ScannerImpl> get copyWith =>
      __$$ScannerImplCopyWithImpl<_$ScannerImpl>(this, _$identity);
}

abstract class _Scanner implements Scanner {
  const factory _Scanner({
    required final String id,
    required final String name,
    required final ScannerBackend backend,
    required final Capabilities capabilities,
  }) = _$ScannerImpl;

  /// Unique identifier for this scanner
  @override
  String get id;

  /// Human-readable name of the scanner
  @override
  String get name;

  /// Backend protocol used to communicate with this scanner
  @override
  ScannerBackend get backend;

  /// Scanner capabilities (resolution, color modes, etc)
  @override
  Capabilities get capabilities;

  /// Create a copy of Scanner
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ScannerImplCopyWith<_$ScannerImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Capabilities {
  /// Available scan sources (flatbed, ADF, etc)
  List<ScanSource> get sources => throw _privateConstructorUsedError;

  /// Supported DPI/resolution values
  List<int> get dpis => throw _privateConstructorUsedError;

  /// Supported color modes
  List<ColorMode> get colorModes => throw _privateConstructorUsedError;

  /// Whether the scanner supports duplex (two-sided) scanning
  bool get supportsDuplex => throw _privateConstructorUsedError;

  /// Create a copy of Capabilities
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CapabilitiesCopyWith<Capabilities> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CapabilitiesCopyWith<$Res> {
  factory $CapabilitiesCopyWith(
    Capabilities value,
    $Res Function(Capabilities) then,
  ) = _$CapabilitiesCopyWithImpl<$Res, Capabilities>;
  @useResult
  $Res call({
    List<ScanSource> sources,
    List<int> dpis,
    List<ColorMode> colorModes,
    bool supportsDuplex,
  });
}

/// @nodoc
class _$CapabilitiesCopyWithImpl<$Res, $Val extends Capabilities>
    implements $CapabilitiesCopyWith<$Res> {
  _$CapabilitiesCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of Capabilities
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? sources = null,
    Object? dpis = null,
    Object? colorModes = null,
    Object? supportsDuplex = null,
  }) {
    return _then(
      _value.copyWith(
            sources: null == sources
                ? _value.sources
                : sources // ignore: cast_nullable_to_non_nullable
                      as List<ScanSource>,
            dpis: null == dpis
                ? _value.dpis
                : dpis // ignore: cast_nullable_to_non_nullable
                      as List<int>,
            colorModes: null == colorModes
                ? _value.colorModes
                : colorModes // ignore: cast_nullable_to_non_nullable
                      as List<ColorMode>,
            supportsDuplex: null == supportsDuplex
                ? _value.supportsDuplex
                : supportsDuplex // ignore: cast_nullable_to_non_nullable
                      as bool,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$CapabilitiesImplCopyWith<$Res>
    implements $CapabilitiesCopyWith<$Res> {
  factory _$$CapabilitiesImplCopyWith(
    _$CapabilitiesImpl value,
    $Res Function(_$CapabilitiesImpl) then,
  ) = __$$CapabilitiesImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    List<ScanSource> sources,
    List<int> dpis,
    List<ColorMode> colorModes,
    bool supportsDuplex,
  });
}

/// @nodoc
class __$$CapabilitiesImplCopyWithImpl<$Res>
    extends _$CapabilitiesCopyWithImpl<$Res, _$CapabilitiesImpl>
    implements _$$CapabilitiesImplCopyWith<$Res> {
  __$$CapabilitiesImplCopyWithImpl(
    _$CapabilitiesImpl _value,
    $Res Function(_$CapabilitiesImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of Capabilities
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? sources = null,
    Object? dpis = null,
    Object? colorModes = null,
    Object? supportsDuplex = null,
  }) {
    return _then(
      _$CapabilitiesImpl(
        sources: null == sources
            ? _value._sources
            : sources // ignore: cast_nullable_to_non_nullable
                  as List<ScanSource>,
        dpis: null == dpis
            ? _value._dpis
            : dpis // ignore: cast_nullable_to_non_nullable
                  as List<int>,
        colorModes: null == colorModes
            ? _value._colorModes
            : colorModes // ignore: cast_nullable_to_non_nullable
                  as List<ColorMode>,
        supportsDuplex: null == supportsDuplex
            ? _value.supportsDuplex
            : supportsDuplex // ignore: cast_nullable_to_non_nullable
                  as bool,
      ),
    );
  }
}

/// @nodoc

class _$CapabilitiesImpl implements _Capabilities {
  const _$CapabilitiesImpl({
    required final List<ScanSource> sources,
    required final List<int> dpis,
    required final List<ColorMode> colorModes,
    required this.supportsDuplex,
  }) : _sources = sources,
       _dpis = dpis,
       _colorModes = colorModes;

  /// Available scan sources (flatbed, ADF, etc)
  final List<ScanSource> _sources;

  /// Available scan sources (flatbed, ADF, etc)
  @override
  List<ScanSource> get sources {
    if (_sources is EqualUnmodifiableListView) return _sources;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_sources);
  }

  /// Supported DPI/resolution values
  final List<int> _dpis;

  /// Supported DPI/resolution values
  @override
  List<int> get dpis {
    if (_dpis is EqualUnmodifiableListView) return _dpis;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_dpis);
  }

  /// Supported color modes
  final List<ColorMode> _colorModes;

  /// Supported color modes
  @override
  List<ColorMode> get colorModes {
    if (_colorModes is EqualUnmodifiableListView) return _colorModes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_colorModes);
  }

  /// Whether the scanner supports duplex (two-sided) scanning
  @override
  final bool supportsDuplex;

  @override
  String toString() {
    return 'Capabilities(sources: $sources, dpis: $dpis, colorModes: $colorModes, supportsDuplex: $supportsDuplex)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CapabilitiesImpl &&
            const DeepCollectionEquality().equals(other._sources, _sources) &&
            const DeepCollectionEquality().equals(other._dpis, _dpis) &&
            const DeepCollectionEquality().equals(
              other._colorModes,
              _colorModes,
            ) &&
            (identical(other.supportsDuplex, supportsDuplex) ||
                other.supportsDuplex == supportsDuplex));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    const DeepCollectionEquality().hash(_sources),
    const DeepCollectionEquality().hash(_dpis),
    const DeepCollectionEquality().hash(_colorModes),
    supportsDuplex,
  );

  /// Create a copy of Capabilities
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CapabilitiesImplCopyWith<_$CapabilitiesImpl> get copyWith =>
      __$$CapabilitiesImplCopyWithImpl<_$CapabilitiesImpl>(this, _$identity);
}

abstract class _Capabilities implements Capabilities {
  const factory _Capabilities({
    required final List<ScanSource> sources,
    required final List<int> dpis,
    required final List<ColorMode> colorModes,
    required final bool supportsDuplex,
  }) = _$CapabilitiesImpl;

  /// Available scan sources (flatbed, ADF, etc)
  @override
  List<ScanSource> get sources;

  /// Supported DPI/resolution values
  @override
  List<int> get dpis;

  /// Supported color modes
  @override
  List<ColorMode> get colorModes;

  /// Whether the scanner supports duplex (two-sided) scanning
  @override
  bool get supportsDuplex;

  /// Create a copy of Capabilities
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CapabilitiesImplCopyWith<_$CapabilitiesImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
