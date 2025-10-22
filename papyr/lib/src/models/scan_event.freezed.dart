// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'scan_event.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$ScanEvent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() started,
    required TResult Function(int bytesScanned, int? totalBytes) progress,
    required TResult Function(ScanResult result) completed,
    required TResult Function(String message, int code) error,
    required TResult Function() cancelled,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? started,
    TResult? Function(int bytesScanned, int? totalBytes)? progress,
    TResult? Function(ScanResult result)? completed,
    TResult? Function(String message, int code)? error,
    TResult? Function()? cancelled,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? started,
    TResult Function(int bytesScanned, int? totalBytes)? progress,
    TResult Function(ScanResult result)? completed,
    TResult Function(String message, int code)? error,
    TResult Function()? cancelled,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ScanStarted value) started,
    required TResult Function(ScanProgress value) progress,
    required TResult Function(ScanCompleted value) completed,
    required TResult Function(ScanError value) error,
    required TResult Function(ScanCancelled value) cancelled,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ScanStarted value)? started,
    TResult? Function(ScanProgress value)? progress,
    TResult? Function(ScanCompleted value)? completed,
    TResult? Function(ScanError value)? error,
    TResult? Function(ScanCancelled value)? cancelled,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ScanStarted value)? started,
    TResult Function(ScanProgress value)? progress,
    TResult Function(ScanCompleted value)? completed,
    TResult Function(ScanError value)? error,
    TResult Function(ScanCancelled value)? cancelled,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ScanEventCopyWith<$Res> {
  factory $ScanEventCopyWith(ScanEvent value, $Res Function(ScanEvent) then) =
      _$ScanEventCopyWithImpl<$Res, ScanEvent>;
}

/// @nodoc
class _$ScanEventCopyWithImpl<$Res, $Val extends ScanEvent>
    implements $ScanEventCopyWith<$Res> {
  _$ScanEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$ScanStartedImplCopyWith<$Res> {
  factory _$$ScanStartedImplCopyWith(
    _$ScanStartedImpl value,
    $Res Function(_$ScanStartedImpl) then,
  ) = __$$ScanStartedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ScanStartedImplCopyWithImpl<$Res>
    extends _$ScanEventCopyWithImpl<$Res, _$ScanStartedImpl>
    implements _$$ScanStartedImplCopyWith<$Res> {
  __$$ScanStartedImplCopyWithImpl(
    _$ScanStartedImpl _value,
    $Res Function(_$ScanStartedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$ScanStartedImpl implements ScanStarted {
  const _$ScanStartedImpl();

  @override
  String toString() {
    return 'ScanEvent.started()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$ScanStartedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() started,
    required TResult Function(int bytesScanned, int? totalBytes) progress,
    required TResult Function(ScanResult result) completed,
    required TResult Function(String message, int code) error,
    required TResult Function() cancelled,
  }) {
    return started();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? started,
    TResult? Function(int bytesScanned, int? totalBytes)? progress,
    TResult? Function(ScanResult result)? completed,
    TResult? Function(String message, int code)? error,
    TResult? Function()? cancelled,
  }) {
    return started?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? started,
    TResult Function(int bytesScanned, int? totalBytes)? progress,
    TResult Function(ScanResult result)? completed,
    TResult Function(String message, int code)? error,
    TResult Function()? cancelled,
    required TResult orElse(),
  }) {
    if (started != null) {
      return started();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ScanStarted value) started,
    required TResult Function(ScanProgress value) progress,
    required TResult Function(ScanCompleted value) completed,
    required TResult Function(ScanError value) error,
    required TResult Function(ScanCancelled value) cancelled,
  }) {
    return started(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ScanStarted value)? started,
    TResult? Function(ScanProgress value)? progress,
    TResult? Function(ScanCompleted value)? completed,
    TResult? Function(ScanError value)? error,
    TResult? Function(ScanCancelled value)? cancelled,
  }) {
    return started?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ScanStarted value)? started,
    TResult Function(ScanProgress value)? progress,
    TResult Function(ScanCompleted value)? completed,
    TResult Function(ScanError value)? error,
    TResult Function(ScanCancelled value)? cancelled,
    required TResult orElse(),
  }) {
    if (started != null) {
      return started(this);
    }
    return orElse();
  }
}

abstract class ScanStarted implements ScanEvent {
  const factory ScanStarted() = _$ScanStartedImpl;
}

/// @nodoc
abstract class _$$ScanProgressImplCopyWith<$Res> {
  factory _$$ScanProgressImplCopyWith(
    _$ScanProgressImpl value,
    $Res Function(_$ScanProgressImpl) then,
  ) = __$$ScanProgressImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int bytesScanned, int? totalBytes});
}

