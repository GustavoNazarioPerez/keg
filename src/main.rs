mod cli;
mod commands;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(command) = args.get(1).map(String::as_str) else {
        eprintln!("error: missing command");
        eprintln!("{}", cli::USAGE);
        eprintln!("For more information, try `--help`.");
        return ExitCode::from(2);
    };

    if matches!(command, "--help" | "-h") {
        println!("{}", cli::HELP);
        return ExitCode::SUCCESS;
    }

    match command.parse::<commands::Command>() {
        Ok(_) => eprintln!("unimplemented"),
        Err(e) => {
            eprintln!("{e}");
            eprintln!("{}", cli::USAGE);
            eprintln!("For more information, try `--help`.");
            return ExitCode::from(2);
        }
    }

    ExitCode::SUCCESS
}
