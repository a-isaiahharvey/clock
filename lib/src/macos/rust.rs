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
pub unsafe extern "C" fn rust_Duration_asSecs(duration: *mut c_void) -> u64 {
    (*duration.cast::<std::time::Duration>()).as_secs()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn rust_Duration_asSecsF64(duration: *mut c_void) -> f64 {
    (*duration.cast::<std::time::Duration>()).as_secs_f64()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn rust_Duration_isZero(duration: *mut c_void) -> bool {
    (*duration.cast::<std::time::Duration>()).is_zero()
}