/// @nodoc
class __$$ScanProgressImplCopyWithImpl<$Res>
    extends _$ScanEventCopyWithImpl<$Res, _$ScanProgressImpl>
    implements _$$ScanProgressImplCopyWith<$Res> {
  __$$ScanProgressImplCopyWithImpl(
    _$ScanProgressImpl _value,
    $Res Function(_$ScanProgressImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? bytesScanned = null, Object? totalBytes = freezed}) {
    return _then(
      _$ScanProgressImpl(
        bytesScanned: null == bytesScanned
            ? _value.bytesScanned
            : bytesScanned // ignore: cast_nullable_to_non_nullable
                  as int,
        totalBytes: freezed == totalBytes
            ? _value.totalBytes
            : totalBytes // ignore: cast_nullable_to_non_nullable
                  as int?,
      ),
    );
  }
}

/// @nodoc

class _$ScanProgressImpl implements ScanProgress {
  const _$ScanProgressImpl({required this.bytesScanned, this.totalBytes});

  @override
  final int bytesScanned;
  @override
  final int? totalBytes;

  @override
  String toString() {
    return 'ScanEvent.progress(bytesScanned: $bytesScanned, totalBytes: $totalBytes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ScanProgressImpl &&
            (identical(other.bytesScanned, bytesScanned) ||
                other.bytesScanned == bytesScanned) &&
            (identical(other.totalBytes, totalBytes) ||
                other.totalBytes == totalBytes));
  }

  @override
  int get hashCode => Object.hash(runtimeType, bytesScanned, totalBytes);

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ScanProgressImplCopyWith<_$ScanProgressImpl> get copyWith =>
      __$$ScanProgressImplCopyWithImpl<_$ScanProgressImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() started,
    required TResult Function(int bytesScanned, int? totalBytes) progress,
    required TResult Function(ScanResult result) completed,
    required TResult Function(String message, int code) error,
    required TResult Function() cancelled,
  }) {
    return progress(bytesScanned, totalBytes);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? started,
    TResult? Function(int bytesScanned, int? totalBytes)? progress,
    TResult? Function(ScanResult result)? completed,
    TResult? Function(String message, int code)? error,
    TResult? Function()? cancelled,
  }) {
    return progress?.call(bytesScanned, totalBytes);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? started,
    TResult Function(int bytesScanned, int? totalBytes)? progress,
    TResult Function(ScanResult result)? completed,
    TResult Function(String message, int code)? error,
    TResult Function()? cancelled,
    required TResult orElse(),
  }) {
    if (progress != null) {
      return progress(bytesScanned, totalBytes);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ScanStarted value) started,
    required TResult Function(ScanProgress value) progress,
    required TResult Function(ScanCompleted value) completed,
    required TResult Function(ScanError value) error,
    required TResult Function(ScanCancelled value) cancelled,
  }) {
    return progress(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ScanStarted value)? started,
    TResult? Function(ScanProgress value)? progress,
    TResult? Function(ScanCompleted value)? completed,
    TResult? Function(ScanError value)? error,
    TResult? Function(ScanCancelled value)? cancelled,
  }) {
    return progress?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ScanStarted value)? started,
    TResult Function(ScanProgress value)? progress,
    TResult Function(ScanCompleted value)? completed,
    TResult Function(ScanError value)? error,
    TResult Function(ScanCancelled value)? cancelled,
    required TResult orElse(),
  }) {
    if (progress != null) {
      return progress(this);
    }
    return orElse();
  }
}

abstract class ScanProgress implements ScanEvent {
  const factory ScanProgress({
    required final int bytesScanned,
    final int? totalBytes,
  }) = _$ScanProgressImpl;

  int get bytesScanned;
  int? get totalBytes;

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ScanProgressImplCopyWith<_$ScanProgressImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ScanCompletedImplCopyWith<$Res> {
  factory _$$ScanCompletedImplCopyWith(
    _$ScanCompletedImpl value,
    $Res Function(_$ScanCompletedImpl) then,
  ) = __$$ScanCompletedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({ScanResult result});

  $ScanResultCopyWith<$Res> get result;
}

/// @nodoc
class __$$ScanCompletedImplCopyWithImpl<$Res>
    extends _$ScanEventCopyWithImpl<$Res, _$ScanCompletedImpl>
    implements _$$ScanCompletedImplCopyWith<$Res> {
  __$$ScanCompletedImplCopyWithImpl(
    _$ScanCompletedImpl _value,
    $Res Function(_$ScanCompletedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? result = null}) {
    return _then(
      _$ScanCompletedImpl(
        result: null == result
            ? _value.result
            : result // ignore: cast_nullable_to_non_nullable
                  as ScanResult,
      ),
    );
  }

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $ScanResultCopyWith<$Res> get result {
    return $ScanResultCopyWith<$Res>(_value.result, (value) {
      return _then(_value.copyWith(result: value));
    });
  }
}

/// @nodoc

class _$ScanCompletedImpl implements ScanCompleted {
  const _$ScanCompletedImpl({required this.result});

