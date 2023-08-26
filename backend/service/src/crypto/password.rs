use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tokio::task;

pub async fn hash(password: String, pepper: String) -> String {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        argon2(&pepper)
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    })
    .await
    .unwrap()
}

pub async fn verify(password: String, hash: String, pepper: String) -> bool {
    task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash).unwrap();
        argon2(&pepper)
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    })
    .await
    .unwrap()
}

fn argon2(pepper: &str) -> Argon2 {
    Argon2::new_with_secret(
        pepper.as_bytes(),
        Default::default(),
        Default::default(),
        Default::default(),
    )
    .unwrap()
}
