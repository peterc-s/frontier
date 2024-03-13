use serde::Deserialize;
use toml::Value;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub package_manager: PackageManager,
    pub pkgs: Packages,
}

#[derive(Deserialize, Debug)]
pub struct PackageManager {
    pub name: Value,
}

#[derive(Deserialize, Debug)]
pub struct Packages {
    pub install: Value,
}

impl Config {
    pub fn build(file_contents: String) -> Result<Config, toml::de::Error> {
        let config: Config = toml::from_str(&file_contents)?;

        Ok(config)
    }
}
