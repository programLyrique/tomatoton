extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate chrono;

use std::time;
use std::fs::{File, OpenOptions};
use serde::Serialize;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use csv::WriterBuilder;



#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Status {
    Completed,
    Aborted,
    Running,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum Kind {
    Task(String),//Description of the task
    Break,
}

type DateTime = chrono::DateTime<chrono::Utc>;


/// Representation of a Pomodoro.
///
/// A Pomodoro is any kind of action starting at some date and lasting for some duration
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pomodoro {
    start_time: DateTime,
    duration: time::Duration,
    expected_duration: time::Duration,
    status: Status,
    kind: Kind,
}

impl Pomodoro {
    fn new(start_time : DateTime, expected_duration : time::Duration, kind : Kind) -> Pomodoro {
        Pomodoro {
            start_time,
            duration: time::Duration::from_secs(0),
            expected_duration,
            status : Status::Running,
            kind
        }
    }

    pub fn new_task(start_time : DateTime, expected_duration : time::Duration, description: String) -> Pomodoro {
        Pomodoro {
            start_time,
            duration: time::Duration::from_secs(0),
            expected_duration,
            status : Status::Running,
            kind : Kind::Task(description),
        }
    }

    pub fn new_break(start_time : DateTime, expected_duration : time::Duration) -> Pomodoro {
        Pomodoro {
            start_time,
            duration: time::Duration::from_secs(0),
            expected_duration,
            status : Status::Running,
            kind : Kind::Break,
        }
    }

    /// Abort the Pomodoro
    pub fn abort(&mut self) {
        if self.status != Status::Completed {
            self.status = Status::Aborted;
        }    
    }

    /// Whether the Pomodoro is finished, comparing current duration with the expected duration
    pub fn is_finished(&self) -> bool {
        self.duration >= self.expected_duration
    }

    /// Update the current duration of the pomodoro
    pub fn update(&mut self, current_time : DateTime) -> Status {
        match self.status {
            Status::Running => {
                let old_duration = current_time.signed_duration_since(self.start_time);
                self.duration = old_duration.to_std().unwrap();
                if self.duration >= self.expected_duration {
                    self.status = Status::Completed;
                }
                self.status
                },
            _ => self.status
            }

        }
}

pub struct Database {
    writer: csv::Writer<std::fs::File>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path : &P) -> Result<Database, Box<Error>> {
        //Open the path in append mode
        let file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)?;

        //Make it so that it adds rows to the file
        let writer = WriterBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_writer(file);
        Ok(Database {writer })
    }

    pub fn serialize(&mut self, pomodoro: &Pomodoro) -> Result<(), Box<Error>> {
        self.writer.serialize(pomodoro)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abort() {
        let now = chrono::Utc::now();
        let mut pomodoro = Pomodoro::new_task(now, time::Duration::from_secs(60 * 25), String::from("A simple task"));
        pomodoro.update(now.checked_add_signed(chrono::Duration::minutes(5)).unwrap());
        pomodoro.abort();

        assert_eq!(pomodoro.status, Status::Aborted);
    }

    #[test]
    fn test_completed() {
        let now = chrono::Utc::now();
        let mut pomodoro = Pomodoro::new_task(now, time::Duration::from_secs(60 * 25), String::from("A simple task"));
        pomodoro.update(now.checked_add_signed(chrono::Duration::minutes(25)).unwrap());

        assert_eq!(pomodoro.status, Status::Completed);
        assert!(pomodoro.duration >= pomodoro.expected_duration);
    }
}
