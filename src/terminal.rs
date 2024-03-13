use crate::editor;
use std::process::{exit, Command, Stdio};
use std::io::Error;

pub fn start_editor() {
    editor::start_editor();
    let tty = match catpure_tty() {
        Ok(v) => Some(v),
        Err(_e) => None,
    };

    if tty.is_none() {
        eprint!("Unable to start the editor");
        exit(1)
    }
}

fn catpure_tty() -> Result<String, Error>{
    let proc = Command::new("tty")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute process");

    let output = proc
        .wait_with_output()
        .expect("Cannot wait for the process to finish executing");

    if output.status.success() {
        return Ok(String::from_utf8(output.stdout).expect("Cannot convert tty ouput to string"));
    }
    Err(Error::other("Cannot Capture TTY"))
}
