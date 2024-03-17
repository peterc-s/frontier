use clap::Args;
use std::{process, process::Command};
use std::fs::File;
use std::io::Write;
use std::error::Error;

#[derive(Args)]
pub struct Generate {
    package_manager: String,
    output_file: String,
}

impl Generate {
    pub fn run(&self) {
        match self.package_manager.as_str() {
            "pacman" => self.gen("pacman", vec!["-Qeq"], Some(vec!["--noconfirm"])),
             "yay" => self.gen("yay", vec!["-Qeq"], Some(vec!["--noconfirm"])),
             s => {
                 eprintln!("Error: unsupported package manager '{}'", s);
                 process::exit(1);
             }
        }
    }
}

fn get_command_output(command: &str, args: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let mut command = Command::new(command);

    command.args(args);

    let output = command.output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}

fn format_package_list(mut package_list: String) -> String {
    package_list = package_list.lines().map(|line| format!("\t\"{}\",\n", line)).collect();
    package_list = format!("install = [\n{}]", package_list);

    package_list
}

impl Generate {
    /// Generates the configuration file using the given command, args to the command,
    /// and the args that should be in the configuration file.
    fn gen(&self, command: &str, args: Vec<&str>, config_args: Option<Vec<&str>>) {
        let mut package_list = get_command_output(command, args).unwrap_or_else(|err| {
            eprintln!("Error whilst getting package list: {}", err);
            process::exit(1);
        });

        package_list = format_package_list(package_list);

        let mut output_string = format!(r#"[package_manager]
name = "{}"{}

[pkgs]"#, self.package_manager, match config_args {
    Some(args) => format!("\n{}",format_args_field(args)),
    None => "".to_string(),
}).to_string();

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

/// Takes in a `Vec<&str>` of args and returns the string of the args field
/// for the config file.
fn format_args_field(args: Vec<&str>) -> String {
    let mut arg_field = args.iter()
            .map(|arg| format!("\"{}\", ", arg))
            .collect::<Vec<_>>()
            .concat()
            .as_mut_str()
            .chars()
            .as_str()
            .to_string();

    arg_field.pop();
    arg_field.pop();

    format!("args = [{}]", arg_field)
}
