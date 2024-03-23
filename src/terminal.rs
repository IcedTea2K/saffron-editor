use crate::editor::*;
use std::fs::File;
use std::os::fd::AsRawFd;
use std::process::{Command, Stdio};
use std::io::{self, Read, Write};
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
            State::START => {
                enter_alternate_screen();
                editor.start();
            }
            State::IN_SESSION => {
                let input = match curr_byte.next() {
                    Some(v) => v.unwrap(),
                    None    => 0,
                };

                let key = match parse_input(input) {
                    Ok(v) => v,
                    Err(e) => {
                        let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &old_term);
                        return Err(e);
                    }
                };

                editor.process_event(key);
                render_editor(&mut editor)?;
            },
            State::EXIT => {
                let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &old_term);
                exit_alternate_screen();
                break;
            }
        }
    }

    Ok(())
}

fn enter_alternate_screen() {
    print!("\x1b[?1049h");
    print!("\x1b[1;1H");
    let _ = io::stdout().flush();
}

fn exit_alternate_screen() {
    print!("\x1b[?1049l");
    let _ = io::stdout().flush();
}

fn render_editor(editor: &mut Editor) -> Result<(), io::Error>{
    let current_action = editor.get_action();
    if current_action.is_none() {
        return Ok(());
    }

    match current_action {
        Action::APPEND(c) => {
            print!("{}", c);
        }
        Action::DELETE => {
            print!("\x08 \x08");
        }
        Action::NEWLINE => {
            print!("\r\n");
        }
        Action::MOVE_UP => {
            print!("\x1b[1A") 
        }
        Action::MOVE_DOWN => {
            print!("\x1b[1B") 
        }
        Action::MOVE_LEFT => {
            print!("\x1b[1D") 
        }
        Action::MOVE_RIGHT => {
            print!("\x1b[1C") 
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
