use colored::*;
use std::path::PathBuf;

use rpwd::formatter::format_path;

#[test]
fn test_format_path_no_color() {
    let input = PathBuf::from("/path/to");
    let expected = "/path/to".to_string();
    let actual = format_path(input, false);

    assert_eq!(actual, expected);
}

#[test]
fn test_format_path_with_color() {
    let input = PathBuf::from("/path/to/aaa/bbb/ccc/ddd/zzz");
    let result = format_path(input, true);
    let expected = format!(
        "/{}/{}/{}/{}/{}/{}/{}",
        "path".color(Color::Red),
        "to".color(Color::Green),
        "aaa".color(Color::Yellow),
        "bbb".color(Color::Blue),
        "ccc".color(Color::Magenta),
        "ddd".color(Color::Cyan),
        "zzz".color(Color::Red),
    );
    assert_eq!(result, expected);
}
