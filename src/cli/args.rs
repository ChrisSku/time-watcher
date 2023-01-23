use clap::{Parser, Subcommand};
use std::path::PathBuf;

const HELP_TEMLATE: &str = "
{before-help} 
 _   _                           _      _              +====+
| |_(_)_ __  ___    __ __ ____ _| |_ __| |_  ___ _ _   |(::)|
|  _| | '  \\/ -_)___\\ V  V / _` |  _/ _| ' \\/ -_) '_|  | )( |
 \\__|_|_|_|_\\___|    \\_/\\_/\\__,_|\\__\\__|_||_\\___|_|    |(..)|
                                                       +====+
Version: {version}
Author: {author}
{about-section}
{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Does really amazing things to great people. Now let's talk a little
more in depth about how this subcommand really works. It may take about
a few lines of text, but that's ok!",
    help_template = HELP_TEMLATE
)]
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// does start a timer
    Start {
        /// subtracts the time given from the start
        #[arg(short, long)]
        offset: Option<String>,
        /// give the exact timing
        #[arg(short, long)]
        actual: Option<String>,
    },

    Check {},

    End {
        /// subtracts the time given from the start
        #[arg(short, long)]
        offset: Option<String>,
        /// give the exact timing
        #[arg(short, long)]
        actual: Option<String>,
    },
}

impl Cli {
    pub fn get() -> Self {
        Self::parse()
    }
}
