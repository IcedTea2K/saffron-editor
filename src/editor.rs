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
pub enum Action {
    APPEND(char),
    DELETE,
    NEWLINE,
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

    buffer: Option<Key>, // TODO: should have internal buffer for something
                    // Right now, it's just storing the last event
    action: Option<Action>,
    row: u32,
    col: u32,
}

impl Editor {    
    pub fn new() -> Self {
        Editor {
            state: State::START,
            mode: Mode::NORMAL,
            buffer: None,
            action: None,
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

    pub fn get_action(&mut self) -> Option<Action> {
        let returned_action = self.action;

        if self.action.is_some() {
            self.action = None;
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
                self.action = Some(Action::DELETE);
            }
            Key::ENTER => {
                self.action = Some(Action::NEWLINE);
                self.row += 1;
            }
            Key::ESCAPE => {
                self.mode = Mode::NORMAL;
            }
            _ => {
                // TODO: do something about control characters
            }
        }
        self.buffer = Some(key);
    }

    pub fn start(&mut self) {
        self.state = State::IN_SESSION;
        // should do some internal setup
    }

    pub fn exit(&mut self) {
        self.state = State::EXIT;
        // should do some internal clean up
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
            }
            'j' => {
                self.row += 1; // TODO: do some boundary checking
            }
            'k' => {
                if self.row != 0 {
                    self.row -= 1;
                }
            }
            'l' => {
                self.col += 1; // TODO: do some boundary checking
            }
            _ => {
                // do nothing for unsupport keys
            }
        }
    }

    fn _process_normal_ascii(&mut self, key: char) {
        self.action = Some(Action::APPEND(key));
        self.col += 1;
    }
    // TODO: print/render, allow a arbitrary front-end trait to be passed in 
}
