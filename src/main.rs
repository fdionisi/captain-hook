mod commands;
mod error;

use std::env;

use crate::error::Error;

const USAGE: &'static str = r#"captain-hook 

USAGE:
    captain-hook install [dir] (default: .hooks)
    captain-hook add <file> <cmd>
    captain-hook uninstall

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
"#;

fn print_usage() -> Result<(), Error> {
    println!("{}", USAGE);
    Ok(())
}

fn print_version() -> Result<(), Error> {
    println!("{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return print_usage();
    }

    let sub_command = args.get(1).unwrap();
    match sub_command.as_str() {
        "install" => {
            let temp_dir = String::from(".hooks");
            let dir = args.get(2).unwrap_or(&temp_dir);
            commands::install(&dir)
        }
        "uninstall" => commands::uninstall(),
        "add" => match (args.get(2), args.get(3)) {
            (Some(file_name), Some(cmd)) => commands::add(&file_name, &cmd),
            _ => print_usage(),
        },
        "--version" | "-V" => print_version(),
        "--help" | "-h" => print_usage(),
        _ => print_usage(),
    }
}
