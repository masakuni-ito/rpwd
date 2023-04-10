use clap::Parser;
use std::env;

use rpwd::formatter::format_path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    color: bool,
}

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let args: Args = Args::parse();

    let formated_path = format_path(current_dir, args.color);

    println!("{}", formated_path);

    Ok(())
}
