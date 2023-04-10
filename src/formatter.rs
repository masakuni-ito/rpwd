use colored::*;
use std::path::PathBuf;

pub fn format_path(path: PathBuf, color: bool) -> String {
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

    let formated_path: String = path
        .iter()
        .skip(1)
        .map(|component| {
            let component_str = component.to_string_lossy().to_string();
            if color {
                let next_color = color_cycle.next().unwrap();
                format!("/{}", component_str.color(next_color))
            } else {
                format!("/{}", component_str)
            }
        })
        .collect();

    format!("{}", formated_path)
}
