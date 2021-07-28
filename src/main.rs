mod commands;

use std::env;

const USAGE: &'static str = r#"captain-hook 

USAGE:
    captain-hook <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help         Prints this message or the help of the given subcommand(s)
    install      
    add          
    uninstall
"#;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let _ = args.remove(0);
    if args.len() == 0 {
        return println!("{}", USAGE);
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
                _ => println!("{}", USAGE),
            }
        }
        _ => println!("{}", USAGE),
    }
}
