use colored::*;
use std::path::PathBuf;

use rpwd::formatter::{format_path, FormatOptions};

#[test]
fn test_format_path_no_option() {
    let input = PathBuf::from("/path/to/test1/test2/test3/test4/test5");
    let options = FormatOptions {
        color: false,
        split: false,
        blankline: false,
    };

    let expected = "/path/to/test1/test2/test3/test4/test5".to_string();
    let actual = format_path(input, options);

    assert_eq!(actual, expected);
}

#[test]
fn test_format_path_with_color() {
    let input = PathBuf::from("/path/to/test1/test2/test3/test4/test5");
    let options = FormatOptions {
        color: true,
        split: false,
        blankline: false,
    };

    let result = format_path(input, options);
    let expected = format!(
        "/{}/{}/{}/{}/{}/{}/{}",
        "path".color(Color::Red),
        "to".color(Color::Green),
        "test1".color(Color::Yellow),
        "test2".color(Color::Blue),
        "test3".color(Color::Magenta),
        "test4".color(Color::Cyan),
        "test5".color(Color::Red),
    );
    assert_eq!(result, expected);
}

#[test]
fn test_format_path_split() {
    let input = PathBuf::from("/path/to/test1/test2/test3/test4/test5");
    let options = FormatOptions {
        color: false,
        split: true,
        blankline: false,
    };

    let formatted_path = format_path(input, options);

    assert_eq!(
        formatted_path,
        " / path / to / test1 / test2 / test3 / test4 / test5"
    );
}

#[test]
fn test_format_path_blankline() {
    let input = PathBuf::from("/path/to/test1/test2/test3/test4/test5");
    let options = FormatOptions {
        color: false,
        split: false,
        blankline: true,
    };

    let formatted_path = format_path(input, options);

    assert_eq!(formatted_path, "\n/path/to/test1/test2/test3/test4/test5\n");
}
