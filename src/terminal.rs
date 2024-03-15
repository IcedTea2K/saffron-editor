use crate::editor;
use std::fs::File;
use std::os::fd::AsRawFd;
use std::process::{Command, Stdio};
use std::io::{self, Read, Write};
use termios;

pub fn start_editor() -> Result<(), io::Error>{
    editor::start_editor();
    let tty          = catpure_tty()?;
    let tty          = &tty[..tty.len()-1]; // remove ending new-line

    let mut tty_file = File::open(tty)?;
    let raw_fd       = tty_file.as_raw_fd();
    let old_term     = termios::Termios::from_fd(raw_fd).expect("Cannot access terminal raw fd");
    let mut term     = old_term.clone();

    term.c_iflag    &= !( termios::IGNCR | termios::IXON | termios::IGNBRK | termios::BRKINT ); // Fook Window users
    term.c_oflag    &= !( termios::OCRNL | termios::OPOST );
    term.c_lflag    &= !( termios::ICANON | termios::ECHO | termios::ISIG | termios::IEXTEN );
    let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &term);

    let mut curr_char = [0;1];
    let _ = io::stdout().lock();
    while b'q' != curr_char[0] {
        // tty_file.read
        let num_read = match tty_file.read(&mut curr_char) {
            Ok(n) => n,
            Err(_) => 0
        };
        if num_read != 0 {
            let _ = io::stdout().write_all(&curr_char);
            let _ = io::stdout().flush();
        }
    }

    let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &old_term);

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
