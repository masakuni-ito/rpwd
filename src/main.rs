use ansi_term::Colour;
use std::env;

const STYLES: [Colour; 5] = [
    Colour::Red,
    Colour::Green,
    Colour::Yellow,
    Colour::Blue,
    Colour::Purple,
];

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();

    let mut dirs = vec![];
    for component in current_dir.components() {
        if let std::path::Component::Normal(dir) = component {
            dirs.push(dir.to_string_lossy());
        }
    }

    let mut style_count = 0;
    let mut styled_dirs = vec![];
    for dir in dirs {
        let style = STYLES.get(style_count).unwrap().paint(dir);
        styled_dirs.push(style.clone());

        style_count += 1;
        if style_count >= STYLES.len() {
            style_count = 0;
        }
    }

    for dir in styled_dirs {
        print!(" / ");
        print!("{}", dir.to_string());
    }
    print!("\n");

    Ok(())
}
