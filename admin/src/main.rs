use base64::{Engine, engine::general_purpose::STANDARD};
use std::fs;

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

fn main() {
    // 1. 获取机器码（Rust）
    let machine_id = core::machine::get_machine_id();
    println!("Machine ID: {}", machine_id);

    // 2. 加载私钥
    let private_key_pem = fs::read_to_string("../private.pem").expect("private.pem missing!");

    // 3. 生成 License
    let license_payload = core::license::LicensePayload {
        machine_id,
        org_name: "NGPACS".to_string(),
        expires_at: "2025-12-31".to_string(),
        max_users: 100,
    };

    let license = core::license::generate_license(&license_payload, &private_key_pem)
        .expect("generate license failed!");
    println!("\nGenerated License:\n{}\n", license);

    let (sig_b64, payload_b64 ) = license.split_once('.').unwrap();

    // 4. 验证 License
    let info = core::license::verify_license(
        payload_b64,
        sig_b64,
        PUBLIC_KEY_PEM,
        &license_payload.machine_id,
    )
    .expect("license verify failed");

    println!("Verified LicenseInfo:\n{:#?}", info);

    // 5. 加密 LicenseInfo
    let encrypted = core::license::encrypt_license_info(&info)
        .expect("license encrypt failed");
    println!("Encrypted LicenseInfo:\n{}\n", STANDARD.encode(encrypted));
}
