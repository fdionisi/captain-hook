mod commands;
mod error;

use std::env;

use crate::error::Error;

const USAGE: &'static str = r#"captain-hook 

USAGE:
    captain-hook <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    install      
    add          
    uninstall
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
    let mut args: Vec<String> = env::args().collect();
    let _ = args.remove(0);
    if args.len() == 0 {
        return print_usage();
    }

    let sub_command = args.remove(0);
    match sub_command.as_str() {
        "install" => {
            let temp_dir = String::from(".hooks");
            let dir = args.get(0).unwrap_or(&temp_dir);
            commands::install(&dir)
        }
        "uninstall" => commands::uninstall(),
        "add" => {
            let file_name = args.get(0);
            let cmd = args.get(1);
            match (file_name, cmd) {
                (Some(f), Some(cmd)) => commands::add(&f, &cmd),
                _ => print_usage(),
            }
        }
        "--version" | "-V" => print_version(),
        "--help" | "-h" => print_usage(),
        _ => print_usage(),
    }
}
