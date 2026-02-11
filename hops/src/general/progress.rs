//! Utility to display an ascii progress bar.

use std::io::{self, Write};
use std::sync::Mutex;
use termion::{clear, cursor};

use log::info;

static PROGRESS_STATE: Mutex<Option<String>> = Mutex::new(None);

pub fn with_progress_suspended<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let state = PROGRESS_STATE.lock().unwrap();
    if let Some(ref bar_str) = *state {
        print!("{}{}", clear::CurrentLine, cursor::Left(u16::MAX));
        io::stdout().flush().ok();
    }
    let result = f();
    if let Some(ref bar_str) = *state {
        print!("{}", bar_str);
        io::stdout().flush().ok();
    }
    result
}

pub fn bar(value: u32, maxval: u32) -> String {
    let mind_value = value.min(maxval);
    let filledstr = "█".repeat(mind_value as usize);
    let emptystr = "░".repeat((maxval - mind_value) as usize);
    let mut res = String::new();
    res.push_str(&filledstr);
    res.push_str(&emptystr);
    res
}

#[derive(Debug)]
pub struct ProgressDisplay {
    name: String,
    max: u32,
    value: Mutex<u32>,
    bar_length: u32,
}

impl ProgressDisplay {
    pub fn new(name: &str, max: u32) -> Self {
        let pd = Self {
            name: name.into(),
            max,
            value: Mutex::new(0),
            bar_length: 40,
        };
        let bar_str = pd.format_bar();
        *PROGRESS_STATE.lock().unwrap() = Some(bar_str.clone());
        print!("{}", bar_str);
        io::stdout().flush().ok();
        pd
    }

    fn format_bar(&self) -> String {
        let value = *self.value.lock().unwrap();
        let bar_progress = (value * self.bar_length) / self.max;
        let bar_string = bar(bar_progress, self.bar_length);
        format!("{}: {} ({}/{})", self.name, bar_string, value, self.max)
    }

    pub fn increase(&self, value: u32) {
        *self.value.lock().unwrap() += value;
        let bar_str = self.format_bar();
        *PROGRESS_STATE.lock().unwrap() = Some(bar_str.clone());
        print!("\r{}", bar_str);
        io::stdout().flush().ok();
    }

    pub fn done(self) {
        let final_value = *self.value.lock().unwrap();
        *PROGRESS_STATE.lock().unwrap() = None;
        println!();
        info!("{} done ({})", self.name, final_value);
    }
}
