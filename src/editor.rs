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
    SWITCH_MODE,
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
    row: usize,
    col: usize,

    virtual_col: usize
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
            col: 0,
            virtual_col: 0
        }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_col(&self) -> usize {
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

    pub fn process_key(&mut self, key: Key) { // TODO: -> Result<...> instead
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
                self.action = Action::SWITCH_MODE;
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
    
    /// Return the current line of cursor in buffer
    pub fn get_current_line(&self) -> &String {
        &self.buffer[self.row]
    }

    pub fn get_all_lines(&self) -> &Vec<String> {
        &self.buffer
    }

    /// Return the current mode of the editor
    pub fn get_mode(&self) -> Mode {
        self.mode
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
                self.action = Action::SWITCH_MODE;
            }
            'h' => {
                if self.col > 0 {
                    self.col -= 1;
                    self.virtual_col = self.col;
                    self.action = Action::MOVE_LEFT;
                }
            }
            'j' => {
                if self.row < self.buffer.len() - 1 {
                    self.row += 1;
                    self._update_cursor();
                    self.action = Action::MOVE_DOWN;
                }
            }
            'k' => {
                if self.row > 0 {
                    self.row -= 1;
                    self._update_cursor();
                    self.action = Action::MOVE_UP;
                }
            }
            'l' => {
                if self.col < self.buffer[self.row].len() - 1 {
                    self.col += 1;
                    self.virtual_col = self.col;
                    self.action = Action::MOVE_RIGHT;
                }
            }
            '$' => {
                self.virtual_col = usize::MAX;
                self._update_cursor();

                self.action = Action::MOVE_RIGHT;
            }
            '0' => {
                self.col = 0;
                self.virtual_col = 0;

                self.action = Action::MOVE_LEFT;
            }
            _ => {
                // do nothing for unsupport keys
            }
        }
    }

    /// Process the normal key and edit the buffer
    fn _process_normal_ascii(&mut self, key: char) {
        self.action = Action::APPEND(key);
        self.buffer[self.row].insert(self.col, key);
        self.col += 1;
    }

    // Update the cursor if necessary since rows have different length
    // This is necessary as different rows has different length
    fn _update_cursor(&mut self) {
        // TODO: account for the start of the row (a.k.a tabs)
        let curr_len = self.buffer[self.row].len();

        if self.virtual_col > curr_len - 1 {
            self.col = curr_len - 1;
        } else {
            self.col = self.virtual_col
        }
    }
    // TODO: print/render, allow a arbitrary front-end trait to be passed in 
}
