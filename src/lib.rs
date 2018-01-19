extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::time;
use serde::Serialize;



#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum Status {
    Completed,
    Aborted,
    Running,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum Kind {
    Task(String),//Description of the task
    Break,
}

impl Serialize for time::Instant {
    fn serialize<S>(&self, serializer: S) -> Result<S:Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize(self.duration_since(time::UNIX_EPOCH))
    }
}

/// Representation of a Pomodoro.
///
/// A Pomodoro is any kind of action starting at some date and lasting for some duration
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Pomodoro {
    start_time: time::Instant,
    duration: time::Duration,
    expected_duration: time::Duration,
    status: Status,
    kind: Kind,
}

impl Pomodoro {
    fn new(start_time : time::Instant, expected_duration : time::Duration, kind : Kind) -> Pomodoro {
        Pomodoro {
            start_time,
            duration: time::Duration::from_secs(0),
            expected_duration,
            status : Status::Running,
            kind
        }
    }

    fn new_task(start_time : time::Instant, expected_duration : time::Duration, description: String) -> Pomodoro {
        Pomodoro {
            start_time,
            duration: time::Duration::from_secs(0),
            expected_duration,
            status : Status::Running,
            kind : Kind::Task(description),
        }
    }

    fn new_break(start_time : time::Instant, expected_duration : time::Duration) -> Pomodoro {
        Pomodoro {
            start_time,
            duration: time::Duration::from_secs(0),
            expected_duration,
            status : Status::Running,
            kind : Kind::Break,
        }
    }

    /// Abort the Pomodoro
    fn abort(&mut self) {
        self.status = Status::Aborted;
    }

    /// Whether the Pomodoro is finished, comparing current duration with the expected duration
    fn is_finished(&self) -> bool {
        self.duration >= self.expected_duration
    }

    /// Update the current duration of the pomodoro
    fn update(&mut self, current_time : time::Instant) -> Status {
        match self.status {
            Status::Running => {
                self.duration = current_time - self.start_time;
                if self.duration >= self.expected_duration {
                    self.status = Status::Completed;
                }
                self.status
                },
            _ => self.status
            }

        }


    /// Write the pomodoro to the database
    fn write(&self, database: &mut Database) {

    }
}

struct Database;
