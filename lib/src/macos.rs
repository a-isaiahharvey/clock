//! macOS bindings

#![allow(non_snake_case)]

use std::ffi::{c_char, c_void, CString};

use crate::stopwatch::{self, format_time, LapTime, Stopwatch};

/// A buffer that stores `LapTime` values for a stopwatch.
#[repr(C)]
pub struct LapTimeBuffer {
    data: *const c_void,
    len: usize,
}

/// A constructor that creates a new Stopwatch with default values.
#[no_mangle]
pub extern "C" fn stopwatch_stopwatch_create() -> *mut c_void {
    let stopwatch = Stopwatch::new();
    Box::into_raw(Box::new(stopwatch)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer and frees the object
#[no_mangle]
pub unsafe extern "C" fn stopwatch_stopwatch_free(stopwatch: *mut c_void) {
    drop(Box::from_raw(stopwatch.cast::<stopwatch::Stopwatch>()));
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_stopwatch_start(stopwatch: *mut c_void) {
    (*stopwatch.cast::<stopwatch::Stopwatch>()).start();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_stopwatch_stop(stopwatch: *mut c_void) {
    (*stopwatch.cast::<stopwatch::Stopwatch>()).stop();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_stopwatch_reset(stopwatch: *mut c_void) {
    (*stopwatch.cast::<stopwatch::Stopwatch>()).reset();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_stopwatch_addLap(stopwatch: *mut c_void) {
    (*stopwatch.cast::<stopwatch::Stopwatch>()).add_lap();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_stopwatch_isRunning(stopwatch: *mut c_void) -> bool {
    (*stopwatch.cast::<stopwatch::Stopwatch>()).is_running()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_stopwatch_elapsedTime(stopwatch: *mut c_void) -> *mut c_void {
    let duration = (*stopwatch.cast::<stopwatch::Stopwatch>()).elapsed_time();

    Box::into_raw(Box::new(duration)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_stopwatch_lapTimes(stopwatch: *mut c_void) -> LapTimeBuffer {
    let buf = (*stopwatch.cast::<stopwatch::Stopwatch>()).lap_times();
    let len = buf.len();
    let data = buf
        .as_ptr()
        .cast::<stopwatch::LapTime>()
        .cast::<std::ffi::c_void>();
    LapTimeBuffer { data, len }
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_stopwatch_freeLapTimes(buf: LapTimeBuffer) {
    let LapTimeBuffer { data, len } = buf;
    unsafe {
        let _ = std::slice::from_raw_parts(data, len);
    }
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_laptime_getIndex(
    buf: LapTimeBuffer,
    index: usize,
) -> *mut c_void {
    let laptimes = unsafe { std::slice::from_raw_parts_mut(buf.data as *mut LapTime, buf.len) };
    let laptime = laptimes[index];
    Box::into_raw(Box::new(laptime)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_laptime_lapNumber(laptime: *mut c_void) -> u32 {
    (*laptime.cast::<stopwatch::LapTime>()).lap_number()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_laptime_splitTime(laptime: *mut c_void) -> *mut c_void {
    let duration = (*laptime.cast::<stopwatch::LapTime>()).split_time();
    Box::into_raw(Box::new(duration)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_laptime_totalTime(laptime: *mut c_void) -> *mut c_void {
    let duration = (*laptime.cast::<stopwatch::LapTime>()).total_time();
    Box::into_raw(Box::new(duration)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_formatTime(duration: *mut c_void) -> *const c_char {
    let time = format_time(*duration.cast::<std::time::Duration>());

    let c_str = CString::new(time).expect("Could not create C string from Rust string");
    c_str.into_raw()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_freeDuration(duration: *mut c_void) {
    drop(Box::from_raw(duration.cast::<std::time::Duration>()));
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_freeLapTime(laptime: *mut c_void) {
    drop(Box::from_raw(laptime.cast::<stopwatch::LapTime>()));
}
