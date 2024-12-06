use std::env;
use std::process;
use std::io;
use std::path::MAIN_SEPARATOR_STR;

use clap::Parser;
use colored::*;

#[derive(Parser)]
struct Args {
    #[arg(short='s', long, help = "Separate path components with spaces")]
    split: bool,

    #[arg(short='c', long, help = "Enable colored output")]
    color: bool,

    #[arg(short='t', long, help = "Display path in stairs format")]
    stairs: bool,
}

fn get_separator(split: bool) -> String {
    if split {
        format!(" {} ", MAIN_SEPARATOR_STR)
    } else {
        MAIN_SEPARATOR_STR.to_string()
    }
}

fn format_color(color: bool, path_components: Vec<String>) -> Vec<String> {

    if !color { return path_components; }

    let mut color_cycle = vec![
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
    ]
    .into_iter()
    .cycle();

    path_components
        .iter()
        .map(|component| {
            let next_color = color_cycle.next().unwrap();
            format!("{}", component.color(next_color))
        })
        .collect()
}

fn format_stairs(stairs: bool, separator: String, path_components: Vec<String>) -> Vec<String> {

    match stairs {
        true => {
            path_components
                .iter()
                .enumerate()
                .map(|(i, component)| format!("{}{}{}\n", " ".repeat(i * 2), separator, component))
                .collect()
        }
        false => {
            path_components
                .iter()
                .map(|component| format!("{}{}", separator, component))
                .collect()
        }
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

    let path_components = format_color(args.color, path_components);

    let path_components = format_stairs(args.stairs, separator, path_components);

    println!("{}", path_components.join(""));

    Ok(true)
}

fn main() {
    if run().is_err() {
        process::exit(1);
    }
}

