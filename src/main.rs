use clap::Parser;

mod config;
mod install;
mod generate;

#[derive(Parser)]
#[command(version)]
enum Frontier {
    Install(install::Install),
    Generate(generate::Generate),
}

fn main() {
    // parse command line arguments
    let args = Frontier::parse();

    // perform specific subcommand
    match args {
        Frontier::Install(install) => install.run(),
        Frontier::Generate(generate) => generate.run(),
    }
}
