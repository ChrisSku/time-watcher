mod cli;
use cli::args::{Cli, Commands};

fn main() {
    let cli = Cli::get();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(Commands::Start { offset, actual: _ }) = &cli.command {
        if let Some(off) = offset {
            println!("Subtracting {} of the current time", off);
        } else {
            println!("Not printing testing lists...");
        }
    }

    // Continued program logic goes here...
}
