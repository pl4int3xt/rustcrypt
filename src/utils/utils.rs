use clap::Parser;
use text_to_ascii_art::convert;

pub enum Mode{
    Decrypt,
    Encrypt
}

pub fn banner(){
    match convert("Rustcrypt".to_string()) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err)   
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args{
    /// Input file or folder
    #[arg(short, long)]
    pub input: String,

    /// Location to output the key
    #[arg(short, long)]
    pub output: String,
}