use clap::Parser;
use std::{fs, process};

mod args;
mod config;

fn main() {
    // parse command line arguments
    let args = args::FrontierArgs::parse();

    let config_contents = fs::read_to_string(&args.config_file).unwrap_or_else(|err| {
        eprintln!("Error reading config file: {}", err);
        process::exit(1);
    });

    // parse toml from input file
    let config = config::Config::build(config_contents).unwrap_or_else(|err| {
        eprintln!("Error parsing config: {}", err.message());
        process::exit(1);
    });

    let pkgs_to_install = config.pkgs_to_install().unwrap_or_else(|err| {
        eprintln!("Error parsing config: {}", err);
        process::exit(1);
    });

    let install_str = pkgs_to_install.join(" ");
}
