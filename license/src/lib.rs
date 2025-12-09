use jni::{
    JNIEnv,
    objects::{JByteArray, JClass},
    sys::jbyteArray,
};

static PUBLIC_KEY_PEM: &str = r#"
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwiCaS0V4xy4iXMYho42L
3cNCwjuAKIlAwO87pLBlHdDyQW6ir8nhns7YW2nKWKyrcoZx6AkjTJjQINcjEMt9
zRinIpr4k6jvYx29HNTX3Wz8BvTRmid0avA07Yxb1gzxsCA0BWQvA4kW8HtxPuRb
X2IW3g8kn1GLXca32b4yOcSAKPW5sOl1WUCTiMuPpD/M2LyLc5uQ7SJ350mAIRZv
RPttOEQzeExWTDes3AItR9atsjqDGPvcM2vM9sPAjFnKv42hFcJ99sK3Xstb9pQX
ucWfarFzC1QNrqjfZOVNXjLidyNGnNc5oqH7RTY1HEhORTfyAXu/u66Mc5oPUt72
8wIDAQAB
-----END PUBLIC KEY-----
"#;

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_ngmis_ngpacs_core_license_NativeLicense_verifyLicense(
    env: JNIEnv,
    _class: JClass,
    sig_b64: JByteArray,
    payload_b64: JByteArray,
    machine_id: JByteArray,
) -> jbyteArray {
    let payload_bytes = env.convert_byte_array(payload_b64).unwrap_or_default();
    let sig_bytes = env.convert_byte_array(sig_b64).unwrap_or_default();
    let machine_bytes = env.convert_byte_array(machine_id).unwrap_or_default();

    let payload_str = String::from_utf8(payload_bytes).unwrap();
    let sig_str = String::from_utf8(sig_bytes).unwrap();
    let machine_str = String::from_utf8(machine_bytes).unwrap();

    let info = core::license::verify_license(&payload_str, &sig_str, PUBLIC_KEY_PEM, &machine_str);
    if info.is_none() {
        return std::ptr::null_mut();
    }

    let encrypted = core::license::encrypt_license_info(&info.unwrap()).unwrap_or_default();
    env.byte_array_from_slice(&encrypted).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_ngmis_ngpacs_core_license_NativeLicense_getMachineId(
    env: JNIEnv,
    _class: JClass,
) -> jbyteArray {
    let mid = core::machine::get_machine_id();
    env.byte_array_from_slice(&mid.as_bytes()).unwrap().into_raw()
}