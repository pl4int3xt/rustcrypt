use std::fs::File;
use rand::{Rng, thread_rng};
use walkdir::Result;
use aes::Aes128;

const KEY_SIZE: usize = 16;
pub fn generate_key() -> [u8; KEY_SIZE]{
    let mut key = [0u8; KEY_SIZE];
    thread_rng().fill(&mut key);
    key
}

pub fn encrypt_file(file_path: &str, key: &[u8]){
    
}