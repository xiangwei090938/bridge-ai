use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::pbkdf2;
use ring::rand::{SecureRandom, SystemRandom};
use std::num::NonZeroU32;
use crate::core::error::{AppError, Result};

const PBKDF2_ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };
const SALT_LEN: usize = 32;
const NONCE_LEN: usize = 12;

/// Encrypt plaintext using AES-256-GCM with a PBKDF2-derived key
pub fn encrypt(plaintext: &str, password: &str) -> Result<Vec<u8>> {
    let rng = SystemRandom::new();

    // Generate random salt
    let mut salt = [0u8; SALT_LEN];
    rng.fill(&mut salt).map_err(|_| AppError::Encryption("Failed to generate salt".into()))?;

    // Derive key using PBKDF2
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITERATIONS,
        &salt,
        password.as_bytes(),
        &mut key_bytes,
    );

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| AppError::Encryption("Failed to create key".into()))?;
    let key = LessSafeKey::new(unbound_key);

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes).map_err(|_| AppError::Encryption("Failed to generate nonce".into()))?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    // Encrypt (in-place)
    let mut in_out = plaintext.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| AppError::Encryption("Encryption failed".into()))?;

    // Result: [salt][nonce][ciphertext + tag]
    let mut result = Vec::with_capacity(SALT_LEN + NONCE_LEN + in_out.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&in_out);

    Ok(result)
}

/// Decrypt ciphertext that was encrypted with `encrypt`
pub fn decrypt(ciphertext: &[u8], password: &str) -> Result<String> {
    if ciphertext.len() < SALT_LEN + NONCE_LEN + 16 {
        return Err(AppError::Encryption("Invalid ciphertext length".into()));
    }

    let (salt, rest) = ciphertext.split_at(SALT_LEN);
    let (nonce_bytes, encrypted) = rest.split_at(NONCE_LEN);

    // Derive the same key
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITERATIONS,
        salt,
        password.as_bytes(),
        &mut key_bytes,
    );

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| AppError::Encryption("Failed to create key".into()))?;
    let key = LessSafeKey::new(unbound_key);

    let nonce = Nonce::assume_unique_for_key({ const L: usize = 12; let mut a = [0u8; L]; a.copy_from_slice(&nonce_bytes[..L]); a });

    let mut in_out = encrypted.to_vec();
    let plaintext = key.open_in_place(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| AppError::Encryption("Decryption failed".into()))?;

    String::from_utf8(plaintext.to_vec())
        .map_err(|_| AppError::Encryption("Invalid UTF-8 after decryption".into()))
}




