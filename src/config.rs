use serde::Deserialize;
use toml::Value;

///Contains the expected configuration in the toml file
#[derive(Deserialize, Debug)]
pub struct Config {
    pub package_manager: PackageManager,
    pub pkgs: Packages,
}

///Contains the expected structure of [package_manager]
#[derive(Deserialize, Debug)]
pub struct PackageManager {
    pub name: Value,
}

///Contains the expected structure of [pkgs]
#[derive(Deserialize, Debug)]
pub struct Packages {
    pub install: Value,
}

impl Config {
    ///Builds a config from the contents of a toml file
    ///
    ///# Example
    ///```
    ///use frontier::config::Config;
    ///
    ///let config_contents = "\
    ///[package_manager]
    ///name = \"pacman\"
    ///
    ///[pkgs]
    ///install = [
    ///    \"neofetch\",
    ///    \"neovim\",
    ///    \"lolcat\",
    ///]".to_string();
    ///
    ///let config = Config::build(config_contents);
    ///
    ///assert!(config.is_ok());
    ///
    ///dbg!(config.unwrap());
    ///```
    pub fn build(file_contents: String) -> Result<Config, toml::de::Error> {
        let config: Config = toml::from_str(&file_contents)?;

        Ok(config)
    }

    ///Gets the packages to install from the \[pkgs\] section as an iterator,
    ///filters out any non-string elements.
    pub fn pkgs_to_install(&self) -> Result<impl Iterator<Item = &str>, &'static str> {
        if !self.pkgs.install.is_array() {
            return Err("expected field `install` of [pkgs] to be an array.");
        }

        assert!(self.pkgs.install.as_array().is_some());

        let pkgs_iter = self.pkgs.install.as_array().unwrap().iter();

        Ok(pkgs_iter
            .map(|pkg| pkg.as_str())
            .filter(|pkg| pkg.is_some())
            .map(|pkg| pkg.unwrap()))
    }
}
