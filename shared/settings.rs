use ::anyhow::{anyhow, Result};
use ::config::Config;
use ::log::info;
use ::serde_derive::{Deserialize, Serialize};
use ::std::fs::{create_dir_all, File};
use ::std::io::prelude::*;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
use crate::linux::directories::*;

#[cfg(target_os = "macos")]
use crate::macos::directories::*;

#[cfg(target_os = "windows")]
use crate::windows::directories::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Client {
    pub server_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub listen_url: String,
    pub database_uri: String,
    pub cache_directory: String,
}


/// The struct representation of a full configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub client: Client,
    pub server: Server,
}

impl Settings {
    /// This function creates a new configuration instance and
    /// populates it with default values for every option.
    /// If a local config file already exists it is parsed and
    /// overwrites the default option values.
    /// The local config is located at "~/.config/movie.yml".
    pub fn new() -> Result<Settings> {
        let mut config = Config::new();

        config.set_default("client.server_url", "localhost:8080")?;

        // Set movie config defaults
        config.set_default("server.listen_url", "localhost:8080")?;
        config.set_default("server.database_uri", "postgres://pollbot:localhost/movie")?;
        config.set_default("server.cache_directory", default_movie_path()?)?;

        // Add in the home config file
        parse_config(&mut config)?;

        // You can deserialize (and thus freeze) the entire configuration
        Ok(config.try_into()?)
    }

    /// Save the current configuration as a file to the configuration path.
    /// The file is written to the main configuration directory of the respective OS.
    pub fn save(&self) -> Result<()> {
        let config_path = default_config_directory()?.join("movie.yml");
        let config_dir = config_path
            .parent()
            .ok_or_else(|| anyhow!("Couldn't resolve config dir"))?;

        // Create the config dir, if it doesn't exist yet
        if !config_dir.exists() {
            create_dir_all(config_dir)?;
        }

        let content = serde_yaml::to_string(self)?;
        let mut file = File::create(config_path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

fn parse_config(settings: &mut Config) -> Result<()> {
    info!("Parsing config files");
    for directory in get_config_directories()?.into_iter() {
        let path = directory.join("movie.yml");
        info!("Checking path: {:?}", &path);
        if path.exists() {
            info!("Parsing config file at: {:?}", path);
            let config_file = config::File::with_name(path.to_str().unwrap());
            settings.merge(config_file)?;
        }
    }

    Ok(())
}
