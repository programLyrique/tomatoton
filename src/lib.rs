use chrono::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum Status {
    Completed,
    Aborted
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum Kind {
    Task(String),//Description of the task
    Break,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Pomodoro {
    start_time: DateTime,
    duration : Duration,
    status : Status,
    kind : Kind,
}
