use clap::Parser;
use text_to_ascii_art::convert;

pub fn banner(){
    match convert("Rustcrypt".to_string()) {
        Ok(string) => println!("{}\n", string),
        Err(err) => println!("Error: {}", err)   
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args{
    /// Input file path
    #[arg(short, long)]
    pub input: String,

    /// Output file path with key
    #[arg(short, long)]
    pub output: String,

    /// file size = [large, small]
    #[arg(short, long)]
    pub size: String,

    /// type of mode = [encrypt, decrypt] 
    #[arg(short, long)]
    pub mode: String,

    /// Key to decrypt file
    #[arg(short, long)]
    pub key: Option<String>,

    /// nonce to decrypt file
    #[arg(short, long)]
    pub nonce: Option<String>,
}