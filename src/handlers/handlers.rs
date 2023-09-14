use std::{
    fs::{self, File},
    io::{Read, Write},
};
use anyhow::{Result, anyhow, Ok};
use base64::{engine::general_purpose, Engine};
use chacha20poly1305::{
    aead::{stream, Aead, OsRng},
    XChaCha20Poly1305, KeyInit
};
use rand::RngCore;

    
pub fn mode_check(mode: &str, key: &[u8; 32], nonce: &[u8; 24]){
    let encoded_key = general_purpose::STANDARD_NO_PAD.encode(key);
    let encoded_nonce = general_purpose::STANDARD_NO_PAD.encode(nonce);
    match mode {
        "encrypt" => {
            println!("Save the key and the nonce safely");
            println!("----------------------------------------------------------\n");
            println!("{{\n");
            println!("  Key :{:?}\n", encoded_key);
            println!("  Nonce :{:?}\n", encoded_nonce);
            println!("}}\n");
            println!("----------------------------------------------------------\n");
        }
        _ => {}
    }
}

pub fn run(mode: &str, size: &str,filepath: &str, dist: &str, input_key: &str, input_nonce: &str){
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

    let dec_key: [u8; 32]= {
        if input_key != ""{
            let mut array = [0; 32];
            array.copy_from_slice(&general_purpose::STANDARD_NO_PAD.decode(input_key).unwrap());
            array
        } else {
            [0u8; 32]
        }
    };

    let dec_nonce: [u8; 24]= {
        if input_nonce != ""{
            let mut array = [0; 24];
            array.copy_from_slice(&general_purpose::STANDARD_NO_PAD.decode(input_nonce).unwrap());
            array
        } else {
            [0u8; 24]
        }
    };

    match mode {
        "encrypt" => match size {
            "small" => {
                match encrypt_small_file(filepath, dist, &key, &nonce) {
                    Err(error) => { 
                        println!("Error occurred : {}", error);
                        std::process::exit(1);
                    }
                    _ => {}
                }
                
                mode_check(mode, &key, &nonce);
            },
            "large" => {
                encrypt_large_file(filepath, dist, &key, &nonce).unwrap();
            },
            _ => {}
        },
        "decrypt" => match size {
            "small" => {
                match decrypt_small_file(filepath, dist, &dec_key, &dec_nonce) {
                    Err(error) => { 
                        println!("Error occurred : {}", error);
                        std::process::exit(1);
                    }
                    _ => {}
                }
            },
            "large" => {
                decrypt_large_file(filepath, dist, &dec_key, &dec_nonce).unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}

pub fn encrypt_small_file(filepath: &str, dist: &str, key: &[u8; 32], nonce: &[u8; 24] ) -> Result<(), anyhow::Error>{
    let cipher = XChaCha20Poly1305::new(key.into());
    let file_data = fs::read(filepath)?;

    let encrypted_file = cipher
        .encrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {}", err))?;

    fs::write(&dist, encrypted_file)?;

    Ok(println!("3ncrypt3d succ3ssfully"))
} 

fn decrypt_small_file(encrypted_filepath: &str,dist: &str,key: &[u8; 32],nonce: &[u8; 24]) -> Result<(), anyhow::Error>{
    let cipher = XChaCha20Poly1305::new(key.into());
    let file_data = fs::read(encrypted_filepath)?;

    let decrypted_file = cipher
        .decrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

    fs::write(&dist, decrypted_file)?;

    Ok(println!("D3crpt3d succ3ssfully"))
}


fn encrypt_large_file(filepath: &str, dist: &str, key: &[u8; 32], nonce: &[u8; 24]) -> Result<(), anyhow::Error>{
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut source_file = File::open(filepath)?;
    let mut dist_file = File::create(dist)?;

    loop {
        let read_count = source_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let ciphertext = stream_encryptor
                .encrypt_next(buffer.as_slice())
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
        } else {
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
            break;
        }
    }

    Ok(println!("3ncrypt3d succ3ssfully"))
}

fn decrypt_large_file(encrypted_file_path: &str,dist: &str,key: &[u8; 32],nonce: &[u8; 24]) -> Result<(), anyhow::Error>{
    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500 + 16;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut dist_file = File::create(dist)?;

    loop {
        let read_count = encrypted_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let plaintext = stream_decryptor
                .decrypt_next(buffer.as_slice())
                .map_err(|err| anyhow!("Decrypting large file: {}", err))?;
            dist_file.write(&plaintext)?;
        } else if read_count == 0 {
            break;
        } else {
            let plaintext = stream_decryptor
                .decrypt_last(&buffer[..read_count])
                .map_err(|err| anyhow!("Decrypting large file: {}", err))?;
            dist_file.write(&plaintext)?;
            break;
        }
    }

    Ok(println!("D3crypt3d succ3ssfully"))
}
    
