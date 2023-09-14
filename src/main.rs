extern crate clap;
use clap::Parser;

mod handlers;
mod utils;

fn main() {
    utils::utils::banner();
    let args = utils::utils::Args::parse();
    let mut key: String = String::from("");
    let mut nonce: String = String::from("");

    match &args.key {
        Some(value) => { key.push_str(&format!("{}", value)); },
        None => {"";}
    }

    match &args.nonce {
        Some(value) => { nonce.push_str(&format!("{}", value)); },
        None => {"";}
    }


    handlers::handlers::run(
        &args.mode, 
        &args.size,
        &args.input,
        &args.output,
        &key,
        &nonce);
}
