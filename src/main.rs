use clap::Parser;
use std::env;

use rpwd::formatter::{format_path, FormatOptions};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long, help = "Enable colored output")]
    color: bool,

    #[arg(short, long, help = "Separate path components with spaces")]
    split: bool,

    #[arg(short, long, help = "Add a blank line before and after the path")]
    blankline: bool,
}

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let args: Args = Args::parse();

    let options = FormatOptions {
        color: args.color,
        split: args.split,
        blankline: args.blankline,
    };

    let formated_path = format_path(current_dir, options);

    println!("{}", formated_path);

    Ok(())
}
