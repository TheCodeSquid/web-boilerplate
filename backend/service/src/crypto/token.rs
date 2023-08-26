use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HS256 = Hmac<Sha256>;

pub fn sign(data: &[u8], secret: &str) -> String {
    let signature = hash(data, secret);

    let joined = format!(
        "{}.{}",
        URL_SAFE_NO_PAD.encode(data),
        URL_SAFE_NO_PAD.encode(&signature)
    );
    URL_SAFE_NO_PAD.encode(joined.as_bytes())
}

pub fn verify(token: &str, secret: &str) -> Option<Vec<u8>> {
    let token = String::from_utf8(URL_SAFE_NO_PAD.decode(token.as_bytes()).ok()?).ok()?;
    let (bytes, signature) = token.split_once('.')?;
    let bytes = URL_SAFE_NO_PAD.decode(bytes).ok()?;
    let signature = URL_SAFE_NO_PAD.decode(signature).ok()?;

    (signature == hash(&bytes, secret)).then(|| bytes)
}

fn hash(bytes: &[u8], secret: &str) -> Vec<u8> {
    let mut mac = HS256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(bytes);
    mac.finalize().into_bytes().to_vec()
}
