use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FrontierArgs {
    /// The .toml file to use.
    pub config_file: String,
}
