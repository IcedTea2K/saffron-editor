use std::{fs, io::{self, ErrorKind}, usize};

#[derive(Copy, Clone)]
pub enum Key {
    ESCAPE,
    DEL,
    ENTER,
    CTRL,
    SHIFT,
    OPTION,
    CMD,
    ASCII(char),
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Action {
    APPEND(char),
    DELETE,
    NEWLINE,
    MOVE_LEFT,
    MOVE_RIGHT,
    MOVE_UP,
    MOVE_DOWN,
    NONE,
    // potentially HIGHLIGHT, DELETE, PASTE
}

#[derive(Copy, Clone)]
pub enum Mode {
    EDIT,
    VISUAL,
    NORMAL,
    COMMAND,
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum State {
    START,
    IN_SESSION,
    EXIT
}

pub struct Editor {
    state: State,
    mode: Mode,

    buffer: Vec<String>, // TODO: should have internal buffer for something
                    // Right now, it's just storing the last event
    action: Action,
    row: u32,
    col: u32,
}

impl Action {
    pub fn is_none(&self) -> bool {
        match self {
            Action::NONE => true,
            _ => false,
        } 
    }
}

impl Editor {    
    pub fn new() -> Self {
        Editor {
            state: State::START,
            mode: Mode::NORMAL,
            buffer: Vec::new(),
            action: Action::NONE,
            row: 0,
            col: 0
        }
    }

    pub fn get_row(&self) -> u32 {
        self.row
    }

    pub fn get_col(&self) -> u32 {
        self.col
    }

    pub fn get_state(&self) -> State {
        self.state 
    }

    pub fn get_action(&mut self) -> Action {
        let returned_action = self.action;

        if !self.action.is_none() {
            self.action = Action::NONE;
        }

        returned_action
    }

    pub fn process_event(&mut self, key: Key) { // TODO: -> Result<...> instead
        match key {
            Key::ASCII(c) => {
                match self.mode {
                    Mode::NORMAL => {
                        self._process_special_ascii(c);
                    }
                    Mode::EDIT => {
                        self._process_normal_ascii(c);
                    }
                    Mode::VISUAL => {

                    }
                    Mode::COMMAND => {

                    }
                }
            }
            Key::DEL => {
                self.action = Action::DELETE;
            }
            Key::ENTER => {
                self.action = Action::NEWLINE;
                self.row += 1;
            }
            Key::ESCAPE => {
                self.mode = Mode::NORMAL;
            }
            _ => {
                // TODO: do something about control characters
            }
        }
    }

    pub fn start(&mut self) {
        self.state = State::IN_SESSION;
        // should do some internal setup
    }

    pub fn add_file(&mut self, file: &String) -> Result<(), io::Error>{
        match fs::read_to_string(file) {
            Ok(buf) => self._split_and_set_buffer(buf),
            Err(_e) => return Err(io::Error::from(ErrorKind::NotFound))
        }
        Ok(())
    }
    
    pub fn get_line_content(&self, line: usize) -> Result<String, io::Error> {
        if line == 0 || line > self.buffer.len() {
            return Err(io::Error::new(io::ErrorKind::Other, "Unknown line"))
        } 
        Ok("".to_string())
    }

    pub fn get_all_lines(&self) -> &Vec<String> {
        &self.buffer
    }

    pub fn exit(&mut self) {
        self.state = State::EXIT;
        // should do some internal clean up
    }

    fn _split_and_set_buffer(&mut self, buf: String) {
        let mut temp_string = String::new();
        for c in buf.chars() {
            match c {
                '\n' => {
                    self.buffer.push(temp_string);
                    temp_string = String::new();
                },
                _ => temp_string.push(c)
            }
        }
    }

    fn _process_special_ascii(&mut self, key: char) {
        match key {
            'q' => {
                self.state = State::EXIT;
            }
            'i' => {
                self.mode = Mode::EDIT;
            }
            'h' => {
                if self.col != 0 {
                    self.col -= 1;
                }
                self.action = Action::MOVE_LEFT;
            }
            'j' => {
                self.row += 1; // TODO: do some boundary checking
                self.action = Action::MOVE_DOWN;
            }
            'k' => {
                if self.row != 0 {
                    self.row -= 1;
                }
                self.action = Action::MOVE_UP;
            }
            'l' => {
                self.col += 1; // TODO: do some boundary checking
                self.action = Action::MOVE_RIGHT;
            }
            _ => {
                // do nothing for unsupport keys
            }
        }
    }

    fn _process_normal_ascii(&mut self, key: char) {
        self.action = Action::APPEND(key);
        self.col += 1;
    }
    // TODO: print/render, allow a arbitrary front-end trait to be passed in 
}
