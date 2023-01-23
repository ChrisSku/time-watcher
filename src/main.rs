mod cli;
mod data;

use chrono::Duration;
use cli::args::{Cli, Commands};

fn add_duration_to_datetime(
    datetime: chrono::DateTime<chrono::Local>,
    duration: chrono::Duration,
    add: bool,
) -> chrono::DateTime<chrono::Local> {
    if add {
        datetime + duration
    } else {
        datetime - duration
    }
}

// read vec time struct from csv
fn read_time_from_csv() -> Vec<data::time::Time> {
    let rdr = csv::Reader::from_path("time.csv");
    if rdr.is_err() {
        return Vec::new();
    }
    let mut times = Vec::new();
    for result in rdr.unwrap().deserialize() {
        let time: data::time::Time = result.unwrap();
        times.push(time);
    }
    times
}

fn save_time_to_csv(times: Vec<data::time::Time>) {
    let mut wtr = csv::Writer::from_path("time.csv").unwrap();
    for time in times {
        wtr.serialize(time).unwrap();
    }
}

fn main() {
    let cli = Cli::get();
    let mut times = read_time_from_csv();
    let has_last_time_ended = match times.last() {
        Some(time) => time.get_end().is_some(),
        None => true,
    };
    let last_time = times.pop();
    match &cli.command {
        Some(Commands::Check {}) | None => {
            let mut duration = times
                .iter()
                .map(|time| time.get_duration())
                .fold(Duration::zero(), |acc, duration| acc + duration);
            if let Some(time) = &last_time {
                duration = duration + time.get_duration();
            }
            println!(
                "You have worked for: {} hours {} minutes and {} seconds Today!",
                duration.num_hours(),
                duration.num_minutes() % 60,
                duration.num_seconds() % 60
            );

            if has_last_time_ended {
                println!("You have not started a time yet!");
                return;
            }
            let last_time = last_time.unwrap();
            let duration = last_time.get_duration();
            println!(
                "You have been working for: {} hours {} minutes and {} seconds",
                duration.num_hours(),
                duration.num_minutes() % 60,
                duration.num_seconds() % 60
            );
        }
        Some(Commands::Start { offset, actual: _ }) => {
            if !has_last_time_ended {
                println!(
                    "You have not ended your last time yet! Started: {}",
                    last_time.unwrap().get_start()
                );
                return;
            }
            let mut start = chrono::Local::now();
            if let Some(off) = offset {
                let time_str = off.replace(['+', '-'], "");
                let iss_addition = off.starts_with('+');
                let std_duration = duration_str::parse(time_str.as_str()).unwrap();
                let duration = Duration::from_std(std_duration).unwrap();
                start = add_duration_to_datetime(start, duration, iss_addition);
                println!("Start time: {}", start);
            }
            if let Some(time) = last_time {
                times.push(time);
            }
            times.push(data::time::Time::new(start));
            save_time_to_csv(times);
        }
        Some(Commands::End {
            offset: _,
            actual: _,
        }) => {
            if has_last_time_ended {
                println!("You have not started a time yet!");
                return;
            }
            let mut last_time = last_time.unwrap();
            last_time.set_end(chrono::Local::now());
            let duration = last_time.get_duration();
            println!(
                "Ended time: {}. You had been working for {} hours {} minutes and {} seconds",
                last_time.get_end().unwrap(),
                duration.num_hours(),
                duration.num_minutes() % 60,
                duration.num_seconds() % 60,
            );
            times.push(last_time);
            save_time_to_csv(times);
        }
    }

    // // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {}", name);
    // }

    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }

    // // You can see how many times a particular flag or argument occurred
    // // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    // // You can check for the existence of subcommands, and if found use their
    // // matches just as you would the top level cmd
    // if let Some(Commands::Start { offset, actual: _ }) = &cli.command {
    //     if let Some(off) = offset {
    //         println!("Subtracting {} of the current time", off);
    //     } else {
    //         println!("Not printing testing lists...");
    //     }
    // }

    // Continued program logic goes here...
}
