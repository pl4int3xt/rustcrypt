use std::fs;
use anyhow::{Result, anyhow, Ok};
use chacha20poly1305::{
    aead::{stream, Aead},
    XChaCha20Poly1305, KeyInit
};

pub fn encrypt_small_file( filepath: &str, dist: &str, key: &[u8; 32], nonce: &[u8; 24] ) -> Result<(), anyhow::Error>{
    let cipher = XChaCha20Poly1305::new(key.into());
    let file_data = fs::read(filepath)?;

    let encrypted_file = cipher
        .encrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {}", err))?;

    fs::write(&dist, encrypted_file)?;

    Ok(())
} 

pub fn decrypt_small_file( encrypted_filepath: &str,dist: &str,key: &[u8; 32],nonce: &[u8; 24]) -> Result<(), anyhow::Error>{
    let cipher = XChaCha20Poly1305::new(key.into());
    let file_data = fs::read(encrypted_filepath)?;

    let decrypted_file = cipher
        .decrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

    fs::write(&dist, decrypted_file)?;

    Ok(())
}