#[allow(dead_code)]
pub static ESCAPE_CODE: &str = "\x1b[";

pub static START_LINE: &str  = "\x1b[1G";

pub static THIN_CURSOR: &str  = "\x1b[5 q";
pub static THICK_CURSOR: &str = "\x1b[2 q";

pub static CLEAR_SCREEN: &str = "\x1b[2J";
pub static ALT_SCREEN: &str   = "\x1b[?1049h";
pub static EXIT_SCREEN: &str  = "\x1b[?1049l";

pub static JUMP_TO_ORG: &str = "\x1b[H";
/// Jump to a specific column
pub fn JUMP_TO_HORIZONTAL(num: usize) -> String {
    format!("{ESCAPE_CODE}{num}G")
}
/// Move the cursor to a coordinates on screen (row, col)
pub fn JUMP_TO(coord: (usize, usize)) -> String {
    let (row, col) = coord;
    format!("{ESCAPE_CODE}{row};{col}H")
}
