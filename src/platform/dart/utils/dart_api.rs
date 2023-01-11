//! Functionality for calling [`Dart DL API`] from Rust.
//!
//! [`Dart DL API`]: https://tinyurl.com/32e7fudh

use std::{ffi::c_void, os::raw::c_char, ptr};

use xayn_dart_api_dl::initialize_dart_api_dl;
use xayn_dart_api_dl_sys::{
    Dart_CObject, Dart_DeletePersistentHandle_DL, Dart_FinalizableHandle,
    Dart_GetError_DL, Dart_Handle, Dart_HandleFromPersistent_DL,
    Dart_IsError_DL, Dart_NewFinalizableHandle_DL, Dart_NewPersistentHandle_DL,
    Dart_PersistentHandle, Dart_Port, Dart_PostCObject_DL,
    Dart_PropagateError_DL,
};

/// Allocates a [`Dart_PersistentHandle`] for provided [`Dart_Handle`].
///
/// [`Dart_PersistentHandle`]s have the lifetime of the current isolate
/// unless they are explicitly deallocated.
pub fn Dart_NewPersistentHandle_DL_Trampolined(
    object: Dart_Handle,
) -> Dart_PersistentHandle {
    unsafe { Dart_NewPersistentHandle_DL.unwrap()(object) }
}

/// Allocates a [`Dart_Handle`] in the current scope from the given
/// [`Dart_PersistentHandle`].
///
/// This doesn't affect the provided [`Dart_PersistentHandle`]'s lifetime.
pub fn Dart_HandleFromPersistent_DL_Trampolined(
    object: Dart_PersistentHandle,
) -> Dart_Handle {
    unsafe { Dart_HandleFromPersistent_DL.unwrap()(object) }
}

/// Deallocates the provided [`Dart_PersistentHandle`].
pub fn Dart_DeletePersistentHandle_DL_Trampolined(
    object: Dart_PersistentHandle,
) {
    unsafe { Dart_DeletePersistentHandle_DL.unwrap()(object) }
}

/// Posts a `message` on some port. It will contain a [`Dart_CObject`]
/// object graph rooted in the `message`.
///
/// While the `message` is being sent the state of the graph of
/// [`Dart_CObject`] structures rooted in the `message` should not be
/// accessed, as the message generation will make temporary modifications to
/// the data. When the message has been sent the graph will be fully
/// restored.
///
/// If `true` is returned, the `message` was enqueued, and finalizers for
/// external typed data will eventually run, even if the receiving isolate
/// shuts down before processing the `message`. If `false` is returned, the
/// `message` was not enqueued and ownership of external typed data in the
/// `message` remains with the caller.
pub fn Dart_PostCObject_DL_Trampolined(
    port_id: Dart_Port,
    message: *mut Dart_CObject,
) -> bool {
    unsafe { Dart_PostCObject_DL.unwrap()(port_id, message) }
}

/// Allocates a finalizable handle for an object.
///
/// This handle has the lifetime of the current isolate group unless the
/// object pointed to by the handle is garbage collected, in this case the
/// VM automatically deletes the handle after invoking the callback
/// associated with the handle.
///
/// Once finalizable handle is collected by GC, the provided `callback` is
/// called.
///
/// `peer` argument will be provided to the `callback` on finalize.
///
/// `external_allocation_size` is a size of the `peer` which can be
/// calculated via [`mem::size_of()`].
///
/// [`mem::size_of()`]: std::mem::size_of
pub fn Dart_NewFinalizableHandle_DL_Trampolined(
    object: Dart_Handle,
    peer: *mut c_void,
    external_allocation_size: libc::intptr_t,
    callback: extern "C" fn(*mut c_void, *mut c_void),
) -> Dart_FinalizableHandle {
    unsafe {
        Dart_NewFinalizableHandle_DL.unwrap()(
            object,
            peer,
            external_allocation_size,
            Some(callback),
        )
    }
}

/// Checks whether the provided [`Dart_Handle`] represents a Dart error.
///
/// Should be called on the current isolate.
pub fn Dart_IsError_DL_Trampolined(object: Dart_Handle) -> bool {
    unsafe { Dart_IsError_DL.unwrap()(object) }
}

/// Returns the error message from the provided Dart error handle.
///
/// Should be called on the current isolate.
///
/// Returns a C string containing a Dart error message if the provided
/// `object` represents a Dart error, or an empty C string ("") otherwise.
pub fn Dart_GetError_DL_Trampolined(
    object: Dart_Handle,
) -> ptr::NonNull<c_char> {
    unsafe {
        let a: *mut c_char = Dart_GetError_DL.unwrap()(object) as _;
        ptr::NonNull::from(a.as_mut().unwrap())
    }
}

/// Propagates the given Dart error to the Dart side.
///
/// If the provided [`Dart_Handle`] is an unhandled exception error, then it
/// will be rethrown in the standard way: walking up Dart frames until an
/// appropriate `catch` block is found, than executing `finally` blocks, and
/// so on.
pub fn Dart_PropagateError_DL_Trampolined(object: Dart_Handle) {
    unsafe {
        Dart_PropagateError_DL.unwrap()(object);
    }
}

/// Initializes usage of Dynamically Linked Dart API.
///
/// # Safety
///
/// Intended to be called ONLY with [`NativeApi.initializeApiDLData`][1] from
/// Dart.
///
/// [1]: https://api.dart.dev/dart-ffi/NativeApi/initializeApiDLData.html
#[no_mangle]
pub unsafe extern "C" fn init_dart_api_dl(obj: ptr::NonNull<c_void>) -> bool {
    initialize_dart_api_dl(obj.as_ptr()).is_ok()
}
