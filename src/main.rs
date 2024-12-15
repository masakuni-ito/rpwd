mod args;
mod format;
mod path;

use clap::Parser;
use std::io;
use std::process;

fn run() -> Result<bool, io::Error> {
    let args = args::Args::parse();

    let path_components = path::get_current_dir(args.logical, args.physical)?;

    let path_components = format::add_separator(args.divide, path_components);

    let path_components = format::format_color(args.color, path_components);

    let path_components = format::format_stairs(args.stairs, path_components);

    println!("{}", path_components.join(""));

    Ok(true)
}

fn main() {
    if run().is_err() {
        process::exit(1);
    }
}