  @override
  final ScanResult result;

  @override
  String toString() {
    return 'ScanEvent.completed(result: $result)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ScanCompletedImpl &&
            (identical(other.result, result) || other.result == result));
  }

  @override
  int get hashCode => Object.hash(runtimeType, result);

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ScanCompletedImplCopyWith<_$ScanCompletedImpl> get copyWith =>
      __$$ScanCompletedImplCopyWithImpl<_$ScanCompletedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() started,
    required TResult Function(int bytesScanned, int? totalBytes) progress,
    required TResult Function(ScanResult result) completed,
    required TResult Function(String message, int code) error,
    required TResult Function() cancelled,
  }) {
    return completed(result);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? started,
    TResult? Function(int bytesScanned, int? totalBytes)? progress,
    TResult? Function(ScanResult result)? completed,
    TResult? Function(String message, int code)? error,
    TResult? Function()? cancelled,
  }) {
    return completed?.call(result);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? started,
    TResult Function(int bytesScanned, int? totalBytes)? progress,
    TResult Function(ScanResult result)? completed,
    TResult Function(String message, int code)? error,
    TResult Function()? cancelled,
    required TResult orElse(),
  }) {
    if (completed != null) {
      return completed(result);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ScanStarted value) started,
    required TResult Function(ScanProgress value) progress,
    required TResult Function(ScanCompleted value) completed,
    required TResult Function(ScanError value) error,
    required TResult Function(ScanCancelled value) cancelled,
  }) {
    return completed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ScanStarted value)? started,
    TResult? Function(ScanProgress value)? progress,
    TResult? Function(ScanCompleted value)? completed,
    TResult? Function(ScanError value)? error,
    TResult? Function(ScanCancelled value)? cancelled,
  }) {
    return completed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ScanStarted value)? started,
    TResult Function(ScanProgress value)? progress,
    TResult Function(ScanCompleted value)? completed,
    TResult Function(ScanError value)? error,
    TResult Function(ScanCancelled value)? cancelled,
    required TResult orElse(),
  }) {
    if (completed != null) {
      return completed(this);
    }
    return orElse();
  }
}

abstract class ScanCompleted implements ScanEvent {
  const factory ScanCompleted({required final ScanResult result}) =
      _$ScanCompletedImpl;

  ScanResult get result;

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ScanCompletedImplCopyWith<_$ScanCompletedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ScanErrorImplCopyWith<$Res> {
  factory _$$ScanErrorImplCopyWith(
    _$ScanErrorImpl value,
    $Res Function(_$ScanErrorImpl) then,
  ) = __$$ScanErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String message, int code});
}

/// @nodoc
class __$$ScanErrorImplCopyWithImpl<$Res>
    extends _$ScanEventCopyWithImpl<$Res, _$ScanErrorImpl>
    implements _$$ScanErrorImplCopyWith<$Res> {
  __$$ScanErrorImplCopyWithImpl(
    _$ScanErrorImpl _value,
    $Res Function(_$ScanErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null, Object? code = null}) {
    return _then(
      _$ScanErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
        code: null == code
            ? _value.code
            : code // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$ScanErrorImpl implements ScanError {
  const _$ScanErrorImpl({required this.message, required this.code});

  @override
  final String message;
  @override
  final int code;

  @override
  String toString() {
    return 'ScanEvent.error(message: $message, code: $code)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ScanErrorImpl &&
            (identical(other.message, message) || other.message == message) &&
            (identical(other.code, code) || other.code == code));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message, code);

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ScanErrorImplCopyWith<_$ScanErrorImpl> get copyWith =>
      __$$ScanErrorImplCopyWithImpl<_$ScanErrorImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() started,
    required TResult Function(int bytesScanned, int? totalBytes) progress,
    required TResult Function(ScanResult result) completed,
    required TResult Function(String message, int code) error,
    required TResult Function() cancelled,
  }) {
    return error(message, code);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? started,
    TResult? Function(int bytesScanned, int? totalBytes)? progress,
    TResult? Function(ScanResult result)? completed,
    TResult? Function(String message, int code)? error,
    TResult? Function()? cancelled,
  }) {
    return error?.call(message, code);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? started,
    TResult Function(int bytesScanned, int? totalBytes)? progress,
    TResult Function(ScanResult result)? completed,
    TResult Function(String message, int code)? error,
    TResult Function()? cancelled,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(message, code);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ScanStarted value) started,
    required TResult Function(ScanProgress value) progress,
    required TResult Function(ScanCompleted value) completed,
    required TResult Function(ScanError value) error,
    required TResult Function(ScanCancelled value) cancelled,
  }) {
    return error(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ScanStarted value)? started,
    TResult? Function(ScanProgress value)? progress,
    TResult? Function(ScanCompleted value)? completed,
    TResult? Function(ScanError value)? error,
    TResult? Function(ScanCancelled value)? cancelled,
  }) {
    return error?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ScanStarted value)? started,
    TResult Function(ScanProgress value)? progress,
    TResult Function(ScanCompleted value)? completed,
    TResult Function(ScanError value)? error,
    TResult Function(ScanCancelled value)? cancelled,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(this);
    }
    return orElse();
  }
}

