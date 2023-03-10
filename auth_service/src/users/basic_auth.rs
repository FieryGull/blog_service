use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string()
}

pub fn verify(password: &String, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeed_hash_and_verify_password() {
        let pwd = String::from("test_pwd");
        let hash_pwd = hash_password(&pwd);
        assert!(verify(&pwd, &hash_pwd))
    }
}
