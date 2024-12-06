use std::env;
use std::process;
use std::io;
use std::path::{PathBuf, MAIN_SEPARATOR_STR};

fn run() -> Result<bool, io::Error> {
    let current_dir: PathBuf = env::current_dir()?;

    let concatenated_path: String = current_dir
        .iter()
        .skip(1)
        .map(|component| component.to_string_lossy())
        .collect::<Vec<_>>()
        .join(MAIN_SEPARATOR_STR);

    println!("{}{}", MAIN_SEPARATOR_STR, concatenated_path);
    Ok(true)
}

fn main() {
    if run().is_err() {
        process::exit(1);
    }
}

