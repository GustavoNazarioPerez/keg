mod cli;
mod commands;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        eprintln!("error: missing command");
        eprintln!("{}", cli::USAGE);
        eprintln!("For more information, try `--help`.");
        return ExitCode::from(2);
    };

    if matches!(command.as_str(), "--help" | "-h") {
        println!("{}", cli::HELP);
        return ExitCode::SUCCESS;
    }

    match commands::Command::from_args(&command, &mut args) {
        Ok(cmd) => cmd.run(),
        Err(e) => {
            eprintln!("{e}");
            eprintln!("{}", cli::USAGE);
            eprintln!("For more information, try `--help`.");
            return ExitCode::from(2);
        }
    }

    ExitCode::SUCCESS
}
