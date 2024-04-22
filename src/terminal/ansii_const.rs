#[allow(dead_code)]
pub static ESCAPE_CODE: &str = "\x1b[";

pub static START_LINE: &str  = "\x1b[1G";
pub static THIN_CURSOR: &str  = "\x1b[5 q";
pub static THICK_CURSOR: &str = "\x1b[2 q";

pub fn JUMP_TO_HORIZONTAL(num: usize) -> String {
    format!("{ESCAPE_CODE}{num}G")
    // format!("{}{}{}", ESCAPE_CODE, num, "G").to_owned()
}
