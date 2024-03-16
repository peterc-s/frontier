use serde::Deserialize;
use toml::Value;
use phf::phf_map;

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

///Contains the supported package managers
#[derive(Debug)]
pub enum PkgMgrs {
    Pacman,
    Yay,
}

pub static PKG_MGR_MAP: phf::Map<&str, PkgMgrs> = phf_map! {
    "pacman" => PkgMgrs::Pacman,
    "yay" => PkgMgrs::Yay,
};


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

    ///Gets the packages to install from the \[pkgs\] section as a vec,
    ///filters out any non-string elements.
    pub fn pkgs_to_install(&self) -> Result<Vec<&str>, &'static str> {
        if !self.pkgs.install.is_array() {
            return Err("expected field `install` of [pkgs] to be an array.");
        }

        assert!(self.pkgs.install.as_array().is_some());

        let pkgs_iter = self.pkgs.install.as_array().unwrap().iter();

        Ok(pkgs_iter
            .map(|pkg| pkg.as_str())
            .filter(|pkg| pkg.is_some())
            .map(|pkg| pkg.unwrap())
            .collect())
    }

    pub fn pkg_mgr(&self) -> Result<&PkgMgrs, &'static str> {
        if !self.package_manager.name.is_str() {
            return Err("expected field `name` of [package_manager] to be a string.");
        }

        assert!(self.package_manager.name.as_str().is_some());

        if !PKG_MGR_MAP.contains_key(self.package_manager.name.as_str().unwrap()) {
            return Err("package manager isn't yet supported. Maybe check configuration for spelling mistakes?");
        }
        
        Ok(PKG_MGR_MAP.get(
                self.package_manager.name
                .as_str()
                .unwrap()
            ).expect("couldn't find package manager despite passing check."))
    }
}
