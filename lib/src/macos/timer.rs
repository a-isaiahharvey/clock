use std::{ffi::c_void, mem::ManuallyDrop, time::Duration};

use crate::timer::{self, Timer};

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub extern "C" fn timer_Timer_create(secs: u64) -> *mut c_void {
    let timer = Timer::new(Duration::from_secs(secs));
    Box::into_raw(Box::new(ManuallyDrop::new(timer))).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer and frees the object
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_free(timer: *mut c_void) {
    drop(Box::from_raw(timer.cast::<ManuallyDrop<timer::Timer>>()));
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_start(timer: *mut c_void) {
    (*timer.cast::<ManuallyDrop<timer::Timer>>()).start();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_stop(timer: *mut c_void) {
    (*timer.cast::<ManuallyDrop<timer::Timer>>()).stop();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_reset(timer: *mut c_void) {
    (*timer.cast::<ManuallyDrop<timer::Timer>>()).reset();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_elapsed(timer: *mut c_void) -> *mut c_void {
    let duration = (*timer.cast::<ManuallyDrop<timer::Timer>>()).elapsed();
    Box::into_raw(Box::new(duration)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_remaining(timer: *mut c_void) -> *mut c_void {
    let duration = (*timer.cast::<ManuallyDrop<timer::Timer>>()).remaining();
    Box::into_raw(Box::new(duration)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_duration(timer: *mut c_void) -> *mut c_void {
    let duration = (*timer.cast::<ManuallyDrop<timer::Timer>>()).duration();
    Box::into_raw(Box::new(duration)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_isRunning(timer: *mut c_void) -> bool {
    (*timer.cast::<ManuallyDrop<timer::Timer>>()).is_running()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_isDone(timer: *mut c_void) -> bool {
    (*timer.cast::<ManuallyDrop<timer::Timer>>()).is_done()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn timer_Timer_hasNotStarted(timer: *mut c_void) -> bool {
    (*timer.cast::<ManuallyDrop<timer::Timer>>()).has_not_started()
}
