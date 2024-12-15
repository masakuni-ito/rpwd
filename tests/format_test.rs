#[cfg(test)]
mod tests {
    use colored::*;
    use rpwd::format::*;
    use std::path::MAIN_SEPARATOR;

    #[test]
    fn test_add_separator_with_divide() {
        let path_components = vec![
            "".to_string(),
            "home".to_string(),
            "user".to_string(),
            "documents".to_string(),
        ];
        let expected = vec![
            format!(" {} ", MAIN_SEPARATOR),
            format!("home {} ", MAIN_SEPARATOR),
            format!("user {} ", MAIN_SEPARATOR),
            "documents".to_string(),
        ];

        let result = add_separator(true, path_components);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_separator_without_divide() {
        let path_components = vec![
            "".to_string(),
            "home".to_string(),
            "user".to_string(),
            "documents".to_string(),
        ];
        let expected = vec![
            MAIN_SEPARATOR.to_string(),
            format!("home{}", MAIN_SEPARATOR),
            format!("user{}", MAIN_SEPARATOR),
            "documents".to_string(),
        ];

        let result = add_separator(false, path_components);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_separator_empty_path() {
        let path_components: Vec<String> = vec![];
        let expected: Vec<String> = vec![];

        let result = add_separator(true, path_components);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_color_enabled_with_cycle() {
        let path_components = vec![
            MAIN_SEPARATOR.to_string(),
            "home".to_string(),
            "user".to_string(),
            "documents".to_string(),
            "pictures".to_string(),
            "downloads".to_string(),
            "music".to_string(),
        ];

        let expected_colors = vec![
            MAIN_SEPARATOR.to_string().color(Color::Red).to_string(),
            "home".color(Color::Green).to_string(),
            "user".color(Color::Yellow).to_string(),
            "documents".color(Color::Blue).to_string(),
            "pictures".color(Color::Magenta).to_string(),
            "downloads".color(Color::Cyan).to_string(),
            "music".color(Color::Red).to_string(),
        ];

        let result = format_color(true, path_components.clone());

        for (actual, expected) in result.iter().zip(expected_colors.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_format_color_disabled() {
        let path_components = vec![
            MAIN_SEPARATOR.to_string(),
            "home".to_string(),
            "user".to_string(),
            "documents".to_string(),
        ];

        let result = format_color(false, path_components.clone());

        assert_eq!(result, path_components);
    }

    #[test]
    fn test_format_color_empty_path() {
        let path_components: Vec<String> = vec![];

        let result = format_color(true, path_components.clone());

        assert_eq!(result, path_components);
    }

    #[test]
    fn test_format_stairs_enabled() {
        let path_components = vec![
            MAIN_SEPARATOR.to_string(),
            "home".to_string(),
            "user".to_string(),
            "documents".to_string(),
        ];

        let expected = vec![
            format!("{}\n", MAIN_SEPARATOR.to_string()),
            "  home\n".to_string(),
            "    user\n".to_string(),
            "      documents".to_string(),
        ];

        let result = format_stairs(true, path_components.clone());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_stairs_disabled() {
        let path_components = vec![
            MAIN_SEPARATOR.to_string(),
            "home".to_string(),
            "user".to_string(),
            "documents".to_string(),
        ];

        let expected = path_components.clone();

        let result = format_stairs(false, path_components.clone());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_stairs_empty_path() {
        let path_components: Vec<String> = vec![];

        let expected: Vec<String> = vec![];

        let result = format_stairs(true, path_components.clone());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_stairs_single_component() {
        let path_components = vec![MAIN_SEPARATOR.to_string()];

        let expected = vec![MAIN_SEPARATOR.to_string()];

        let result = format_stairs(true, path_components.clone());

        assert_eq!(result, expected);
    }
}
