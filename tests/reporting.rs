extern crate tomatoton;
extern crate chrono;

use tomatoton::{Pomodoro, Database};
use std::time;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn write_pomodoro(path : &Path) {
    // Write pomodoro
    let now = chrono::Utc::now();
    let mut pomodoro = Pomodoro::new_task(now, time::Duration::from_secs(60 * 25), String::from("A simple task"));
    pomodoro.update(now.checked_add_signed(chrono::Duration::minutes(5)).unwrap());
    pomodoro.abort();


    let mut db = Database::open(&path).expect("Impossible to open database test_once.pom");
    db.serialize(&pomodoro).unwrap();
}

fn count_lines(path: &Path) -> usize {
    let file = fs::File::open(&path).expect("Impossible to check file test_append.pom");
    let f = BufReader::new(file);
    f.lines().count()
}

#[test]
fn write_once_pomodoro() {
    //Erase any database file that would exist
    let path = Path::new("tests/test_once.pom");
    if path.exists() {
        fs::remove_file(path).expect("Impossible to delete file test_once.pom");
    }
    write_pomodoro(&path);

    assert_eq!(count_lines(&path), 1);
}

#[test]
fn append_pomodoro() {
    //Add record to a test pomodoro
    let path = Path::new("tests/test_append.pom");


    //Check if number of lines in file has increased
    let nb_lines_before = count_lines(&path);
    write_pomodoro(&path);

    //Check if number of lines in file has increased
    let nb_lines_after = count_lines(&path);

    assert_eq!(nb_lines_after, nb_lines_before + 1);

}
