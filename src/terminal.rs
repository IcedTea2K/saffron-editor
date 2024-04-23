use crate::editor::{*};
use std::fs::File;
use std::os::fd::AsRawFd;
use std::process::{Command, Stdio};
use std::io::{self, Read, Write, Bytes};
use std::env;
use termios::{self, Termios};

mod ansii_const;
use ansii_const::*;

pub fn start_editor() -> Result<(), io::Error>{
    let mut drawer = Drawer::new();

    loop { // main program loop
        match drawer.get_editor_state() {
            State::START => {
                drawer.start()?;
            }
            State::IN_SESSION => {
                match drawer.take_input() {
                    Ok(()) => (),
                    Err(_e) => {
                        // do nothing about the error for now
                    }
                }

                match drawer.render_editor() {
                    Ok(()) => (),
                    Err(_e) => {
                        // do nothing about the error for now
                    }
                }
            },
            State::EXIT => {
                drawer.exit();
                break;
            }
        }
    }

    Ok(())
}

struct Drawer {
    raw_fd: i32,
    old_term: Termios,
    input_stream: Bytes<File>,
    editor: Editor
}

impl Drawer {
    pub fn new() -> Self {
        let tty          = Self::_catpure_tty().expect("Something went wrong: Cannot capture tty");

        let tty_file     = File::open(tty).expect("Something went wrong: Cannot access tty");
        let raw_fd       = tty_file.as_raw_fd();
        let input_stream = tty_file.bytes();
        let old_term     = termios::Termios::from_fd(raw_fd).expect("Something went wrong: Cannot access terminal raw fd");
        let mut cur_term     = old_term.clone();

        let editor       = Editor::new();

        cur_term.c_iflag    &= !( termios::IGNCR | termios::IXON | termios::IGNBRK | termios::BRKINT ); // Fook Window users
        cur_term.c_oflag    &= !( termios::OCRNL | termios::OPOST );
        cur_term.c_lflag    &= !( termios::ICANON | termios::ECHO | termios::ISIG | termios::IEXTEN );
        termios::tcsetattr(raw_fd, termios::TCSAFLUSH, &cur_term).expect("Something went wrong: Cannot enter raw mode");

        Drawer {
            raw_fd,
            old_term,
            input_stream, 
            editor
        } 
    }

    pub fn start(&mut self) -> Result<(), io::Error>{
        self.editor.start();

        self._enter_alternate_screen();
        self._process_args()?;
        self.refresh_screen()
    }

    pub fn exit(&mut self) {
        self.editor.exit();
        print!("{}", THICK_CURSOR);

        self._exit_alternate_screen().expect("Something went wrong: cannot quite alternate buffer");
        termios::tcsetattr(self.raw_fd, termios::TCSAFLUSH, &self.old_term).expect("Something went wrong: cannot close editor cleanly");
    }

    pub fn refresh_screen(&self) -> Result<(), io::Error> {
        // Setup screen
        print!("{}", CLEAR_SCREEN);
        print!("{}", JUMP_TO_ORG);
 

        // Print out the content
        let lines = self.editor.get_all_lines();
        for (row, line) in lines.iter().enumerate() {
            self._render_line(row + 1, line)?;
        }

        print!("{}", JUMP_TO_ORG);
        io::stdout().flush()?;
        Ok(())
    }

    pub fn take_input(&mut self) -> io::Result<()>{
        let input = match self.input_stream.next() {
            Some(v) => v.unwrap(),
            None    => 0,
        };

        let key = self._parse_input(input)?;
        self.editor.process_key(key);

        Ok(())
    }

    pub fn render_editor(&mut self) -> Result<(), io::Error>{
        let action = self.editor.get_action();
        let curr_row: usize;
        let curr_col: usize;
        let curr_line: &String;

        if action.is_none() {
            return Ok(());
        }

        match action {
            Action::APPEND(_c) => {
                curr_row = self.editor.get_row();
                curr_line = self.editor.get_current_line();

                self._render_line(curr_row + 1, curr_line)?;

                // offset by 1 to stay right
                print!("{}", JUMP_TO_HORIZONTAL(self.editor.get_col() + 1));
            }
            Action::DELETE => {
                print!("\x08 \x08");
            }
            Action::NEWLINE => {
                print!("\n\r");
            }
            Action::MOVE_UP | Action::MOVE_DOWN | Action::MOVE_LEFT | Action::MOVE_RIGHT => {
                curr_row = self.editor.get_row();
                curr_col = self.editor.get_col();

                print!("{}", JUMP_TO((curr_row + 1, curr_col + 1)));
            }
            Action::SWITCH_MODE => {
                self._render_mode(self.editor.get_mode()); 
            }
            _ => {
                // do nothing for now
            }
        }

        io::stdout().flush()?;
        Ok(())
    }

    pub fn get_editor_state(&self) -> State {
        self.editor.get_state()
    }

    /// Render the mode of the Editor. Usually used when entering new mode
    fn _render_mode(&self, mode: Mode) {
        match mode {
            Mode::EDIT => {
                print!("{}", THIN_CURSOR);
            }
            Mode::VISUAL => {

            }
            Mode::NORMAL => {
                print!("{}", THICK_CURSOR);
            }
            Mode::COMMAND => {

            }
        } 
    }

    /// render a line with given content, at a certain location (row, col)
    fn _render_line(&self, line: usize, content: &String) -> Result<(), io::Error> {
        print!("{}", JUMP_TO((line, 0)));
        print!("{}", START_LINE);
        print!("{}", content);

        io::stdout().flush()
    }

    fn _process_args(&mut self) -> io::Result<()>{
        let args: Vec<String> = env::args().collect();

        if args.len() < 1 {
            return Err(io::Error::from(io::ErrorKind::InvalidInput))
        } 
        if args.len() == 1 {
            return Ok(()) 
        }

        for arg in &args[1..] {
            match self.editor.add_file(&arg) {
                Ok(_v) => (),
                Err(e) => eprintln!("Error {e}: {arg} cannot be open"),
            }
        }

        Ok(())
    }

    fn _enter_alternate_screen(&self) {
        print!("{}", ALT_SCREEN);
        print!("{}", JUMP_TO_ORG);
        let _ = io::stdout().flush();
    }
    fn _exit_alternate_screen(&self) -> io::Result<()>{
        print!("{}", EXIT_SCREEN);
        io::stdout().flush()?;
        Ok(())
    }

    fn _parse_input(&self, input: u8) -> io::Result<Key>{
        match input {
            b'\x1b' => Ok(Key::ESCAPE),
            b'\x7f' => Ok(Key::DEL),
            b'\x0a' => Ok(Key::ENTER),
            _       => Ok(Key::ASCII(input as char)), // TODO: is_ascii()
        }
    }

    fn _catpure_tty() -> Result<String, io::Error>{
        let mut res_tty: String;
        let proc = Command::new("tty")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute process");

        let output = proc
            .wait_with_output()
            .expect("Cannot wait for the process to finish executing");

        if output.status.success() {
            res_tty = String::from_utf8(output.stdout).expect("Cannot convert tty ouput to string");
            res_tty = res_tty.trim_end().to_string(); // remove new line at the end
            return Ok(res_tty);
        }
        Err(io::Error::other("Cannot Capture TTY"))
    }
}
