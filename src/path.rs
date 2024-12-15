use std::env;
use std::io;
use std::path::PathBuf;

pub fn get_current_dir(logical: bool, physical: bool) -> Result<Vec<String>, io::Error> {
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
