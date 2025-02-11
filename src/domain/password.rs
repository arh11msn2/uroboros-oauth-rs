use pbkdf2::pbkdf2_hmac_array;
use rand::Rng;
use sha2::Sha256;

pub fn salt_password(password: &str, salt: &str) -> String {
    pbkdf2_hmac_array::<Sha256, 20>(password.as_bytes(), salt.as_bytes(), 1551)
        .iter()
        .map(|&c| c as char)
        .collect::<String>()
}

pub fn generate_salt() -> String {
    let mut rng = rand::thread_rng();
    let salt: String = (0..64)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    salt
}