abstract class ScanError implements ScanEvent {
  const factory ScanError({
    required final String message,
    required final int code,
  }) = _$ScanErrorImpl;

  String get message;
  int get code;

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ScanErrorImplCopyWith<_$ScanErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ScanCancelledImplCopyWith<$Res> {
  factory _$$ScanCancelledImplCopyWith(
    _$ScanCancelledImpl value,
    $Res Function(_$ScanCancelledImpl) then,
  ) = __$$ScanCancelledImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$ScanCancelledImplCopyWithImpl<$Res>
    extends _$ScanEventCopyWithImpl<$Res, _$ScanCancelledImpl>
    implements _$$ScanCancelledImplCopyWith<$Res> {
  __$$ScanCancelledImplCopyWithImpl(
    _$ScanCancelledImpl _value,
    $Res Function(_$ScanCancelledImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ScanEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$ScanCancelledImpl implements ScanCancelled {
  const _$ScanCancelledImpl();

  @override
  String toString() {
    return 'ScanEvent.cancelled()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$ScanCancelledImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() started,
    required TResult Function(int bytesScanned, int? totalBytes) progress,
    required TResult Function(ScanResult result) completed,
    required TResult Function(String message, int code) error,
    required TResult Function() cancelled,
  }) {
    return cancelled();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function()? started,
    TResult? Function(int bytesScanned, int? totalBytes)? progress,
    TResult? Function(ScanResult result)? completed,
    TResult? Function(String message, int code)? error,
    TResult? Function()? cancelled,
  }) {
    return cancelled?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? started,
    TResult Function(int bytesScanned, int? totalBytes)? progress,
    TResult Function(ScanResult result)? completed,
    TResult Function(String message, int code)? error,
    TResult Function()? cancelled,
    required TResult orElse(),
  }) {
    if (cancelled != null) {
      return cancelled();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ScanStarted value) started,
    required TResult Function(ScanProgress value) progress,
    required TResult Function(ScanCompleted value) completed,
    required TResult Function(ScanError value) error,
    required TResult Function(ScanCancelled value) cancelled,
  }) {
    return cancelled(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ScanStarted value)? started,
    TResult? Function(ScanProgress value)? progress,
    TResult? Function(ScanCompleted value)? completed,
    TResult? Function(ScanError value)? error,
    TResult? Function(ScanCancelled value)? cancelled,
  }) {
    return cancelled?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ScanStarted value)? started,
    TResult Function(ScanProgress value)? progress,
    TResult Function(ScanCompleted value)? completed,
    TResult Function(ScanError value)? error,
    TResult Function(ScanCancelled value)? cancelled,
    required TResult orElse(),
  }) {
    if (cancelled != null) {
      return cancelled(this);
    }
    return orElse();
  }
}

abstract class ScanCancelled implements ScanEvent {
  const factory ScanCancelled() = _$ScanCancelledImpl;
}

