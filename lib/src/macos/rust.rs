use std::ffi::c_void;

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn rust_Duration_free(duration: *mut c_void) {
    drop(Box::from_raw(duration.cast::<std::time::Duration>()));
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn rust_Duration_asSecs(duration: *const c_void) -> u64 {
    let duration_ptr = duration.cast::<std::time::Duration>();
    (*duration_ptr).as_secs()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn rust_Duration_asSecsF64(duration: *const c_void) -> f64 {
    let duration_ptr = duration.cast::<std::time::Duration>();
    (*duration_ptr).as_secs_f64()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn rust_Duration_isZero(duration: *const c_void) -> bool {
    let duration_ptr = duration.cast::<std::time::Duration>();
    (*duration_ptr).is_zero()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn rust_Duration_eq(duration: *const c_void, other: *const c_void) -> bool {
    let duration_ptr = duration.cast::<std::time::Duration>();
    let other_ptr = other.cast::<std::time::Duration>();
    (*duration_ptr).eq(&*other_ptr)
}
