use serde::{de::DeserializeOwned, Serialize};
use std::{fs::read_to_string, path::Path};

use crate::{
    error::{LoadConfigError, LoadOrWriteConfigError, WriteConfigError},
    global::RESOURCES_DIR_NAME,
};

pub mod prelude {
    pub use crate::config::ensure_dir;
    pub use crate::config::load_config;
    pub use crate::config::write_content;
    pub use crate::config::ConfigStyle;
    pub use crate::error::Error;
    pub use crate::global::RESOURCES_DIR_NAME;
    pub use anyhow::Result;
    pub use std::path::Path;
    pub use structopt::clap::crate_name;
}

pub enum ConfigStyle {
    Toml,
    Yaml,
}

pub const CONFIG_DEFAULT_ACTIVE: &str = "dev";

pub const CONFIG_FILE_NAME: &str = "application";

pub fn load_config<T>(location: &str, style: ConfigStyle) -> Result<T, LoadConfigError>
where
    T: DeserializeOwned,
{
    match style {
        ConfigStyle::Yaml => {
            let content = read_to_string(location)?;
            let profile = serde_yaml::from_str::<T>(&content)?;
            Ok(profile)
        }
        ConfigStyle::Toml => {
            let content = read_to_string(location)?;
            let profile = toml::from_str::<T>(&content)?;
            Ok(profile)
        }
    }
}

pub fn load_or_write_config<T>(
    location: &str,
    style: ConfigStyle,
    config: &T,
) -> Result<T, LoadOrWriteConfigError>
where
    T: DeserializeOwned + Serialize,
{
    let path = Path::new(location);
    if !path.exists() {
        write_config(config, location)?;
    }
    let result = load_config(location, style)?;
    Ok(result)
}

pub fn load_or_write_content<T>(
    location: &str,
    style: ConfigStyle,
    content: &str,
) -> Result<T, LoadOrWriteConfigError>
where
    T: DeserializeOwned + Serialize,
{
    let path = Path::new(location);
    if !path.exists() {
        write_content(content, location)?;
    }
    let result = load_config(location, style)?;
    Ok(result)
}

pub fn load_app_config<T>(active: Option<&str>) -> Result<T, LoadConfigError>
where
    T: DeserializeOwned,
{
    let mut location = default_app_yaml_location(active);
    let mut path = Path::new(&location);
    if path.exists() {
        let content = read_to_string(path)?;
        let profile = serde_yaml::from_str::<T>(&content)?;
        return Ok(profile);
    }
    location = default_app_toml_location(active);
    path = Path::new(&location);
    let content = read_to_string(path)?;
    let profile = toml::from_str::<T>(&content)?;
    Ok(profile)
}

pub fn load_app_yaml_config<T>(active: Option<&str>) -> Result<T, LoadConfigError>
where
    T: DeserializeOwned,
{
    let location = default_app_yaml_location(active);
    let path = Path::new(&location);
    let content = read_to_string(path)?;
    let profile = serde_yaml::from_str::<T>(&content)?;
    Ok(profile)
}

pub fn load_app_toml_config<T>(active: Option<&str>) -> Result<T, LoadConfigError>
where
    T: DeserializeOwned,
{
    let location = default_app_toml_location(active);
    let path = Path::new(&location);
    let content = read_to_string(path)?;
    let profile = toml::from_str::<T>(&content)?;
    Ok(profile)
}

pub fn write_app_config<T>(
    config: &T,
    active: Option<&str>,
    style: ConfigStyle,
) -> Result<(), WriteConfigError>
where
    T: Serialize,
{
    ensure_res_dir()?;
    match style {
        ConfigStyle::Toml => write_app_yaml_config(config, active),
        ConfigStyle::Yaml => write_app_toml_config(config, active),
    }
}

pub fn write_app_yaml_config<T>(config: &T, active: Option<&str>) -> Result<(), WriteConfigError>
where
    T: Serialize,
{
    let yaml_str = serde_yaml::to_string(config)?;
    let path = default_app_yaml_location(active);
    std::fs::write(path, yaml_str)?;
    Ok(())
}

pub fn write_config<T>(config: &T, location: &str) -> Result<(), WriteConfigError>
where
    T: Serialize,
{
    let yaml_str = serde_yaml::to_string(config)?;
    write_content(&yaml_str, location)
}

pub fn write_content(content: &str, location: &str) -> Result<(), WriteConfigError> {
    std::fs::write(location, content)?;
    Ok(())
}

// 0.58存在bug结构体和列表字段需要放在最后，否则会报错
pub fn write_app_toml_config<T>(config: &T, active: Option<&str>) -> Result<(), WriteConfigError>
where
    T: Serialize,
{
    let toml_str = toml::to_string(config)?;
    let path = default_app_toml_location(active);
    std::fs::write(path, toml_str)?;
    Ok(())
}

pub fn default_app_yaml_location(active: Option<&str>) -> String {
    let active = active.unwrap_or(CONFIG_DEFAULT_ACTIVE);
    format!(
        "{}/{}-{}.yaml",
        RESOURCES_DIR_NAME, CONFIG_FILE_NAME, active
    )
}

pub fn default_app_toml_location(active: Option<&str>) -> String {
    let active = active.unwrap_or(CONFIG_DEFAULT_ACTIVE);
    format!(
        "{}/{}-{}.toml",
        RESOURCES_DIR_NAME, CONFIG_FILE_NAME, active
    )
}

pub fn ensure_res_dir() -> Result<(), std::io::Error> {
    let path = Path::new(RESOURCES_DIR_NAME);
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn ensure_dir(path: &Path) -> Result<(), std::io::Error> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
