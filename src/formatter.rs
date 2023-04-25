use colored::*;
use std::path::PathBuf;

pub struct FormatOptions {
    pub color: bool,
    pub split: bool,
    pub blankline: bool,
}

pub fn format_path(path: PathBuf, options: FormatOptions) -> String {
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

    let separator = if options.split { " / " } else { "/" };
    let blankline = if options.blankline { "\n" } else { "" };

    let formated_path: String = path
        .iter()
        .skip(1)
        .map(|component| {
            let component_str = component.to_string_lossy().to_string();
            if options.color {
                let next_color = color_cycle.next().unwrap();
                format!("{}{}", separator, component_str.color(next_color))
            } else {
                format!("{}{}", separator, component_str)
            }
        })
        .collect();

    format!("{}{}{}", blankline, formated_path, blankline)
}
