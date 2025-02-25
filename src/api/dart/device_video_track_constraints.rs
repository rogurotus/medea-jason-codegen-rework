use std::{os::raw::c_char, ptr};

use crate::media::FacingMode;

use super::{
    propagate_panic,
    utils::{c_str_into_string, ArgumentError, DartResult},
    ForeignClass,
};

pub use crate::media::DeviceVideoTrackConstraints;

impl ForeignClass for DeviceVideoTrackConstraints {}

/// Creates new [`DeviceVideoTrackConstraints`] with none constraints
/// configured.
#[no_mangle]
pub extern "C" fn DeviceVideoTrackConstraints__new(
) -> ptr::NonNull<DeviceVideoTrackConstraints> {
    propagate_panic(move || DeviceVideoTrackConstraints::new().into_ptr())
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__device_id(
    mut this: ptr::NonNull<DeviceVideoTrackConstraints>,
    device_id: ptr::NonNull<c_char>,
) {
    propagate_panic(move || {
        this.as_mut().device_id(c_str_into_string(device_id));
    });
}

/// Sets an exact [facingMode][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__exact_facing_mode(
    mut this: ptr::NonNull<DeviceVideoTrackConstraints>,
    facing_mode: FacingMode,
) {
    propagate_panic(move || {
        this.as_mut().exact_facing_mode(facing_mode);
    });
}

/// Sets an ideal [facingMode][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__ideal_facing_mode(
    mut this: ptr::NonNull<DeviceVideoTrackConstraints>,
    facing_mode: FacingMode,
) {
    propagate_panic(move || {
        this.as_mut().ideal_facing_mode(facing_mode);
    });
}

/// Sets an exact [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__exact_height(
    mut this: ptr::NonNull<DeviceVideoTrackConstraints>,
    height: i64,
) -> DartResult {
    propagate_panic(move || {
        let Ok(height) = u32::try_from(height) else {
            return ArgumentError::new(height, "height", "Expected u32")
                .into();
        };
        this.as_mut().exact_height(height);
        Ok(()).into()
    })
}

/// Sets an ideal [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__ideal_height(
    mut this: ptr::NonNull<DeviceVideoTrackConstraints>,
    height: i64,
) -> DartResult {
    propagate_panic(move || {
        let Ok(height) = u32::try_from(height) else {
            return ArgumentError::new(height, "height", "Expected u32")
                .into();
        };
        this.as_mut().ideal_height(height);
        Ok(()).into()
    })
}

/// Sets a range of a [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__height_in_range(
    mut this: ptr::NonNull<DeviceVideoTrackConstraints>,
    min: i64,
    max: i64,
) -> DartResult {
    propagate_panic(move || {
        match (u32::try_from(min), u32::try_from(max)) {
            (Ok(min), Ok(max)) => this.as_mut().height_in_range(min, max),
            (Err(_), _) => {
                return ArgumentError::new(min, "min", "Expected u32").into();
            }
            (_, Err(_)) => {
                return ArgumentError::new(max, "max", "Expected u32").into();
            }
        };
        Ok(()).into()
    })
}

/// Sets an exact [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__exact_width(
    mut this: ptr::NonNull<DeviceVideoTrackConstraints>,
    width: i64,
) -> DartResult {
    propagate_panic(move || {
        let Ok(width) = u32::try_from(width) else {
            return ArgumentError::new(width, "width", "Expected u32")
                .into();
        };
        this.as_mut().exact_width(width);
        Ok(()).into()
    })
}

/// Sets an ideal [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__ideal_width(
    mut this: ptr::NonNull<DeviceVideoTrackConstraints>,
    width: i64,
) -> DartResult {
    propagate_panic(|| {
        let Ok(width) = u32::try_from(width) else {
            return ArgumentError::new(width, "width", "Expected u32")
                .into();
        };
        this.as_mut().ideal_width(width);
        Ok(()).into()
    })
}

/// Sets a range of a [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__width_in_range(
    mut this: ptr::NonNull<DeviceVideoTrackConstraints>,
    min: i64,
    max: i64,
) -> DartResult {
    propagate_panic(move || {
        match (u32::try_from(min), u32::try_from(max)) {
            (Ok(min), Ok(max)) => this.as_mut().width_in_range(min, max),
            (Err(_), _) => {
                return ArgumentError::new(min, "min", "Expected u32").into();
            }
            (_, Err(_)) => {
                return ArgumentError::new(max, "max", "Expected u32").into();
            }
        };
        Ok(()).into()
    })
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn DeviceVideoTrackConstraints__free(
    this: ptr::NonNull<DeviceVideoTrackConstraints>,
) {
    propagate_panic(move || {
        drop(DeviceVideoTrackConstraints::from_ptr(this));
    });
}
