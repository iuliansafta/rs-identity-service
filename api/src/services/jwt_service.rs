use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub struct JwtService {
    signing_key: String,
}

impl JwtService {
    // Initialize a new JwtService instance with a signing key.
    pub fn new(&mut self, singing_key: &str) {
        self.signing_key = singing_key.to_string();
    }

    // Generate an access token for a user.
    pub fn generate_access_token(&self, user_id: &str, user_identity: &str) -> String {
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(self.signing_key.as_bytes()).expect("Valid sining key");
        let mut claims = BTreeMap::new();
        claims.insert("uid", user_id);
        claims.insert("identity", user_identity);

        claims.sign_with_key(&key).unwrap()
    }
}
