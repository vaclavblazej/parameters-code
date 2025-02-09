use std::io::{self, Write};

use log::info;


pub fn bar(value: u32, maxval: u32) -> String {
    let mind_value = value.min(maxval);
    let filledstr = "█".repeat(mind_value as usize);
    let emptystr = "░".repeat((maxval-mind_value) as usize);
    let mut res = String::new();
    res.push_str(&filledstr);
    res.push_str(&emptystr);
    res
}

#[derive(Debug)]
pub struct ProgressDisplay {
    name: String,
    max: u32,
    value: u32,
    bar_length: u32,
}

impl ProgressDisplay {

    pub fn new(name: &str, max: u32) -> Self {
        Self {
            name: name.into(),
            max,
            value: 0,
            bar_length: 40,
        }
    }

    pub fn increase(&mut self, value: u32) {
        self.value += value;
        let bar_progress = (self.value * self.bar_length) / self.max;
        let bar_string = bar(bar_progress, self.bar_length);
        print!("\r{}: {} ({}/{})", self.name, bar_string, self.value, self.max);
        io::stdout().flush();
    }

    pub fn done(self) {
        println!();
        info!("{} done ({})", self.name, self.value);
    }

}

