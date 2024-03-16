use std::error::Error;
use crate::config::PkgMgrs;

pub fn install_pkgs(package_manager: &PkgMgrs, packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    match package_manager {
        PkgMgrs::Pacman => pacman(packages),
        _ => Err("package manager isn't yet supported. Maybe check configuration for spelling mist)akes?".into()),
    }
}

fn pacman(packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    todo!()
}
