use std::ffi::{c_char, c_void, CString};

use crate::stopwatch::{self, format_time, LapTime, Stopwatch};

/// A buffer that stores `LapTime` values for a stopwatch.
#[repr(C)]
pub struct LapTimeBuffer {
    /// A pointer to the data stored in the buffer.
    data: *const c_void,
    /// The length of the buffer.
    len: usize,
}

/// A constructor that creates a new Stopwatch with default values.
#[no_mangle]
pub extern "C" fn stopwatch_Stopwatch_create() -> *mut c_void {
    Box::into_raw(Box::new(Stopwatch::new())).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer and frees the object
#[no_mangle]
pub unsafe extern "C" fn stopwatch_Stopwatch_free(stopwatch: *mut c_void) {
    drop(Box::from_raw(stopwatch.cast::<stopwatch::Stopwatch>()));
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_Stopwatch_start(stopwatch: *mut c_void) {
    (*stopwatch.cast::<Stopwatch>()).start();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_Stopwatch_stop(stopwatch: *mut c_void) {
    (*stopwatch.cast::<Stopwatch>()).stop();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_Stopwatch_reset(stopwatch: *mut c_void) {
    (*stopwatch.cast::<Stopwatch>()).reset();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_Stopwatch_addLap(stopwatch: *mut c_void) {
    (*stopwatch.cast::<stopwatch::Stopwatch>()).add_lap();
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_Stopwatch_isRunning(stopwatch: *mut c_void) -> bool {
    (*stopwatch.cast::<stopwatch::Stopwatch>()).is_running()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_Stopwatch_elapsedTime(stopwatch: *mut c_void) -> *mut c_void {
    let duration = (*stopwatch.cast::<Stopwatch>()).elapsed_time();

    Box::into_raw(Box::new(duration)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_Stopwatch_lapTimes(stopwatch: *mut c_void) -> LapTimeBuffer {
    let buf = (*stopwatch.cast::<Stopwatch>()).lap_times();
    let len = buf.len();
    let data = buf.as_ptr().cast::<std::ffi::c_void>();
    LapTimeBuffer { data, len }
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_Stopwatch_freeLapTimes(buf: LapTimeBuffer) {
    let LapTimeBuffer { data, len } = buf;
    unsafe {
        let _ = std::slice::from_raw_parts(data, len);
    }
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_LapTime_getIndex(
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
pub unsafe extern "C" fn stopwatch_LapTime_lapNumber(laptime: *mut c_void) -> usize {
    (*laptime.cast::<stopwatch::LapTime>()).lap_number()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_LapTime_splitTime(laptime: *mut c_void) -> *mut c_void {
    let duration = (*laptime.cast::<stopwatch::LapTime>()).split_time();
    Box::into_raw(Box::new(duration)).cast::<std::ffi::c_void>()
}

/// # Safety
///
/// This function dereferences a raw pointer
#[no_mangle]
pub unsafe extern "C" fn stopwatch_LapTime_totalTime(laptime: *mut c_void) -> *mut c_void {
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
pub unsafe extern "C" fn stopwatch_LapTime_free(laptime: *mut c_void) {
    drop(Box::from_raw(laptime.cast::<stopwatch::LapTime>()));
}
