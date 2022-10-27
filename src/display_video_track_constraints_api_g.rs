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

use crate::api::dart::display_video_track_constraints_api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::sync::Mutex;
use std::sync::RwLock;

// Section: imports

// Section: wire functions

fn wire_display_video_track_constraints_new_impl(
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "display_video_track_constraints_new",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || Ok(display_video_track_constraints_new()),
    )
}
fn wire_display_video_track_constraints_device_id_impl(
    constraints: impl Wire2Api<Opaque<RefCell<DisplayVideoTrackConstraints>>>
        + UnwindSafe,
    device_id: impl Wire2Api<String> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "display_video_track_constraints_device_id",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_constraints = constraints.wire2api();
            let api_device_id = device_id.wire2api();
            Ok(display_video_track_constraints_device_id(
                api_constraints,
                api_device_id,
            ))
        },
    )
}
fn wire_display_video_track_constraints_exact_height_impl(
    constraints: impl Wire2Api<Opaque<RefCell<DisplayVideoTrackConstraints>>>
        + UnwindSafe,
    exact_height: impl Wire2Api<u32> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "display_video_track_constraints_exact_height",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_constraints = constraints.wire2api();
            let api_exact_height = exact_height.wire2api();
            Ok(display_video_track_constraints_exact_height(
                api_constraints,
                api_exact_height,
            ))
        },
    )
}
fn wire_display_video_track_constraints_ideal_height_impl(
    constraints: impl Wire2Api<Opaque<RefCell<DisplayVideoTrackConstraints>>>
        + UnwindSafe,
    ideal_height: impl Wire2Api<u32> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "display_video_track_constraints_ideal_height",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_constraints = constraints.wire2api();
            let api_ideal_height = ideal_height.wire2api();
            Ok(display_video_track_constraints_ideal_height(
                api_constraints,
                api_ideal_height,
            ))
        },
    )
}
fn wire_display_video_track_constraints_exact_width_impl(
    constraints: impl Wire2Api<Opaque<RefCell<DisplayVideoTrackConstraints>>>
        + UnwindSafe,
    exact_width: impl Wire2Api<u32> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "display_video_track_constraints_exact_width",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_constraints = constraints.wire2api();
            let api_exact_width = exact_width.wire2api();
            Ok(display_video_track_constraints_exact_width(
                api_constraints,
                api_exact_width,
            ))
        },
    )
}
fn wire_display_video_track_constraints_ideal_width_impl(
    constraints: impl Wire2Api<Opaque<RefCell<DisplayVideoTrackConstraints>>>
        + UnwindSafe,
    ideal_width: impl Wire2Api<u32> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "display_video_track_constraints_ideal_width",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_constraints = constraints.wire2api();
            let api_ideal_width = ideal_width.wire2api();
            Ok(display_video_track_constraints_ideal_width(
                api_constraints,
                api_ideal_width,
            ))
        },
    )
}
fn wire_display_video_track_constraints_ideal_frame_rate_impl(
    constraints: impl Wire2Api<Opaque<RefCell<DisplayVideoTrackConstraints>>>
        + UnwindSafe,
    ideal_frame_rate: impl Wire2Api<u32> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "display_video_track_constraints_ideal_frame_rate",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_constraints = constraints.wire2api();
            let api_ideal_frame_rate = ideal_frame_rate.wire2api();
            Ok(display_video_track_constraints_ideal_frame_rate(
                api_constraints,
                api_ideal_frame_rate,
            ))
        },
    )
}
fn wire_display_video_track_constraints_exact_frame_rate_impl(
    constraints: impl Wire2Api<Opaque<RefCell<DisplayVideoTrackConstraints>>>
        + UnwindSafe,
    exact_frame_rate: impl Wire2Api<u32> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "display_video_track_constraints_exact_frame_rate",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_constraints = constraints.wire2api();
            let api_exact_frame_rate = exact_frame_rate.wire2api();
            Ok(display_video_track_constraints_exact_frame_rate(
                api_constraints,
                api_exact_frame_rate,
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

// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "display_video_track_constraints_api_g.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
