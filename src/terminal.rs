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

    let _ = io::stdout().lock();
     
    let mut curr_byte = tty_file.bytes();
    let mut input     = b'0';

    while b'q' != input {
        input = match curr_byte.next() {
            Some(v) => v.unwrap(),
            None    => 0,
        };

        let _ = parse_input(input);
    }

    let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &old_term);

    Ok(())
}

fn parse_input(input: u8) -> io::Result<()>{
    let _ = match input {
        b'\x1b' => io::stdout().write_all("ESC".as_bytes()),
        _       => io::stdout().write_all(&vec![input]),
    };
    io::stdout().flush()
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
