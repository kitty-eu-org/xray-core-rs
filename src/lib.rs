mod ffi;

use std::ffi::{c_uint, CStr, CString};

pub fn query_stats(api_server: &str, timeout: i32, my_pattern: &str, reset: i8) -> String {
    let api_server_cstr = CString::new(api_server).expect("CString::new failed");
    let my_pattern_cstr = CString::new(my_pattern).expect("CString::new failed");

    let ptr = unsafe {
        ffi::queryStats(api_server_cstr.as_ptr(), timeout, my_pattern_cstr.as_ptr(), reset as c_uint)
    };

    let result = unsafe {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    };


    result
}

pub fn start_from_json(json: &str) {
    let json_cstr = CString::new(json).expect("CString::new failed");
    unsafe {
        ffi::startFromJSON(json_cstr.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let json = r#"{"key": "value"}"#;
        // query_stats()
        start_from_json(json);
    }
}