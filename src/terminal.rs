use crate::editor;
use std::fs::File;
use std::os::fd::AsRawFd;
use std::process::{Command, Stdio};
use std::io;
use termios;

pub fn start_editor() -> Result<(), io::Error>{
    editor::start_editor();
    let tty         = catpure_tty()?;
    let tty         = &tty[..tty.len()-1]; // remove ending new-line

    let tty_file    = File::open(tty)?;
    let raw_fd      = tty_file.as_raw_fd();
    let mut term    = termios::Termios::from_fd(raw_fd).expect("Cannot access terminal raw fd");

    // term.c_iflag = termios::IGNBRK;
    
    // termios::cfmakeraw(&mut term);
    println!("{:#?}", term);
    // let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &term);

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
