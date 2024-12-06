use std::env;
use std::process;
use std::io;

fn run() -> Result<bool, io::Error> {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            Ok(true)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn main() {
    if run().is_err() {
        process::exit(1);
    }
}

