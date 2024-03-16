use std::error::Error;
use std::process::Command;
use crate::config::PkgMgrs;

pub fn install_pkgs(package_manager: &PkgMgrs, packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    match package_manager {
        PkgMgrs::Pacman => pacman(packages),
    }
}

fn pacman(packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let result = run_install_command(true, "pacman", vec![ "-S", "--noconfirm"], packages);

    if result.is_err() {
        return result;
    }

    Ok(())
}

fn run_install_command(sudo: bool, pkg_mgr_string: &str, args: Vec<&str>, packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new({
        if sudo {
            "sudo"
        } else {
            pkg_mgr_string
        }
    });

    if sudo {
        command.arg(pkg_mgr_string);
    }

    let spawn_result = command.args(args)
                              .args(packages)
                              .spawn();

    match spawn_result {
        Err(e) => return Err(format!("unable to spawn child process: {}", e).into()),
        Ok(mut child) => match child.wait() {
            Err(e) => return Err(format!("failed to wait on child install process: {}", e).into()),
            Ok(_) => return Ok(()),
        }
    }
}
