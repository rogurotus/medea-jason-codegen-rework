use flutter_rust_bridge::DartOpaque;
use xayn_dart_api_dl_sys::Dart_Handle;

use crate::{
    api::ForeignClass,
    platform::utils::dart_api::Dart_NewPersistentHandle_DL_Trampolined,
};

/// Creates a new [`DartOpaque`].
pub unsafe fn new_dart_opaque(handle: Dart_Handle) -> DartOpaque {
    DartOpaque::new_non_droppable(
        Dart_NewPersistentHandle_DL_Trampolined(handle).cast(),
    )
}

impl<T> ForeignClass for Vec<T> {}
