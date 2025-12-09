use aes::cipher::{BlockEncryptMut, KeyIvInit};
use base64::{Engine, engine::general_purpose::STANDARD};
use block_padding::Pkcs7;
use hmac::Mac;
use rsa::{Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey, pkcs8::{DecodePrivateKey, DecodePublicKey}};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

type HmacSha256 = hmac::Hmac<sha2::Sha256>;
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

const SESSION_KEY_SALT: &[u8] = b"-ngpacs-salt-2025-";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LicensePayload {
    // The machine ID is a unique identifier for the machine.
    pub machine_id: String,
    // The organization is the name of the organization that the license is for.
    pub org_name: String,
    // The expires_at is the timestamp in seconds when the license expires.
    pub expires_at: String,
    // The max_users is the maximum number of users that the license allows.
    pub max_users: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub session_key: Vec<u8>,
    // The expires_at is the timestamp in seconds when the license expires.
    pub expires_at: String,
    // The org_name is the name of the organization that the license is for.
    pub org_name: String,
    // The max_users is the maximum number of users that the license allows.
    pub max_users: u32,
}

pub fn verify_license(
    payload_b64: &str,
    sig_b64: &str,
    public_key_pem: &str,
    machine_id: &str,
) -> Option<LicenseInfo> {
    // 1. verify signature
    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem).unwrap();
    let sig_bytes = STANDARD.decode(sig_b64).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(payload_b64.as_bytes());
    let digest = hasher.finalize();
    if public_key
        .verify(Pkcs1v15Sign::new_unprefixed(), &digest, &sig_bytes)
        .is_err()
    {
        return None;
    }
    // 2. decode payload
    let payload_bytes = STANDARD.decode(payload_b64).ok()?;
    let payload_json = String::from_utf8(payload_bytes).ok()?;
    let payload: LicensePayload = serde_json::from_str(&payload_json).ok()?;

    // 3. check machine id
    if payload.machine_id != machine_id.to_string() {
        return None;
    }

    // 4. generate session key
    let mut session_hasher = Sha256::new();
    session_hasher.update(payload_json.as_bytes());
    session_hasher.update(SESSION_KEY_SALT);
    let session_key = session_hasher.finalize().to_vec();

    // 5. return license info
    Some(LicenseInfo {
        session_key,
        expires_at: payload.expires_at,
        org_name: payload.org_name,
        max_users: payload.max_users,
    })
}

pub fn generate_license(payload: &LicensePayload, private_key_pem: &str) -> Result<String, String> {
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem).map_err(|e| e.to_string())?;
    let paylod_json = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    let paylod_encode = STANDARD.encode(paylod_json);

    let mut hasher = Sha256::new();
    hasher.update(&paylod_encode);
    let digest = hasher.finalize();
    let signature = private_key
        .sign(Pkcs1v15Sign::new_unprefixed(), &digest)
        .unwrap();
    let sign_encode = STANDARD.encode(signature);
    Ok(format!("{}.{}", sign_encode, paylod_encode))
}

pub fn encrypt_license_info(info: &LicenseInfo) -> Option<Vec<u8>> {
    let key = &info.session_key;
    let iv = &key[..16];
    let info_json = serde_json::to_string(info).ok()?;
    let mut buf = vec![0u8; (info_json.len() + 15) / 16 * 16 + 16];
    buf[..info_json.len()].copy_from_slice(info_json.as_bytes());

    let encryptor = Aes256CbcEnc::new_from_slices(key, iv).ok()?;
    let ciphertext = encryptor
        .encrypt_padded_mut::<Pkcs7>(&mut buf, info_json.len())
        .ok()?;

    let mut mac = HmacSha256::new_from_slice(key).ok()?;
    mac.update(&ciphertext);
    let tag = mac.finalize().into_bytes();

    Some([ciphertext, tag.to_vec().as_slice()].concat())
}
