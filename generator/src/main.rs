use core::{license::{LicensePayload, decode_license, encode_license}, machine::get_machine_id};
use chrono::NaiveDate;
use std::fs;

fn main() {
    println!("--- Auto License Test Start ---");

    // ===== 1. 获取本机机器码 =====
    let machine_id = get_machine_id();

    println!("Machine ID: {}", machine_id);

    // ===== 2. 固定参数 =====
    let payload = LicensePayload {
        machine_id,
        org_name: "Local".to_string(),
        expires_at: NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
        max_users: 9999,
    };

    // ===== 3. 读取 RSA 私钥/公钥 =====
    let private_key_pem = fs::read_to_string("../private.pem")
        .expect("cannot read private.pem");
    let public_key_pem = fs::read_to_string("../public.pem")
        .expect("cannot read public.pem");

    // ===== 4. 生成 License =====
    let license = encode_license(&payload, &private_key_pem);
    println!("\nGenerated License:\n{}\n", license);

    // ===== 5. 校验 License =====
    match decode_license(&license, &public_key_pem) {
        Ok(decoded) => {
            println!("License verification: OK");
            println!("Decoded payload = {:?}", decoded);

            // ===== 6. 判断是否过期 =====
            let today = chrono::Local::now().naive_local().date();
            let expires = payload.expires_at;

            if today > expires {
                println!("⚠ License expired!");
            } else {
                let days_left = (expires - today).num_days();
                println!("Valid days left: {}", days_left);
            }

            println!("\n--- Auto License Test Success ---");
        }
        Err(err) => {
            println!("Verification failed: {}", err);
            println!("--- Auto License Test Failed ---");
        }
    }
}
