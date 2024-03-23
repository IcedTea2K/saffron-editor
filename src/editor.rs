use std::io::{self, Write};

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
    buffer: Option<Key>, // TODO: should have internal buffer for something
                    // Right now, it's just storing the last event
    action: Option<Action>,
    row: u32,
    col: u32,
}

impl Editor {    
    pub fn new() -> Self {
        let state = State::IN_SESSION;
        Editor {
            state,
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

    pub fn process_event(&mut self, event: Key) { // TODO: -> Result<...> instead
        match event {
            Key::ASCII(c) => {
                if 'q' == c {
                    self.state = State::EXIT;
                }

                self.action = Some(Action::APPEND(c));
                self.col += 1;
            }
            Key::DEL => {
                self.action = Some(Action::DELETE);
            }
            Key::ENTER => {
                self.action = Some(Action::NEWLINE);
                self.row += 1;
            }
            _ => {
                // TODO: do something about control characters
            }
        }
        self.buffer = Some(event);
    }

    // TODO: print/render, allow a arbitrary front-end trait to be passed in 
}
