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

use crate::api::dart::media_display_info_api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::sync::Mutex;
use std::sync::RwLock;

// Section: imports

// Section: wire functions

fn wire_media_display_info_device_id_impl(
    media_display: impl Wire2Api<Opaque<MediaDisplayInfo>> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_display_info_device_id",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_media_display = media_display.wire2api();
            Ok(media_display_info_device_id(api_media_display))
        },
    )
}
fn wire_media_display_info_title_impl(
    media_display: impl Wire2Api<Opaque<MediaDisplayInfo>> + UnwindSafe,
) -> support::WireSyncReturnStruct {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "media_display_info_title",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_media_display = media_display.wire2api();
            Ok(media_display_info_title(api_media_display))
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
#[path = "media_display_info_api_g.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
