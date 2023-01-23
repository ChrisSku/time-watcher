// a struct with a start and end property of utc DateTime<Utc>

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    date: String,
    start: DateTime<Local>,
    end: Option<DateTime<Local>>,
}

impl Time {
    pub fn new(start: DateTime<Local>) -> Time {
        Time {
            date: start.format("%Y-%m-%d").to_string(),
            start,
            end: None,
        }
    }

    pub fn get_start(&self) -> DateTime<Local> {
        self.start
    }

    pub fn get_end(&self) -> Option<DateTime<Local>> {
        self.end
    }

    pub fn set_end(&mut self, end: DateTime<Local>) {
        self.end = Some(end);
    }

    pub fn get_duration(&self) -> chrono::Duration {
        self.end
            .map_or(chrono::Local::now() - self.start, |end| end - self.start)
    }
}
