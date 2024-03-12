use crate::editor;
use std::io::{self, Write};

pub fn start_editor() {
    editor::start_editor();
    let _ = io::stdout().write_all(b"Starting the editor directly");
}
