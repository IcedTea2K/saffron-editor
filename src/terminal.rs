use crate::editor::{self, *};
use std::error::Error;
use std::fs::File;
use std::os::fd::AsRawFd;
use std::process::{Command, Stdio};
use std::io::{self, Read, Write, Bytes};
use std::env;
use termios::{self, Termios};

pub fn start_editor() -> Result<(), io::Error>{
    let mut editor = Editor::new();
    let mut drawer = Drawer::new(&mut editor);

    loop { // main program loop
        match editor.get_state() {
            State::START => {
                // open_file(&mut editor).expect("First argument should be program name");
                drawer.start();
            }
            State::IN_SESSION => {
                // render_editor(&mut editor)?;
                drawer.take_input();
            },
            State::EXIT => {
                drawer.end();
                break;
            }
        }
    }

    Ok(())
}

struct Drawer<'a> {
    editor: &'a mut Editor,
    raw_fd: i32,
    old_term: Termios,
    cur_term: Termios,
    input_stream: Bytes<File>,
}

impl<'a> Drawer<'a> {
    pub fn new(editor: &'a mut Editor) -> Self {
        let tty          = catpure_tty().expect("Something went wrong: Cannot capture tty");
        let tty          = &tty[..tty.len()-1]; // remove ending new-line

        let tty_file     = File::open(tty).expect("Something went wrong: Cannot access tty");
        let raw_fd       = tty_file.as_raw_fd();
        let input_stream = tty_file.bytes();
        let old_term     = termios::Termios::from_fd(raw_fd).expect("Something went wrong: Cannot access terminal raw fd");
        let mut cur_term     = old_term.clone();

        cur_term.c_iflag    &= !( termios::IGNCR | termios::IXON | termios::IGNBRK | termios::BRKINT ); // Fook Window users
        cur_term.c_oflag    &= !( termios::OCRNL | termios::OPOST );
        cur_term.c_lflag    &= !( termios::ICANON | termios::ECHO | termios::ISIG | termios::IEXTEN );
        let _ = termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &cur_term);

        Drawer {
            editor,
            raw_fd,
            old_term,
            cur_term,
            input_stream, 
        } 
    }

    pub fn start(&mut self) {
        self.editor.start();
        self._enter_alternate_screen();
        self.refresh_screen();
    }

    pub fn end(&self) {
        let _ = termios::tcsetattr(self.raw_fd, termios::TCSAFLUSH, &self.old_term);
        self._exit_alternate_screen();
    }

    pub fn refresh_screen(&self) {

    }

    pub fn take_input(&mut self) {
        let _ = io::stdout().lock();

        let input = match self.input_stream.next() {
            Some(v) => v.unwrap(),
            None    => 0,
        };

        let key;
        match parse_input(input) {
            Ok(v) => key = v,
            Err(_e) => {
                self.end();
                return
            }
        };

        self.editor.process_event(key);
    }

    fn _enter_alternate_screen(&self) {
        print!("\x1b[?1049h");
        print!("\x1b[1;1H");
        let _ = io::stdout().flush();
    }
    fn _exit_alternate_screen(&self) {
        print!("\x1b[?1049l");
        let _ = io::stdout().flush();
    }

}

fn render_editor(editor: &mut Editor) -> Result<(), io::Error>{
    // print!("\x1b[2J"); // clear the entire screen
    // print!("\x1b[H"); // return cursor to home pos?
    // for l in editor.get_all_lines() {
    //     print!("{}\r\n", l);
    // }
    //
    let current_action = editor.get_action();
    if current_action.is_none() {
        return Ok(());

    }
    // Temporarily disable printing input
    match current_action {
        // Action::APPEND(c) => {
        //     print!("{}", c);
        // }
        // Action::DELETE => {
        //     print!("\x08 \x08");
        // }
        // Action::NEWLINE => {
        //     print!("\r\n");
        // }
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

fn refresh_render(editor: &Editor) {

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

fn open_file(editor: &mut Editor) -> Result<(), io::Error>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        return Err(io::Error::from(io::ErrorKind::InvalidInput))
    } 
    if args.len() == 1 {
        return Ok(()) 
    }

    for arg in &args[1..] {
        match editor.add_file(&arg) {
            Ok(_v) => (),
            Err(e) => eprintln!("Error {e}: {arg} cannot be open"),
        }
    }

    Ok(())
}
