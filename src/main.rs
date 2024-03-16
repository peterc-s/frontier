use clap::Parser;

mod config;
mod install;
mod generate_pacman;

#[derive(Parser)]
enum Frontier {
    Install(install::Install),
    GeneratePacman(generate_pacman::GeneratePacman),
}

fn main() {
    // parse command line arguments
    let args = Frontier::parse();

    // perform specific subcommand
    match args {
        Frontier::Install(install) => install.run(),
        Frontier::GeneratePacman(generate_pacman) => generate_pacman.run(),
    }

}
