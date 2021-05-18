use std::time::{SystemTime, UNIX_EPOCH};

gut_plugin::gut_export!(["time_now"], ["Prints the current time in milliseconds"]);

#[no_mangle]
fn time_now(_ptr: i32, _len: i32) {
    let now = SystemTime::now();
    let epoch = now.duration_since(UNIX_EPOCH).expect("Failed to get time");

    println!("{}", epoch.as_millis())
}
