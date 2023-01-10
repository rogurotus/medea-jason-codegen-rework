mod err;
mod result;
mod frb_adapter;

use std::{future::Future, marker::PhantomData};

use xayn_dart_api_dl_sys::Dart_Handle;

use crate::{
    api::DartValue,
    platform::{spawn, utils::Completer},
};

pub use self::{
    err::{new_panic_error, ArgumentError, DartError},
    frb_adapter::*,
};