/// @nodoc
mixin _$ScanResult {
  /// The scanned data as bytes (if outputPath was null).
  List<int>? get data => throw _privateConstructorUsedError;

  /// The output file path (if outputPath was provided).
  String? get filePath => throw _privateConstructorUsedError;

  /// Number of pages scanned.
  int get pageCount => throw _privateConstructorUsedError;

  /// Total bytes scanned.
  int get totalBytes => throw _privateConstructorUsedError;

  /// Create a copy of ScanResult
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $ScanResultCopyWith<ScanResult> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ScanResultCopyWith<$Res> {
  factory $ScanResultCopyWith(
    ScanResult value,
    $Res Function(ScanResult) then,
  ) = _$ScanResultCopyWithImpl<$Res, ScanResult>;
  @useResult
  $Res call({List<int>? data, String? filePath, int pageCount, int totalBytes});
}

/// @nodoc
class _$ScanResultCopyWithImpl<$Res, $Val extends ScanResult>
    implements $ScanResultCopyWith<$Res> {
  _$ScanResultCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ScanResult
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = freezed,
    Object? filePath = freezed,
    Object? pageCount = null,
    Object? totalBytes = null,
  }) {
    return _then(
      _value.copyWith(
            data: freezed == data
                ? _value.data
                : data // ignore: cast_nullable_to_non_nullable
                      as List<int>?,
            filePath: freezed == filePath
                ? _value.filePath
                : filePath // ignore: cast_nullable_to_non_nullable
                      as String?,
            pageCount: null == pageCount
                ? _value.pageCount
                : pageCount // ignore: cast_nullable_to_non_nullable
                      as int,
            totalBytes: null == totalBytes
                ? _value.totalBytes
                : totalBytes // ignore: cast_nullable_to_non_nullable
                      as int,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$ScanResultImplCopyWith<$Res>
    implements $ScanResultCopyWith<$Res> {
  factory _$$ScanResultImplCopyWith(
    _$ScanResultImpl value,
    $Res Function(_$ScanResultImpl) then,
  ) = __$$ScanResultImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<int>? data, String? filePath, int pageCount, int totalBytes});
}

/// @nodoc
class __$$ScanResultImplCopyWithImpl<$Res>
    extends _$ScanResultCopyWithImpl<$Res, _$ScanResultImpl>
    implements _$$ScanResultImplCopyWith<$Res> {
  __$$ScanResultImplCopyWithImpl(
    _$ScanResultImpl _value,
    $Res Function(_$ScanResultImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ScanResult
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = freezed,
    Object? filePath = freezed,
    Object? pageCount = null,
    Object? totalBytes = null,
  }) {
    return _then(
      _$ScanResultImpl(
        data: freezed == data
            ? _value._data
            : data // ignore: cast_nullable_to_non_nullable
                  as List<int>?,
        filePath: freezed == filePath
            ? _value.filePath
            : filePath // ignore: cast_nullable_to_non_nullable
                  as String?,
        pageCount: null == pageCount
            ? _value.pageCount
            : pageCount // ignore: cast_nullable_to_non_nullable
                  as int,
        totalBytes: null == totalBytes
            ? _value.totalBytes
            : totalBytes // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$ScanResultImpl implements _ScanResult {
  const _$ScanResultImpl({
    final List<int>? data,
    this.filePath,
    required this.pageCount,
    required this.totalBytes,
  }) : _data = data;

  /// The scanned data as bytes (if outputPath was null).
  final List<int>? _data;

  /// The scanned data as bytes (if outputPath was null).
  @override
  List<int>? get data {
    final value = _data;
    if (value == null) return null;
    if (_data is EqualUnmodifiableListView) return _data;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(value);
  }

  /// The output file path (if outputPath was provided).
  @override
  final String? filePath;

  /// Number of pages scanned.
  @override
  final int pageCount;

  /// Total bytes scanned.
  @override
  final int totalBytes;

  @override
  String toString() {
    return 'ScanResult(data: $data, filePath: $filePath, pageCount: $pageCount, totalBytes: $totalBytes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ScanResultImpl &&
            const DeepCollectionEquality().equals(other._data, _data) &&
            (identical(other.filePath, filePath) ||
                other.filePath == filePath) &&
            (identical(other.pageCount, pageCount) ||
                other.pageCount == pageCount) &&
            (identical(other.totalBytes, totalBytes) ||
                other.totalBytes == totalBytes));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    const DeepCollectionEquality().hash(_data),
    filePath,
    pageCount,
    totalBytes,
  );

  /// Create a copy of ScanResult
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ScanResultImplCopyWith<_$ScanResultImpl> get copyWith =>
      __$$ScanResultImplCopyWithImpl<_$ScanResultImpl>(this, _$identity);
}

abstract class _ScanResult implements ScanResult {
  const factory _ScanResult({
    final List<int>? data,
    final String? filePath,
    required final int pageCount,
    required final int totalBytes,
  }) = _$ScanResultImpl;

  /// The scanned data as bytes (if outputPath was null).
  @override
  List<int>? get data;

  /// The output file path (if outputPath was provided).
  @override
  String? get filePath;

  /// Number of pages scanned.
  @override
  int get pageCount;

  /// Total bytes scanned.
  @override
  int get totalBytes;

  /// Create a copy of ScanResult
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ScanResultImplCopyWith<_$ScanResultImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
