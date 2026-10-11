mod cli;
use std::env;
use std::process::ExitCode;

// TODO: use Command enum after the first command is implemented
const SUPPORTED_COMMANDS: &[&str] = &[];

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

    if !SUPPORTED_COMMANDS.contains(&command) {
        eprintln!("error: invalid command `{command}`");
        eprintln!("{}", cli::USAGE);
        eprintln!("For more information, try `--help`.");
        return ExitCode::from(2);
    }

    ExitCode::SUCCESS
}
