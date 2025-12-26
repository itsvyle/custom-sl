use crossterm::{QueueableCommand, cursor::MoveTo, style::Print};
use std::io::Write;

pub fn safe_print<W: Write>(out: &mut W, y: i16, x: i16, s: &str, width: u16, height: u16) {
    if y < 0 || y >= height as i16 || x >= width as i16 {
        return;
    }

    let mut x = x;
    let mut text = s;

    if x < 0 {
        let cut = (-x) as usize;
        if cut >= s.len() {
            return;
        }
        text = &s[cut..];
        x = 0;
    }

    let max_len = (width as i16 - x) as usize;
    let text = &text[..text.len().min(max_len)];

    out.queue(MoveTo(x as u16, y as u16)).ok();
    out.queue(Print(text)).ok();
}
