use std::env;
use std::process;
use std::io;
use std::path::{PathBuf, MAIN_SEPARATOR_STR};

fn run() -> Result<bool, io::Error> {
    let current_dir: PathBuf = env::current_dir()?;

        // PathBuf を文字列の配列に変換
    let path_components: Vec<String> = current_dir
        .iter()
        .skip(1)
        .map(|component| component.to_string_lossy().to_string()) // 各コンポーネントを String に変換
        .collect();

    let concatenated_path: String = path_components.join(MAIN_SEPARATOR_STR);

    println!("{}{}", MAIN_SEPARATOR_STR, concatenated_path);
    Ok(true)
}

fn main() {
    if run().is_err() {
        process::exit(1);
    }
}

