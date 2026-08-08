use archive_forge::sha256;
use std::{env, fs};

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: archive-forge FILE");
        std::process::exit(2);
    };
    match fs::read(&path) {
        Ok(bytes) => println!("{}  {}  {} bytes", sha256(&bytes), path, bytes.len()),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
