use crate::editor;
use std::process::{Command, Stdio};

pub fn start_editor() {
    editor::start_editor();
    match catpure_tty() {
        Ok(v) => print!("{}", v),
        Err(_e) => print!("Cannot start the editor"),
    }
}

fn catpure_tty() -> Result<String, String>{
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
    Err(String::from("Cannot obtain TTY"))
}
