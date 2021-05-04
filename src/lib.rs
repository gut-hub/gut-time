
#[allow(improper_ctypes_definitions)]
pub mod gut_time {
    use std::time::{SystemTime, UNIX_EPOCH};

    #[no_mangle]
    pub extern "C" fn gut_export_functions() -> String {
        r#"["time_now"]"#.to_string()
    }

    #[no_mangle]
    pub extern "C" fn gut_export_descriptions() -> String {
        r#"["Prints the current time in milliseconds"]"#.to_string()
    }

    #[no_mangle]
    fn time_now() {
        let now = SystemTime::now();
        let epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get time");

        println!("{}", epoch.as_millis())
    }
}
