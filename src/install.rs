use std::error::Error;
use std::process::Command;
use crate::config::PkgMgrs;

/// Used to install the packages in the packages list using a given supported
/// package manager.
///
/// # Example
/// ```
/// use frontier::install;
/// use frontier::config::PkgMgrs;
///
/// assert!(install::install_pkgs(&PkgMgrs::Yay, vec!["neovim", "neofetch"]).is_ok());
/// ```
///
/// # Errors
/// Can lead to errors if the command was malformed, if a process couldn't spawn,
/// or std::process::Child.wait() fails.
pub fn install_pkgs(package_manager: &PkgMgrs, packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    match package_manager {
        PkgMgrs::Pacman => pacman(packages),
        PkgMgrs::Yay => yay(packages),
    }
}

fn pacman(packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    run_install_command(true, "pacman", vec!["-S", "--noconfirm"], packages)?;

    Ok(())
}

fn yay(packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    run_install_command(false, "yay", vec!["-S", "--noconfirm"], packages)?;

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
