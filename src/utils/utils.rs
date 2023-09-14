use clap::Parser;
use text_to_ascii_art::convert;

pub fn banner(){
    match convert("Rustcrypt".to_string()) {
        Ok(string) => println!("{}\n", string),
        Err(err) => println!("Error: {}", err)   
    }
}

#[derive(Parser, Debug)]
#[command(name = "Rustcrypt")]
#[command(author = "Myst3ry")]
#[command(version = "1.0.0")]
#[command(about, long_about = None)]
#[command(
    help_template = " {author-with-newline} {about-section}Version: {version} \n {usage-heading} {usage} \n {all-args} {tab}"
)]
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