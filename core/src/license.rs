use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{Local, NaiveDate};
use rsa::{
    Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
    pkcs8::{DecodePrivateKey, DecodePublicKey},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LicensePayload {
    // The machine ID is a unique identifier for the machine.
    pub machine_id: String,
    // The organization is the name of the organization that the license is for.
    pub org_name: String,
    // The expires_at is the timestamp in seconds when the license expires.
    pub expires_at: NaiveDate,
    // The max_users is the maximum number of users that the license allows.
    pub max_users: u32,
}

#[derive(Debug)]
pub struct LicenseStatus {
    pub payload: LicensePayload,
    pub valid: bool,
    pub is_expired: bool,
    pub expires_in_days: i64,
}

/**
 * Encode the license payload into a license string.
 *
 * The license string is a base64-encoded string that contains the license payload and the signature.
 *
 * # Arguments
 *
 * * `payload` - The license payload.
 * * `private_key_pem` - The private key in PEM format.
 *
 * # Returns
 *
 * The license string.
 */
pub fn encode_license(payload: &LicensePayload, private_key_pem: &str) -> String {
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem).unwrap();
    let paylod_json = serde_json::to_string(payload).unwrap();
    let paylod_encode = STANDARD.encode(paylod_json);

    let mut hasher = Sha256::new();
    hasher.update(&paylod_encode);
    let digest = hasher.finalize();
    let signature = private_key
        .sign(Pkcs1v15Sign::new_unprefixed(), &digest)
        .unwrap();
    let sign_encode = STANDARD.encode(signature);
    format!("{}.{}", paylod_encode, sign_encode)
}

pub fn decode_license(license: &str, public_key_pem: &str) -> Result<LicenseStatus, String> {
    let parts: Vec<&str> = license.split('.').collect();
    if parts.len() != 2 {
        return Err("Invalid license format".into());
    }
    let payload = parts[0];
    let signature = STANDARD
        .decode(parts[1])
        .map_err(|_| "Signature decode error")?;

    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    let digest = hasher.finalize();
    public_key
        .verify(Pkcs1v15Sign::new_unprefixed(), &digest, &signature)
        .map_err(|_| "Signature verify error")?;

    let json_bytes = STANDARD
        .decode(payload)
        .map_err(|_| "Payload decode error")?;
    let payload: LicensePayload =
        serde_json::from_slice(&json_bytes).map_err(|_| "Payload deserialize error")?;

    // Compute expiration
    let today = Local::now().date_naive();
    let expires_in_days = (payload.expires_at - today).num_days();
    let is_expired = expires_in_days < 0;

    Ok(LicenseStatus {
        payload,
        valid: !is_expired,
        is_expired,
        expires_in_days,
    })
}
