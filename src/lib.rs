pub mod gut_time {
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[no_mangle]
    pub extern "C" fn gut_export_functions() -> *mut c_char {
        let c_string = CString::new(r#"["time_now"]"#).expect("Failed to create c_string");
        c_string.into_raw()
    }

    #[no_mangle]
    pub extern "C" fn gut_export_descriptions() -> *mut c_char {
        let c_string = CString::new(r#"["Prints the current time in milliseconds"]"#)
            .expect("Failed to create c_string");
        c_string.into_raw()
    }

    #[no_mangle]
    fn time_now(_ptr: i32, _len: i32) {
        let now = SystemTime::now();
        let epoch = now.duration_since(UNIX_EPOCH).expect("Failed to get time");

        println!("{}", epoch.as_millis())
    }
}
