use crate::editor::*;
use std::fs::File;
use std::os::fd::AsRawFd;
use std::process::{Command, Stdio};
use std::io::{self, repeat, Read, Write};
use termios;

pub fn start_editor() -> Result<(), io::Error>{
    let mut editor = Editor::new();

    let tty          = catpure_tty()?;
    let tty          = &tty[..tty.len()-1]; // remove ending new-line

    let tty_file = File::open(tty)?;
    let raw_fd       = tty_file.as_raw_fd();
    let old_term     = termios::Termios::from_fd(raw_fd).expect("Cannot access terminal raw fd");
    let mut term     = old_term.clone();

    term.c_iflag    &= !( termios::IGNCR | termios::IXON | termios::IGNBRK | termios::BRKINT ); // Fook Window users
    term.c_oflag    &= !( termios::OCRNL | termios::OPOST );
    term.c_lflag    &= !( termios::ICANON | termios::ECHO | termios::ISIG | termios::IEXTEN );
    let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &term);

    let _ = io::stdout().lock();
     
    let mut curr_byte = tty_file.bytes();

    loop { // main program loop
        match editor.get_state() {
            State::IN_SESSION => {
                let input = match curr_byte.next() {
                    Some(v) => v.unwrap(),
                    None    => 0,
                };

                let event = match parse_input(input) {
                    Ok(v) => v,
                    Err(e) => {
                        let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &old_term);
                        return Err(e);
                    }
                };

                editor.process_event(event);
                render_editor(&mut editor).unwrap();
            },
            _ => {
                let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &old_term);
                break;
            }
        }
    }

    Ok(())
}

fn render_editor(editor: &mut Editor) -> Result<(), io::Error>{
    let current_action = editor.get_action();
    if current_action.is_none() {
        return Ok(());
    }

    match current_action.unwrap() {
        Action::APPEND(c) => {
            print!("{}", c);
        }
        Action::DELETE => {
            print!("\x08 \x08");
        }
        Action::NEWLINE => {
            let start = std::iter::repeat("\x08").take(editor.get_col() as usize).collect::<String>();
            println!();
            print!("{}", start);
        }
        _ => {
            // do nothing for now
        }
    }

    io::stdout().flush().unwrap();
    Ok(())
}

fn parse_input(input: u8) -> io::Result<Key>{
    match input {
        b'\x1b' => Ok(Key::ESCAPE),
        b'\x7f' => Ok(Key::DEL),
        b'\x0a' => Ok(Key::ENTER),
        _       => Ok(Key::ASCII(input as char)), // TODO: is_ascii()
    }
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
