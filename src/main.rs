use saffron_editor::terminal;
use std::process;

fn main() { 
    match terminal::start_editor() {
        Ok(_v) => (),
        Err(e) => {
            eprintln!("Unable to start the editor!!!");
            eprintln!("Error: {}", e);
            process::exit(1)
        }
    }
}
