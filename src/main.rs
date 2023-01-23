mod cli;
mod data;

use chrono::Duration;
use cli::args::{Cli, Commands};
use data::file::{read_time_from_csv, save_time_to_csv};

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
                let is_addition = off.starts_with('+');
                let std_duration = duration_str::parse(time_str.as_str()).unwrap();
                let duration = Duration::from_std(std_duration).unwrap();
                if is_addition {
                    start = start + duration;
                } else {
                    start = start - duration;
                }
            }
            println!("Start time: {}", start);
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
}
