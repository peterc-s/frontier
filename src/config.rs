use serde::Deserialize;
use toml::Value;
use phf::phf_map;

///Contains the expected configuration in the toml file.
#[derive(Deserialize, Debug)]
pub struct Config {
    pub package_manager: PackageManager,
    pub pkgs: Packages,
}

///Contains the expected structure of [package_manager].
#[derive(Deserialize, Debug)]
pub struct PackageManager {
    pub name: Value,
    pub args: Option<Value>,
}

///Contains the expected structure of [pkgs].
#[derive(Deserialize, Debug)]
pub struct Packages {
    pub install: Value,
}

///Contains the supported package managers.
#[derive(Debug)]
pub enum PkgMgrs {
    Pacman,
    Yay,
}

///Maps the string name of a package manager to its PkgMgrs equivalent.
pub static PKG_MGR_MAP: phf::Map<&str, PkgMgrs> = phf_map! {
    "pacman" => PkgMgrs::Pacman,
    "yay" => PkgMgrs::Yay,
};


impl Config {
    ///Builds a config from the contents of a toml file.
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
            .collect()
        )
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

    pub fn args_to_pkg_mgr(&self) -> Result<Vec<&str>, &'static str> {
        if self.package_manager.args.is_none() {
            return Ok(vec![]);
        }

        let args = self.package_manager.args.as_ref().unwrap();

        if !args.is_array() {
            return Err("expected field `args` of [package_manager] to be an array.");
        }

        assert!(args.as_array().is_some());

        let args_iter = args.as_array().unwrap().iter();

        Ok(args_iter
            .map(|arg| arg.as_str())
            .filter(|arg| arg.is_some())
            .map(|arg| arg.unwrap())
            .collect()
        )

    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use std::error::Error;
    use std::fs;

    fn get_test_file_contents(name: &str) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_owned() + "/resources/test/" + name)?)
    }

    // there is a lot of repetition in these tests which is fine,
    // it makes debugging a little easier.

    #[test]
    fn good_config() {
        let config_contents = get_test_file_contents("good/config.toml").unwrap();
        
        let config = Config::build(config_contents);

        assert!(config.is_ok(), "Config was: {:?}", config);
    }

    #[test]
    fn good_config_no_args() {
        let config_contents = get_test_file_contents("good/no_args.toml").unwrap();

        let config = Config::build(config_contents);

        assert!(config.is_ok(), "Config was: {:?}", config);
    }

    #[test]
    fn bad_config_missing_package_manager_name() {
        let config_contents = get_test_file_contents("bad/missing_package_manager_name.toml").unwrap();

        let config = Config::build(config_contents);

        assert!(config.is_err(), "Config was: {:?}", config);
    }

    #[test]
    fn bad_config_missing_pkgs_install() {
        let config_contents = get_test_file_contents("bad/missing_pkgs_install.toml").unwrap();

        let config = Config::build(config_contents);

        assert!(config.is_err(), "Config was: {:?}", config);
    }

    #[test]
    fn bad_config_args_not_array() {
        let config_contents = get_test_file_contents("bad/args_not_array.toml").unwrap();

        let config = Config::build(config_contents).unwrap();

        let args_to_pkg_mgr = config.args_to_pkg_mgr();

        assert!(args_to_pkg_mgr.is_err(), "Args were: {:?}", args_to_pkg_mgr);
    }

    #[test]
    fn bad_config_install_not_array() {
        let config_contents = get_test_file_contents("bad/install_not_array.toml").unwrap();

        let config = Config::build(config_contents).unwrap();

        let pkgs_to_install = config.pkgs_to_install();

        assert!(pkgs_to_install.is_err(), "Install was: {:?}", pkgs_to_install);
    }

    #[test]
    fn bad_config_unsupported_package_manager() {
        let config_contents = get_test_file_contents("bad/unsupported_package_manager.toml").unwrap();

        let config = Config::build(config_contents).unwrap();

        let pkg_mgr = config.pkg_mgr();

        assert!(pkg_mgr.is_err(), "Package manager was: {:?}", pkg_mgr);
    }

    #[test]
    fn bad_config_package_manager_not_string() {
        let config_contents = get_test_file_contents("bad/package_manager_not_string.toml").unwrap();

        let config = Config::build(config_contents).unwrap();

        let pkg_mgr = config.pkg_mgr();

        assert!(pkg_mgr.is_err(), "Package manager was: {:?}", pkg_mgr);
    }
}
