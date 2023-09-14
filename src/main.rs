extern crate clap;
use clap::Parser;

mod handlers;
mod utils;

fn main() {
    utils::utils::banner();
    let args = utils::utils::Args::parse();

}
