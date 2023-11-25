use core::ffi::{c_int, c_char, c_uint};


#[link(name = "xray")]
extern "C" {
    pub(crate) fn queryStats(serverAddr: *const c_char, timeout: c_int, pattern: *const c_char, reset: c_uint) -> *mut c_char;
    pub(crate) fn startFromJSON(jsonString: *const c_char);
    fn freeCString(ptr: *mut c_char);
}