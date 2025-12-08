use std::ffi::{CStr, CString, c_char, c_int};

#[unsafe(no_mangle)]
pub extern "C" fn get_machine_id() -> *mut c_char {
    let id = core::machine::get_machine_id();

    // CString 必须用 into_raw 交给 C/Java
    let c_str = CString::new(id).unwrap();
    c_str.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn verify_license(
    license_ptr: *const c_char,
    public_key_ptr: *const c_char,
    err_buf: *mut c_char,
    err_buf_len: c_int,
) -> c_int {
    let license_cstr = unsafe {
        if license_ptr.is_null() {
            return write_error("license_ptr is null", err_buf, err_buf_len);
        }
        CStr::from_ptr(license_ptr)
    };

    let public_key_cstr = unsafe {
        if public_key_ptr.is_null() {
            return write_error("public_key_ptr is null", err_buf, err_buf_len);
        }
        CStr::from_ptr(public_key_ptr)
    };

    let license = match license_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return write_error("Invalid UTF-8 in license", err_buf, err_buf_len),
    };

    let public_key = match public_key_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return write_error("Invalid UTF-8 in public key", err_buf, err_buf_len),
    };

    let result = core::license::decode_license(license, public_key);

    match result {
        Ok(status) => {
            if status.is_expired {
                return write_error("License expired", err_buf, err_buf_len);
            }
            0 // success
        }
        Err(err) => write_error(&err, err_buf, err_buf_len),
    }
}

fn write_error(msg: &str, err_buf: *mut c_char, buf_len: c_int) -> c_int {
    if err_buf.is_null() || buf_len <= 1 {
        return 1; // cannot write msg
    }

    let msg = CString::new(msg).unwrap();
    let bytes = msg.as_bytes_with_nul();

    unsafe {
        let max_len = (buf_len as usize) - 1;
        let copy_len = bytes.len().min(max_len);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), err_buf as *mut u8, copy_len);
        *err_buf.add(copy_len) = 0;
    }
    1
}