use clap::Parser;

///The expected format of the arguments passed to frontier.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FrontierArgs {
    /// The .toml file to use.
    pub config_file: String,
}
