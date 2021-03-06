use ::anyhow::{anyhow, Result};
use ::std::path::{Path, PathBuf};

fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().ok_or_else(|| anyhow!("Couldn't resolve home dir"))
}

pub fn default_config_directory() -> Result<PathBuf> {
    Ok(get_home_dir()?.join("Library/Preferences"))
}

pub fn get_config_directories() -> Result<Vec<PathBuf>> {
    Ok(vec![
        default_config_directory()?,
        Path::new(".").to_path_buf(),
    ])
}

pub fn default_movie_path() -> Result<String> {
    let path = get_home_dir()?.join(".local/share/movie");
    path.to_str().map_or_else(
        || Err(anyhow!("Failed to parse log path (Weird characters?)")),
        |v| Ok(v.to_string()),
    )
}
