use url::Url;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use dialoguer::Confirm;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    urls: Vec<String>,
}

fn main() {
    println!("Hello, world!");
}
