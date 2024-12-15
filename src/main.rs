use std::env;
use std::io;
use std::path::{PathBuf, MAIN_SEPARATOR_STR};
use std::process;

use clap::Parser;
use colored::*;

#[derive(Parser)]
struct Args {
    #[arg(
        short = 'L',
        long,
        help = "Display the logical current working directory"
    )]
    logical: bool,

    #[arg(
        short = 'P',
        long,
        help = "Display the physical current working directory"
    )]
    physical: bool,

    #[arg(short, long, help = "Divide path components with spaces")]
    divide: bool,

    #[arg(short, long, help = "Display the path with colored components")]
    color: bool,

    #[arg(short, long, help = "Display the path in stairs format")]
    stairs: bool,
}

fn get_current_dir(logical: bool, physical: bool) -> Result<Vec<String>, io::Error> {
    let path_components = if physical {
        env::current_dir()?
    } else if logical {
        PathBuf::from(env::var("PWD").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?)
    } else {
        PathBuf::from(env::var("PWD").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?)
    };

    Ok(path_components
        .iter()
        .map(|component| component.to_string_lossy().to_string())
        .collect())
}

fn add_separator(divide: bool, path_components: Vec<String>) -> Vec<String> {
    let separator = match divide {
        true => format!(" {} ", MAIN_SEPARATOR_STR),
        false => MAIN_SEPARATOR_STR.to_string(),
    };

    path_components
        .iter()
        .enumerate()
        .map(|(i, component)| {
            if i == 0 {
                separator.clone()
            } else if i < path_components.len() - 1 {
                format!("{}{}", component, separator)
            } else {
                component.to_string()
            }
        })
        .collect()
}

fn format_color(color: bool, path_components: Vec<String>) -> Vec<String> {
    if !color {
        return path_components;
    }

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

fn format_stairs(stairs: bool, path_components: Vec<String>) -> Vec<String> {
    match stairs {
        true => path_components
            .iter()
            .enumerate()
            .map(|(i, component)| {
                if i < path_components.len() - 1 {
                    format!("{}{}\n", " ".repeat(i * 2), component)
                } else {
                    format!("{}{}", " ".repeat(i * 2), component)
                }
            })
            .collect(),
        false => path_components,
    }
}

fn run() -> Result<bool, io::Error> {
    let args: Args = Args::parse();

    let path_components = get_current_dir(args.logical, args.physical)?;

    let path_components = add_separator(args.divide, path_components);

    let path_components = format_color(args.color, path_components);

    let path_components = format_stairs(args.stairs, path_components);

    println!("{}", path_components.join(""));

    Ok(true)
}

fn main() {
    if run().is_err() {
        process::exit(1);
    }
}
