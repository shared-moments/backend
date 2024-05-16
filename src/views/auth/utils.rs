use std::collections::BTreeMap;
use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::config;

use super::structs::CurrentUser;


const COST: u32 = 13;


pub fn hash_password(
    password: &str
) -> String {
    let salt: [u8; 16] = config::CONFIG.encrypt_salt.as_bytes().try_into().unwrap();

    bcrypt::hash_with_salt(password.as_bytes(), COST, salt).unwrap().to_string()
}

pub fn get_token(user_id: i32) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config::CONFIG.secret_ket.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();

    claims.insert("user_id", user_id.to_string());

    claims.sign_with_key(&key).unwrap()
}

pub async fn authorize_current_user(
    token: &str
) -> Option<CurrentUser> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config::CONFIG.secret_ket.as_bytes()).unwrap();
    let claims: BTreeMap<String, String> = token.verify_with_key(&key).unwrap();

    let id = claims.get("user_id")?.parse::<u32>().ok()?;

    Some(CurrentUser { id })
}
