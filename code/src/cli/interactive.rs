use std::collections::LinkedList;
use std::io;
use std::io::Write;

use log::{error, warn};

use crate::data::data::Data;

pub(super) fn process_command(data: &Data, mut command: LinkedList<String>) -> bool {
    if let Some(cmd) = command.pop_front() {
        match cmd.as_str() {
            "help" => {
                println!("help:");
                println!(
                    "    hasse <par_id> [<par_id> ...] - draws hasse diagram of the listed parameters"
                );
                println!("    exit - end the interactive prompt");
            }
            // "hasse" => {
            //     let mut sets = Vec::new();
            //     for i in command.iter() {
            //         let set_id: PreviewParameterId = PreviewParameterId::from(i.clone());
            //         sets.push(data.get(&set_id));
            //     }
            //     let target_dir = &self.paths.tmp_dir;
            //     let name = "drawing";
            //     let res_dot_target_file = make_drawing(data, target_dir, name, &sets, None);
            //     if let Ok(dot_target_file) = res_dot_target_file {
            //         println!("dot drawing created at '{:?}'", dot_target_file);
            //         let pdf_target_file = target_dir.join(format!("{}.pdf", name));
            //         Command::new("dot")
            //             .arg("-Tpdf")
            //             .arg(&dot_target_file)
            //             .arg("-o")
            //             .arg(&pdf_target_file)
            //             .output()
            //             .expect("dot command failed");
            //         assert!(pdf_target_file.exists());
            //         println!("pdf generated at '{:?}'", pdf_target_file);
            //     }
            // }
            "exit" => return false,
            x => warn!("unknown command '{}'", x),
        }
    }
    true
}

pub(super) fn run_interactive(data: &Data) {
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        match stdin.read_line(&mut buffer) {
            Ok(_okcode) => {
                let mut chars = buffer.as_str().chars();
                chars.next_back(); // remove the last \n character
                buffer = String::from(chars.as_str());
                let mut commands: LinkedList<String> = LinkedList::new();
                for word in buffer.as_str().split(' ') {
                    commands.push_back(String::from(word));
                }
                if !process_command(data, commands) {
                    break;
                }
                buffer = String::new();
            }
            Err(err) => {
                error!("{:?}", err);
            }
        }
    }
}
