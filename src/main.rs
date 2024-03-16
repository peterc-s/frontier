use clap::Parser;

mod config;
mod install;

#[derive(Parser)]
enum Frontier {
    Install(install::Install),
}

fn main() {
    // parse command line arguments
    let args = Frontier::parse();

    // perform specific subcommand
    match args {
        Frontier::Install(install) => install.run(),
    }

}
