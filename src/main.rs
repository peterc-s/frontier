use clap::Parser;
use colored::Colorize;

mod config;
mod install;
mod generate_pacman;
mod generate_yay;

#[derive(Parser)]
enum Frontier {
    Install(install::Install),
    GeneratePacman(generate_pacman::GeneratePacman),
    GenerateYay(generate_yay::GenerateYay),
}

fn main() {
    // parse command line arguments
    let args = Frontier::parse();

    // perform specific subcommand
    match args {
        Frontier::Install(install) => install.run(),
        Frontier::GeneratePacman(generate_pacman) => {
                println!("{} creating output file...", "[frontier]".bold().purple());
                generate_pacman.run();
                println!("{} success!", "[frontier]".bold().purple());
            },
        Frontier::GenerateYay(generate_yay) => {
                println!("{} creating output file...", "[frontier]".bold().purple());
                generate_yay.run();
                println!("{} success!", "[frontier]".bold().purple());
            },
    }
}
