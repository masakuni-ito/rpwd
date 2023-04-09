use clap::Parser;
use colored::*;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    color: bool,
}

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let mut path = PathBuf::new();

    let args: Args = Args::parse();

    if args.color {
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

        for component in current_dir.iter().skip(1) {
            let next_color = color_cycle.next().unwrap();
            let dir = component
                .to_string_lossy()
                .to_string()
                .color(next_color)
                .to_string();
            path.push(dir);
        }
    } else {
        for component in current_dir.iter().skip(1) {
            let dir = component.to_string_lossy().to_string();
            path.push(dir);
        }
    }

    println!("/{}", path.display());

    Ok(())
}
