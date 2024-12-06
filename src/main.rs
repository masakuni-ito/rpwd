use std::env;
use std::process;
use std::io;
use std::path::MAIN_SEPARATOR_STR;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, help = "Separate path components with spaces")]
    split: bool,
}

fn get_separator(split: bool) -> String {
    if split {
        format!(" {} ", MAIN_SEPARATOR_STR)
    } else {
        MAIN_SEPARATOR_STR.to_string()
    }
}

fn run() -> Result<bool, io::Error> {
    let args: Args = Args::parse();

    let path_components: Vec<String> = env::current_exe()?
        .iter()
        .skip(1)
        .map(|component| component.to_string_lossy().to_string())
        .collect();

    let separator = get_separator(args.split);

    let concatenated_path: String = path_components.join(&separator);
    println!("{}{}", separator, concatenated_path);
    Ok(true)
}

fn main() {
    if run().is_err() {
        process::exit(1);
    }
}

