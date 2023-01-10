#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.58.2.

use crate::jason_api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::{ffi::c_void, sync::Arc};

// Section: imports

use crate::{
    api::dart::api_struct::{
        ApiAudioTrackConstrs, ApiConstrainFacingMode,
        ApiDeviceVideoTrackConstrs, ApiDisplayVideoTrackConstrs,
        ApiMediaDeviceInfo, ApiMediaDisplayInfo, ApiMediaStreamSettings,
        ApiOptionConstrainFacingMode, ApiOptionConstrainU32,
    },
    media::{
        constraints::{ConstrainU32, FacingMode},
        track::{remote::MediaDirection, MediaSourceKind},
        MediaDeviceKind, MediaKind,
    },
    room::RoomCloseReason,
};

// Section: wire functions

fn wire_connection_handle_from_ptr_impl(
    ptr: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "connection_handle_from_ptr",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_ptr = ptr.wire2api();
            Ok(connection_handle_from_ptr(api_ptr))
        },
    )
}
fn wire_connection_handle_on_close_impl(
    connection: impl Wire2Api<RustOpaque<ConnectionHandle>> + UnwindSafe,
    f: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "connection_handle_on_close",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_connection = connection.wire2api();
            let api_f = f.wire2api();
            connection_handle_on_close(api_connection, api_f)
        },
    )
}
fn wire_connection_handle_on_remote_track_added_impl(
    connection: impl Wire2Api<RustOpaque<ConnectionHandle>> + UnwindSafe,
    f: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "connection_handle_on_remote_track_added",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_connection = connection.wire2api();
            let api_f = f.wire2api();
            connection_handle_on_remote_track_added(api_connection, api_f)
        },
    )
}
fn wire_connection_handle_on_quality_score_update_impl(
    connection: impl Wire2Api<RustOpaque<ConnectionHandle>> + UnwindSafe,
    f: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "connection_handle_on_quality_score_update",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_connection = connection.wire2api();
            let api_f = f.wire2api();
            connection_handle_on_quality_score_update(api_connection, api_f)
        },
    )
}
fn wire_connection_handle_get_remote_member_id_impl(
    connection: impl Wire2Api<RustOpaque<ConnectionHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "connection_handle_get_remote_member_id",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_connection = connection.wire2api();
            connection_handle_get_remote_member_id(api_connection)
        },
    )
}
fn wire_connection_handle_enable_remote_audio_impl(
    connection: impl Wire2Api<RustOpaque<ConnectionHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "connection_handle_enable_remote_audio",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_connection = connection.wire2api();
            Ok(connection_handle_enable_remote_audio(api_connection))
        },
    )
}
fn wire_connection_handle_disable_remote_audio_impl(
    connection: impl Wire2Api<RustOpaque<ConnectionHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "connection_handle_disable_remote_audio",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_connection = connection.wire2api();
            Ok(connection_handle_disable_remote_audio(api_connection))
        },
    )
}
fn wire_connection_handle_enable_remote_video_impl(
    connection: impl Wire2Api<RustOpaque<ConnectionHandle>> + UnwindSafe,
    source_kind: impl Wire2Api<Option<i64>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "connection_handle_enable_remote_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_connection = connection.wire2api();
            let api_source_kind = source_kind.wire2api();
            Ok(connection_handle_enable_remote_video(
                api_connection,
                api_source_kind,
            ))
        },
    )
}
fn wire_connection_handle_disable_remote_video_impl(
    connection: impl Wire2Api<RustOpaque<ConnectionHandle>> + UnwindSafe,
    source_kind: impl Wire2Api<Option<i64>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "connection_handle_disable_remote_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_connection = connection.wire2api();
            let api_source_kind = source_kind.wire2api();
            Ok(connection_handle_disable_remote_video(
                api_connection,
                api_source_kind,
            ))
        },
    )
}
fn wire_on_panic_impl(
    cb: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "on_panic",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_cb = cb.wire2api();
            Ok(on_panic(api_cb))
        },
    )
}
fn wire_jason_new_impl() -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "jason_new",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || Ok(jason_new()),
    )
}
fn wire_jason_init_room_impl(
    jason: impl Wire2Api<RustOpaque<Jason>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "jason_init_room",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_jason = jason.wire2api();
            Ok(jason_init_room(api_jason))
        },
    )
}
fn wire_jason_media_manager_impl(
    jason: impl Wire2Api<RustOpaque<Jason>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "jason_media_manager",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_jason = jason.wire2api();
            Ok(jason_media_manager(api_jason))
        },
    )
}
fn wire_jason_close_room_impl(
    jason: impl Wire2Api<RustOpaque<Jason>> + UnwindSafe,
    room_to_delete: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "jason_close_room",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_jason = jason.wire2api();
            let api_room_to_delete = room_to_delete.wire2api();
            Ok(jason_close_room(api_jason, api_room_to_delete))
        },
    )
}
fn wire_jason_dispose_impl(
    jason: impl Wire2Api<RustOpaque<Jason>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "jason_dispose",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_jason = jason.wire2api();
            Ok(jason_dispose(api_jason))
        },
    )
}
fn wire_local_media_track_from_ptr_impl(
    ptr: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "local_media_track_from_ptr",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_ptr = ptr.wire2api();
            Ok(local_media_track_from_ptr(api_ptr))
        },
    )
}
fn wire_vec_local_tracks_from_ptr_impl(
    ptr: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "vec_local_tracks_from_ptr",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_ptr = ptr.wire2api();
            Ok(vec_local_tracks_from_ptr(api_ptr))
        },
    )
}
fn wire_local_media_track_get_track_impl(
    track: impl Wire2Api<RustOpaque<LocalMediaTrack>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "local_media_track_get_track",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            Ok(local_media_track_get_track(api_track))
        },
    )
}
fn wire_local_media_track_kind_impl(
    track: impl Wire2Api<RustOpaque<LocalMediaTrack>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "local_media_track_kind",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            Ok(local_media_track_kind(api_track))
        },
    )
}
fn wire_local_media_track_media_source_kind_impl(
    track: impl Wire2Api<RustOpaque<LocalMediaTrack>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "local_media_track_media_source_kind",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            Ok(local_media_track_media_source_kind(api_track))
        },
    )
}
fn wire_vec_media_device_info_from_ptr_impl(
    ptr: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "vec_media_device_info_from_ptr",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_ptr = ptr.wire2api();
            Ok(vec_media_device_info_from_ptr(api_ptr))
        },
    )
}
fn wire_vec_media_display_info_from_ptr_impl(
    ptr: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "vec_media_display_info_from_ptr",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_ptr = ptr.wire2api();
            Ok(vec_media_display_info_from_ptr(api_ptr))
        },
    )
}
fn wire_media_manager_handle_init_local_tracks_impl(
    manager: impl Wire2Api<RustOpaque<MediaManagerHandle>> + UnwindSafe,
    caps: impl Wire2Api<ApiMediaStreamSettings> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_manager_handle_init_local_tracks",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_manager = manager.wire2api();
            let api_caps = caps.wire2api();
            Ok(media_manager_handle_init_local_tracks(
                api_manager,
                api_caps,
            ))
        },
    )
}
fn wire_media_manager_handle_enumerate_devices_impl(
    manager: impl Wire2Api<RustOpaque<MediaManagerHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_manager_handle_enumerate_devices",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_manager = manager.wire2api();
            Ok(media_manager_handle_enumerate_devices(api_manager))
        },
    )
}
fn wire_media_manager_handle_enumerate_displays_impl(
    manager: impl Wire2Api<RustOpaque<MediaManagerHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_manager_handle_enumerate_displays",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_manager = manager.wire2api();
            Ok(media_manager_handle_enumerate_displays(api_manager))
        },
    )
}
fn wire_media_manager_handle_set_output_audio_id_impl(
    manager: impl Wire2Api<RustOpaque<MediaManagerHandle>> + UnwindSafe,
    device_id: impl Wire2Api<String> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_manager_handle_set_output_audio_id",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_manager = manager.wire2api();
            let api_device_id = device_id.wire2api();
            Ok(media_manager_handle_set_output_audio_id(
                api_manager,
                api_device_id,
            ))
        },
    )
}
fn wire_media_manager_handle_set_microphone_volume_impl(
    manager: impl Wire2Api<RustOpaque<MediaManagerHandle>> + UnwindSafe,
    level: impl Wire2Api<i64> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_manager_handle_set_microphone_volume",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_manager = manager.wire2api();
            let api_level = level.wire2api();
            Ok(media_manager_handle_set_microphone_volume(
                api_manager,
                api_level,
            ))
        },
    )
}
fn wire_media_manager_handle_microphone_volume_is_available_impl(
    manager: impl Wire2Api<RustOpaque<MediaManagerHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_manager_handle_microphone_volume_is_available",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_manager = manager.wire2api();
            Ok(media_manager_handle_microphone_volume_is_available(
                api_manager,
            ))
        },
    )
}
fn wire_media_manager_handle_microphone_volume_impl(
    manager: impl Wire2Api<RustOpaque<MediaManagerHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_manager_handle_microphone_volume",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_manager = manager.wire2api();
            Ok(media_manager_handle_microphone_volume(api_manager))
        },
    )
}
fn wire_media_manager_handle_on_device_change_impl(
    manager: impl Wire2Api<RustOpaque<MediaManagerHandle>> + UnwindSafe,
    cb: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_manager_handle_on_device_change",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_manager = manager.wire2api();
            let api_cb = cb.wire2api();
            media_manager_handle_on_device_change(api_manager, api_cb)
        },
    )
}
fn wire_reconnect_handle_from_ptr_impl(
    ptr: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "reconnect_handle_from_ptr",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_ptr = ptr.wire2api();
            Ok(reconnect_handle_from_ptr(api_ptr))
        },
    )
}
fn wire_reconnect_handle_reconnect_with_delay_impl(
    reconnect_handle: impl Wire2Api<RustOpaque<ReconnectHandle>> + UnwindSafe,
    delay_ms: impl Wire2Api<i64> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "reconnect_handle_reconnect_with_delay",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_reconnect_handle = reconnect_handle.wire2api();
            let api_delay_ms = delay_ms.wire2api();
            Ok(reconnect_handle_reconnect_with_delay(
                api_reconnect_handle,
                api_delay_ms,
            ))
        },
    )
}
fn wire_reconnect_handle_reconnect_with_backoff_impl(
    reconnect_handle: impl Wire2Api<RustOpaque<ReconnectHandle>> + UnwindSafe,
    starting_delay: impl Wire2Api<i64> + UnwindSafe,
    multiplier: impl Wire2Api<f64> + UnwindSafe,
    max_delay: impl Wire2Api<i64> + UnwindSafe,
    max_elapsed_time_ms: impl Wire2Api<Option<i64>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "reconnect_handle_reconnect_with_backoff",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_reconnect_handle = reconnect_handle.wire2api();
            let api_starting_delay = starting_delay.wire2api();
            let api_multiplier = multiplier.wire2api();
            let api_max_delay = max_delay.wire2api();
            let api_max_elapsed_time_ms = max_elapsed_time_ms.wire2api();
            Ok(reconnect_handle_reconnect_with_backoff(
                api_reconnect_handle,
                api_starting_delay,
                api_multiplier,
                api_max_delay,
                api_max_elapsed_time_ms,
            ))
        },
    )
}
fn wire_remote_media_track_from_ptr_impl(
    ptr: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_from_ptr",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_ptr = ptr.wire2api();
            Ok(remote_media_track_from_ptr(api_ptr))
        },
    )
}
fn wire_remote_media_track_get_track_impl(
    track: impl Wire2Api<RustOpaque<RemoteMediaTrack>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_get_track",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            Ok(remote_media_track_get_track(api_track))
        },
    )
}
fn wire_remote_media_track_on_muted_impl(
    track: impl Wire2Api<RustOpaque<RemoteMediaTrack>> + UnwindSafe,
    f: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_on_muted",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            let api_f = f.wire2api();
            Ok(remote_media_track_on_muted(api_track, api_f))
        },
    )
}
fn wire_remote_media_track_on_unmuted_impl(
    track: impl Wire2Api<RustOpaque<RemoteMediaTrack>> + UnwindSafe,
    f: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_on_unmuted",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            let api_f = f.wire2api();
            Ok(remote_media_track_on_unmuted(api_track, api_f))
        },
    )
}
fn wire_remote_media_track_on_stopped_impl(
    track: impl Wire2Api<RustOpaque<RemoteMediaTrack>> + UnwindSafe,
    f: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_on_stopped",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            let api_f = f.wire2api();
            Ok(remote_media_track_on_stopped(api_track, api_f))
        },
    )
}
fn wire_remote_media_track_on_media_direction_changed_impl(
    track: impl Wire2Api<RustOpaque<RemoteMediaTrack>> + UnwindSafe,
    f: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_on_media_direction_changed",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            let api_f = f.wire2api();
            Ok(remote_media_track_on_media_direction_changed(
                api_track, api_f,
            ))
        },
    )
}
fn wire_remote_media_track_muted_impl(
    track: impl Wire2Api<RustOpaque<RemoteMediaTrack>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_muted",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            Ok(remote_media_track_muted(api_track))
        },
    )
}
fn wire_remote_media_track_kind_impl(
    track: impl Wire2Api<RustOpaque<RemoteMediaTrack>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_kind",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            Ok(remote_media_track_kind(api_track))
        },
    )
}
fn wire_remote_media_track_media_source_kind_impl(
    track: impl Wire2Api<RustOpaque<RemoteMediaTrack>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_media_source_kind",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            Ok(remote_media_track_media_source_kind(api_track))
        },
    )
}
fn wire_remote_media_track_media_direction_impl(
    track: impl Wire2Api<RustOpaque<RemoteMediaTrack>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remote_media_track_media_direction",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_track = track.wire2api();
            Ok(remote_media_track_media_direction(api_track))
        },
    )
}
fn wire_room_close_reason_from_ptr_impl(
    ptr: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_close_reason_from_ptr",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_ptr = ptr.wire2api();
            Ok(room_close_reason_from_ptr(api_ptr))
        },
    )
}
fn wire_room_handle_join_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    token: impl Wire2Api<String> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_join",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_token = token.wire2api();
            Ok(room_handle_join(api_room_handle, api_token))
        },
    )
}
fn wire_room_handle_set_local_media_settings_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    settings: impl Wire2Api<ApiMediaStreamSettings> + UnwindSafe,
    stop_first: impl Wire2Api<bool> + UnwindSafe,
    rollback_on_fail: impl Wire2Api<bool> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_set_local_media_settings",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_settings = settings.wire2api();
            let api_stop_first = stop_first.wire2api();
            let api_rollback_on_fail = rollback_on_fail.wire2api();
            Ok(room_handle_set_local_media_settings(
                api_room_handle,
                api_settings,
                api_stop_first,
                api_rollback_on_fail,
            ))
        },
    )
}
fn wire_room_handle_mute_audio_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_mute_audio",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            Ok(room_handle_mute_audio(api_room_handle))
        },
    )
}
fn wire_room_handle_unmute_audio_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_unmute_audio",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            Ok(room_handle_unmute_audio(api_room_handle))
        },
    )
}
fn wire_room_handle_enable_audio_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_enable_audio",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            Ok(room_handle_enable_audio(api_room_handle))
        },
    )
}
fn wire_room_handle_disable_audio_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_disable_audio",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            Ok(room_handle_disable_audio(api_room_handle))
        },
    )
}
fn wire_room_handle_mute_video_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    source_kind: impl Wire2Api<Option<i64>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_mute_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_source_kind = source_kind.wire2api();
            room_handle_mute_video(api_room_handle, api_source_kind)
        },
    )
}
fn wire_room_handle_unmute_video_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    source_kind: impl Wire2Api<Option<i64>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_unmute_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_source_kind = source_kind.wire2api();
            room_handle_unmute_video(api_room_handle, api_source_kind)
        },
    )
}
fn wire_room_handle_enable_video_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    source_kind: impl Wire2Api<Option<i64>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_enable_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_source_kind = source_kind.wire2api();
            room_handle_enable_video(api_room_handle, api_source_kind)
        },
    )
}
fn wire_room_handle_disable_video_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    source_kind: impl Wire2Api<Option<i64>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_disable_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_source_kind = source_kind.wire2api();
            room_handle_disable_video(api_room_handle, api_source_kind)
        },
    )
}
fn wire_room_handle_enable_remote_audio_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_enable_remote_audio",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            Ok(room_handle_enable_remote_audio(api_room_handle))
        },
    )
}
fn wire_room_handle_disable_remote_audio_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_disable_remote_audio",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            Ok(room_handle_disable_remote_audio(api_room_handle))
        },
    )
}
fn wire_room_handle_enable_remote_video_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    source_kind: impl Wire2Api<Option<i64>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_enable_remote_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_source_kind = source_kind.wire2api();
            room_handle_enable_remote_video(api_room_handle, api_source_kind)
        },
    )
}
fn wire_room_handle_disable_remote_video_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    source_kind: impl Wire2Api<Option<i64>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_disable_remote_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_source_kind = source_kind.wire2api();
            room_handle_disable_remote_video(api_room_handle, api_source_kind)
        },
    )
}
fn wire_room_handle_on_new_connection_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    cb: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_on_new_connection",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_cb = cb.wire2api();
            room_handle_on_new_connection(api_room_handle, api_cb)
        },
    )
}
fn wire_room_handle_on_close_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    cb: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_on_close",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_cb = cb.wire2api();
            room_handle_on_close(api_room_handle, api_cb)
        },
    )
}
fn wire_room_handle_on_local_track_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    cb: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_on_local_track",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_cb = cb.wire2api();
            room_handle_on_local_track(api_room_handle, api_cb)
        },
    )
}
fn wire_room_handle_on_connection_loss_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    cb: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_on_connection_loss",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_cb = cb.wire2api();
            room_handle_on_connection_loss(api_room_handle, api_cb)
        },
    )
}
fn wire_room_handle_on_failed_local_media_impl(
    room_handle: impl Wire2Api<RustOpaque<RoomHandle>> + UnwindSafe,
    cb: impl Wire2Api<DartOpaque> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "room_handle_on_failed_local_media",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_room_handle = room_handle.wire2api();
            let api_cb = cb.wire2api();
            room_handle_on_failed_local_media(api_room_handle, api_cb)
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}

impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}
impl Wire2Api<FacingMode> for i32 {
    fn wire2api(self) -> FacingMode {
        match self {
            0 => FacingMode::User,
            1 => FacingMode::Environment,
            2 => FacingMode::Left,
            3 => FacingMode::Right,
            _ => unreachable!("Invalid variant for FacingMode: {}", self),
        }
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<i64> for i64 {
    fn wire2api(self) -> i64 {
        self
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<usize> for usize {
    fn wire2api(self) -> usize {
        self
    }
}
// Section: impl IntoDart

impl support::IntoDart for ApiMediaDeviceInfo {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.kind.into_dart(),
            self.device_id.into_dart(),
            self.label.into_dart(),
            self.group_id.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ApiMediaDeviceInfo {}

impl support::IntoDart for ApiMediaDisplayInfo {
    fn into_dart(self) -> support::DartAbi {
        vec![self.device_id.into_dart(), self.title.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ApiMediaDisplayInfo {}

impl support::IntoDart for MediaDeviceKind {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::AudioInput => 0,
            Self::VideoInput => 1,
            Self::AudioOutput => 2,
        }
        .into_dart()
    }
}
impl support::IntoDart for MediaDirection {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::SendRecv => 0,
            Self::SendOnly => 1,
            Self::RecvOnly => 2,
            Self::Inactive => 3,
        }
        .into_dart()
    }
}
impl support::IntoDart for MediaKind {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Audio => 0,
            Self::Video => 1,
        }
        .into_dart()
    }
}
impl support::IntoDart for MediaSourceKind {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Device => 0,
            Self::Display => 1,
        }
        .into_dart()
    }
}

impl support::IntoDart for RoomCloseReason {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.is_closed_by_server.into_dart(),
            self.reason.into_dart(),
            self.is_err.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for RoomCloseReason {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler =
        Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "jason_api_g.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
