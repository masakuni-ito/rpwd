use ansi_term::Colour;
use std::env;

const STYLES: [Colour; 5] = [
    Colour::Red,
    Colour::Green,
    Colour::Yellow,
    Colour::Blue,
    Colour::Purple,
];
struct ColorizedPath {
    path: String,
    color: Colour,
}

impl ColorizedPath {
    fn new(path: String, color: Colour) -> ColorizedPath {
        ColorizedPath {
            path: path,
            color: color,
        }
    }
}

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();
    let mut dirs: Vec<ColorizedPath> = vec![];

    let mut style_count = 0;
    for component in current_dir.components() {
        if let std::path::Component::Normal(dir) = component {
            let style = STYLES.get(style_count).unwrap();
            let dir = ColorizedPath::new(dir.to_string_lossy().to_string(), style.clone());
            dirs.push(dir);

            style_count += 1;
            if style_count >= STYLES.len() {
                style_count = 0;
            }
        }
    }

    for dir in dirs {
        print!("/{}", dir.color.paint(dir.path));
    }
    print!("\n");

    Ok(())
}
