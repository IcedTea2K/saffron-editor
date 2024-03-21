use std::io::{self, Write};

pub enum Event {
    ESCAPE,
    CTRL,
    SHIFT,
    OPTION,
    CMD,
    ASCII(char),
}

pub enum Mode {
    EDIT,
    VISUAL,
    COMMAND,
}

#[derive(Copy, Clone)]
pub enum State {
    START,
    IN_SESSION,
    EXIT
}

pub struct Editor {
    state: State,
    buffer: Option<Event>, // TODO: should have internal buffer for something
                    // Right now, it's just storing the last event
}

impl Editor {    
    pub fn new() -> Self {
        let state = State::IN_SESSION;
        Editor {
            state,
            buffer: None
        }
    }

    pub fn get_state(&self) -> State {
        self.state 
    }

    pub fn process_event(&mut self, event: Event) { // TODO: -> Result<...> instead
        match event {
            Event::ASCII(c) => {
                if 'q' == c {
                    self.state = State::EXIT;
                }
                print!("{}", c); // TODO: remove these calls. Should not have no frontend
                io::stdout().flush();
            }
            _ => {
                // TODO: do something about control characters
            }
        }
        self.buffer = Some(event);
    }

    // TODO: print/render, allow a arbitrary front-end trait to be passed in 
}
