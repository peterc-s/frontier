use std::error::Error;
use std::process::Command;
use std::process;
use colored::Colorize;
use crate::config::{PkgMgrs, Config};
use clap::Args;
use std::fs;

#[derive(Args)]
pub struct Install {
    #[clap(help = "Sets the configuration toml file.")]
    config: String,
}

impl Install {
    pub fn run(&self) {
        let config_contents = fs::read_to_string(&self.config).unwrap_or_else(|err| {
            eprintln!("Error reading config file: {}", err);
            process::exit(1);
        });

        // parse toml from input file
        let config = Config::build(config_contents).unwrap_or_else(|err| {
            eprintln!("Error parsing config: {}", err.message());
            process::exit(1);
        });

        let pkgs_to_install = config.pkgs_to_install().unwrap_or_else(|err| {
            eprintln!("Error parsing config: {}", err);
            process::exit(1);
        });

        // gets the package manager from the file
        let pkg_mgr = config.pkg_mgr().unwrap_or_else(|err| {
            eprintln!("Error parsing config: {}", err);
            process::exit(1);
        });

        // gets the args to the package manager
        let args_to_pkg_mgr = config.args_to_pkg_mgr().unwrap_or_else(|err| {
            eprintln!("Error parsing config: {}", err);
            process::exit(1);
        });

        println!("{} Running install command.", "[frontier]".bold().purple());
        install_pkgs(pkg_mgr, args_to_pkg_mgr, pkgs_to_install).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            process::exit(1);
        });
    }
}

/// Used to install the packages in the packages list using a given supported
/// package manager.
///
/// # Example
/// ```
/// use frontier::install;
/// use frontier::config::PkgMgrs;
///
/// assert!(install::install_pkgs(&PkgMgrs::Yay, vec!["--noconfirm"], vec!["neovim", "neofetch"]).is_ok());
/// ```
///
/// # Errors
/// Can lead to errors if the command was malformed, if a process couldn't spawn,
/// or std::process::Child.wait() fails.
pub fn install_pkgs(package_manager: &PkgMgrs, args: Vec<&str>, packages: Vec<&str>) -> Result<(), Box<dyn Error>> {
    match package_manager {
        PkgMgrs::Pacman => pacman(packages, args),
        PkgMgrs::Yay => yay(packages, args),
    }
}

fn pacman(packages: Vec<&str>, mut args: Vec<&str>) -> Result<(), Box<dyn Error>> {
    if !args.contains(&"-S") {
        args.append(&mut vec!["-S"]);
    }

    run_install_command(true, "pacman", args, packages)?;

    Ok(())
}

fn yay(packages: Vec<&str>, mut args: Vec<&str>) -> Result<(), Box<dyn Error>> {
    if !args.contains(&"-S") {
        args.append(&mut vec!["-S"]);
    }

    run_install_command(false, "yay", args, packages)?;

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
