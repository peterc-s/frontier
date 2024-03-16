use clap::Parser;

mod args;
mod config;
mod install;

#[derive(Parser)]
enum Frontier {
    Install(install::Install),
}

fn main() {
    // parse command line arguments
    let args = Frontier::parse();

    match args {
        Frontier::Install(install) => install.run(),
    }

}
