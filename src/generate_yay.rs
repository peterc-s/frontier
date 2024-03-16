use clap::Args;
use std::{process, process::Command};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use frontier::generate;

#[derive(Args)]
#[clap(about = "Generates a frontier configuration file from the yay AUR helper. Recommended over `generate-pacman`.")]
pub struct GenerateYay {
    #[clap(help = "Specifies the output path.")]
    output_file: String,
}

impl GenerateYay {
    pub fn run(&self) {
        let mut package_list = get_package_list().unwrap_or_else(|err| {
            eprintln!("Error whilst getting package list: {}", err);
            process::exit(1);
        });

        package_list = generate::format_package_list(package_list);

        let mut output_string = r#"[package_manager]
name = "yay"
args = ["--noconfirm"]

[pkgs]"#.to_string();

        output_string = format!("{}\n{}", output_string, package_list);

        let mut out_file = File::create(&self.output_file).unwrap_or_else(|err| {
            eprintln!("Error whilst writing to output file: {}", err);
            process::exit(1);
        });

        out_file.write_all(output_string.as_bytes()).unwrap_or_else(|err| {
            eprintln!("Error whilst writing to output file: {}", err);
            process::exit(1);
        });
    }
}

fn get_package_list() -> Result<String, Box<dyn Error>> {
    let mut command = Command::new("yay");

    command.arg("-Qeq");

    let output = command.output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}

