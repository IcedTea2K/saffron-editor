use crate::editor;
use std::fs::File;
use std::process::{exit, Command, Stdio};
use std::io;
// use std::fs::File;
// use termios::

pub fn start_editor() -> Result<(), io::Error>{
    editor::start_editor();
    let tty = catpure_tty()?;

    Ok(())
}

fn catpure_tty() -> Result<String, io::Error>{
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
    Err(io::Error::other("Cannot Capture TTY"))
}
