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
// Generated by `flutter_rust_bridge`@ 1.49.0.

use crate::api::dart::media_stream_settings_api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::sync::Mutex;
use std::sync::RwLock;

// Section: imports

// Section: wire functions

fn wire_media_stream_settings_new_impl() -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_stream_settings_new",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || Ok(media_stream_settings_new()),
    )
}
fn wire_media_stream_settings_cast_impl(
    media_stream_settings: impl Wire2Api<Opaque<RefCell<MediaStreamSettings>>>
        + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_stream_settings_cast",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_media_stream_settings = media_stream_settings.wire2api();
            Ok(media_stream_settings_cast(api_media_stream_settings))
        },
    )
}
fn wire_media_stream_settings_audio_impl(
    media_stream_settings: impl Wire2Api<Opaque<RefCell<MediaStreamSettings>>>
        + UnwindSafe,
    constraints: impl Wire2Api<Opaque<AudioTrackConstraints>> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_stream_settings_audio",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_media_stream_settings = media_stream_settings.wire2api();
            let api_constraints = constraints.wire2api();
            Ok(media_stream_settings_audio(
                api_media_stream_settings,
                api_constraints,
            ))
        },
    )
}
fn wire_media_stream_settings_device_video_impl(
    media_stream_settings: impl Wire2Api<Opaque<RefCell<MediaStreamSettings>>>
        + UnwindSafe,
    constraints: impl Wire2Api<Opaque<DeviceVideoTrackConstraints>> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_stream_settings_device_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_media_stream_settings = media_stream_settings.wire2api();
            let api_constraints = constraints.wire2api();
            Ok(media_stream_settings_device_video(
                api_media_stream_settings,
                api_constraints,
            ))
        },
    )
}
fn wire_media_stream_settings_display_video_impl(
    media_stream_settings: impl Wire2Api<Opaque<RefCell<MediaStreamSettings>>>
        + UnwindSafe,
    constraints: impl Wire2Api<Opaque<DisplayVideoTrackConstraints>> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_stream_settings_display_video",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_media_stream_settings = media_stream_settings.wire2api();
            let api_constraints = constraints.wire2api();
            Ok(media_stream_settings_display_video(
                api_media_stream_settings,
                api_constraints,
            ))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: deallocate functions

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

// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "media_stream_settings_api_g.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
