// Jackson Coxson

#[no_mangle]
/// Starts the muxer and heartbeat client
/// # Arguments
/// Pairing file as a list of chars terminated by null
/// # Safety
/// Don't be stupid
pub unsafe extern "C" fn start_usbmuxd(pairing_file: *mut libc::c_char) -> libc::c_int {
    minimuxer::minimuxer_c_start(pairing_file)
}

#[no_mangle]
/// Debugs an app from an app ID
/// # Safety
/// Don't be stupid
pub unsafe extern "C" fn minimuxer_debug_app(app_id: *mut libc::c_char) -> libc::c_int {
    if app_id.is_null() {
        return -1;
    }

    let c_str = std::ffi::CStr::from_ptr(app_id);

    let app_id = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    }
    .to_string();

    minimuxer::tools::enable_jit(app_id)
}

#[no_mangle]
/// Sets the current environment variable for libusbmuxd to localhost
pub extern "C" fn target_minimuxer_address() {
    std::env::set_var("USBMUXD_SOCKET_ADDRESS", "127.0.0.1:27015");
}