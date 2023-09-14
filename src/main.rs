extern crate clap;
use chacha20poly1305::aead::OsRng;
use clap::Parser;
use rand::RngCore;

mod handlers;
mod utils;

fn main() {
    utils::utils::banner();
    let args = utils::utils::Args::parse();

    let mut key = [0u8; 32];
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut nonce);

    println!("{:?} ==============", key);
    println!("{:?} ==============", nonce);
    println!("{:?}",
        handlers::handlers::encrypt_small_file(&args.input, &args.output, &key, &nonce).unwrap());
}
