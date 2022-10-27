// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.49.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names

import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'package:meta/meta.dart';
import 'package:meta/meta.dart';
import 'dart:ffi' as ffi;

abstract class AudioTrackConstraintsApi {
  /// Creates new [`AudioTrackConstraints`] with none constraints configured.
  RefCellAudioTrackConstraints audioTrackConstraintsNew({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAudioTrackConstraintsNewConstMeta;

  /// Sets an exact [deviceId][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
  void audioTrackConstraintsDeviceId(
      {required RefCellAudioTrackConstraints track,
      required String deviceId,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAudioTrackConstraintsDeviceIdConstMeta;
}

@sealed
class RefCellAudioTrackConstraints extends FrbOpaque {
  RefCellAudioTrackConstraints.fromRaw(int ptr, int drop, int share)
      : super.unsafe(ptr, drop, share);
}

class AudioTrackConstraintsApiImpl implements AudioTrackConstraintsApi {
  final AudioTrackConstraintsApiPlatform _platform;
  factory AudioTrackConstraintsApiImpl(ExternalLibrary dylib) =>
      AudioTrackConstraintsApiImpl.raw(AudioTrackConstraintsApiPlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory AudioTrackConstraintsApiImpl.wasm(FutureOr<WasmModule> module) =>
      AudioTrackConstraintsApiImpl(module as ExternalLibrary);
  AudioTrackConstraintsApiImpl.raw(this._platform);
  RefCellAudioTrackConstraints audioTrackConstraintsNew({dynamic hint}) {
    return _platform.executeSync(FlutterRustBridgeSyncTask(
      callFfi: () => _platform.inner.wire_audio_track_constraints_new(),
      parseSuccessData: _wire2api_SyncReturn_RefCellAudioTrackConstraints,
      constMeta: kAudioTrackConstraintsNewConstMeta,
      argValues: [],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kAudioTrackConstraintsNewConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "audio_track_constraints_new",
        argNames: [],
      );

  void audioTrackConstraintsDeviceId(
      {required RefCellAudioTrackConstraints track,
      required String deviceId,
      dynamic hint}) {
    var arg0 = _platform.api2wire_RefCellAudioTrackConstraints(track);
    var arg1 = _platform.api2wire_String(deviceId);
    return _platform.executeSync(FlutterRustBridgeSyncTask(
      callFfi: () =>
          _platform.inner.wire_audio_track_constraints_device_id(arg0, arg1),
      parseSuccessData: _wire2api_SyncReturn_unit,
      constMeta: kAudioTrackConstraintsDeviceIdConstMeta,
      argValues: [track, deviceId],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kAudioTrackConstraintsDeviceIdConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "audio_track_constraints_device_id",
        argNames: ["track", "deviceId"],
      );

// Section: wire2api

  RefCellAudioTrackConstraints _wire2api_RefCellAudioTrackConstraints(
      dynamic raw) {
    return RefCellAudioTrackConstraints.fromRaw(raw[0], raw[1], raw[2]);
  }

  RefCellAudioTrackConstraints
      _wire2api_SyncReturn_RefCellAudioTrackConstraints(dynamic raw) {
    var pointBitLen = raw.length ~/ 3;
    var ptrList = List.filled(pointBitLen, 0);
    var dropList = List.filled(pointBitLen, 0);
    var lendList = List.filled(pointBitLen, 0);

    List.copyRange(ptrList, 0, raw, 0, pointBitLen);
    List.copyRange(dropList, 0, raw, pointBitLen, pointBitLen * 2);
    List.copyRange(lendList, 0, raw, pointBitLen * 2);

    int ptr = 0;
    int drop = 0;
    int lend = 0;

    if (pointBitLen == 8) {
      ptr = ByteData.view(Uint8List.fromList(ptrList).buffer).getUint64(0);
      drop = ByteData.view(Uint8List.fromList(dropList).buffer).getUint64(0);
      lend = ByteData.view(Uint8List.fromList(lendList).buffer).getUint64(0);
    } else if (pointBitLen == 4) {
      ptr = ByteData.view(Uint8List.fromList(ptrList).buffer).getUint32(0);
      drop = ByteData.view(Uint8List.fromList(dropList).buffer).getUint32(0);
      lend = ByteData.view(Uint8List.fromList(lendList).buffer).getUint32(0);
    }

    return RefCellAudioTrackConstraints.fromRaw(ptr, drop, lend);
  }

  void _wire2api_SyncReturn_unit(dynamic raw) {
    return;
  }

  void _wire2api_unit(dynamic raw) {
    return;
  }
}

// Section: api2wire

@protected
int api2wire_u8(int raw) {
  return raw;
}

class AudioTrackConstraintsApiPlatform
    extends FlutterRustBridgeBase<AudioTrackConstraintsApiWire> {
  AudioTrackConstraintsApiPlatform(ffi.DynamicLibrary dylib)
      : super(AudioTrackConstraintsApiWire(dylib));
// Section: api2wire

  @protected
  ffi.Pointer<wire_RefCellAudioTrackConstraints>
      api2wire_RefCellAudioTrackConstraints(RefCellAudioTrackConstraints raw) {
    if (raw.isStale()) {
      throw 'Use after dispose.';
    }
    final ptr = inner.new_RefCellAudioTrackConstraints();
    _api_fill_to_wire_RefCellAudioTrackConstraints(raw, ptr);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_1(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: api_fill_to_wire

  void _api_fill_to_wire_RefCellAudioTrackConstraints(
      RefCellAudioTrackConstraints apiObj,
      ffi.Pointer<wire_RefCellAudioTrackConstraints> wireObj) {
    wireObj.ref.ptr = FrbOpaque.share(apiObj).cast();
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class AudioTrackConstraintsApiWire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  AudioTrackConstraintsApiWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  AudioTrackConstraintsApiWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  WireSyncReturnStruct wire_audio_track_constraints_new() {
    return _wire_audio_track_constraints_new();
  }

  late final _wire_audio_track_constraints_newPtr =
      _lookup<ffi.NativeFunction<WireSyncReturnStruct Function()>>(
          'wire_audio_track_constraints_new');
  late final _wire_audio_track_constraints_new =
      _wire_audio_track_constraints_newPtr
          .asFunction<WireSyncReturnStruct Function()>();

  WireSyncReturnStruct wire_audio_track_constraints_device_id(
    ffi.Pointer<wire_RefCellAudioTrackConstraints> track,
    ffi.Pointer<wire_uint_8_list> device_id,
  ) {
    return _wire_audio_track_constraints_device_id(
      track,
      device_id,
    );
  }

  late final _wire_audio_track_constraints_device_idPtr = _lookup<
          ffi.NativeFunction<
              WireSyncReturnStruct Function(
                  ffi.Pointer<wire_RefCellAudioTrackConstraints>,
                  ffi.Pointer<wire_uint_8_list>)>>(
      'wire_audio_track_constraints_device_id');
  late final _wire_audio_track_constraints_device_id =
      _wire_audio_track_constraints_device_idPtr.asFunction<
          WireSyncReturnStruct Function(
              ffi.Pointer<wire_RefCellAudioTrackConstraints>,
              ffi.Pointer<wire_uint_8_list>)>();

  ffi.Pointer<wire_RefCellAudioTrackConstraints>
      new_RefCellAudioTrackConstraints() {
    return _new_RefCellAudioTrackConstraints();
  }

  late final _new_RefCellAudioTrackConstraintsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_RefCellAudioTrackConstraints>
              Function()>>('new_RefCellAudioTrackConstraints');
  late final _new_RefCellAudioTrackConstraints =
      _new_RefCellAudioTrackConstraintsPtr.asFunction<
          ffi.Pointer<wire_RefCellAudioTrackConstraints> Function()>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_1(
    int len,
  ) {
    return _new_uint_8_list_1(
      len,
    );
  }

  late final _new_uint_8_list_1Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list_1');
  late final _new_uint_8_list_1 = _new_uint_8_list_1Ptr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>(
          'free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct = _free_WireSyncReturnStructPtr
      .asFunction<void Function(WireSyncReturnStruct)>();
}

class wire_RefCellAudioTrackConstraints extends ffi.Struct {
  external ffi.Pointer<ffi.Void> ptr;
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
