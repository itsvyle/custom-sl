use anyhow::{Context as _, Result as AnyhowResult};
use crossterm::{QueueableCommand, cursor::MoveTo, style::Print};
use std::io::Write;
use unicode_width::UnicodeWidthChar;

pub fn safe_print<W: Write>(
    out: &mut W,
    y: i16,
    x: i16,
    s: &str,
    width: u16,
    height: u16,
) -> AnyhowResult<()> {
    // If completely outside vertical bounds, do nothing
    if y < 0 || y >= height as i16 {
        return Ok(());
    }

    let mut x = x;
    let mut text = s;

    if x < 0 {
        let mut skip_width = -x;
        let mut char_index = 0;
        for c in text.chars() {
            let w = UnicodeWidthChar::width(c).unwrap_or(0) as i16;
            if skip_width < w {
                break; // stop skipping in the middle of this char
            }
            skip_width -= w;
            char_index += c.len_utf8();
        }
        text = &text[char_index..];
        x = 0;
    }

    let max_width = width as i16 - x;
    if max_width <= 0 {
        return Ok(());
    }

    let mut clipped_text = String::new();
    let mut current_width = 0;
    for c in text.chars() {
        let w = UnicodeWidthChar::width(c).unwrap_or(0) as i16;
        if current_width + w > max_width {
            break;
        }
        clipped_text.push(c);
        current_width += w;
    }

    if clipped_text.is_empty() {
        return Ok(());
    }

    // Debug output (optional)
    // println!("safe_print at ({}, {}) text: {:?}", x, y, clipped_text);

    // Queue the crossterm commands
    out.queue(MoveTo(x as u16, y as u16))
        .context("Failed to move cursor")?;
    out.queue(Print(&clipped_text))
        .context("Failed to print text")?;
    out.flush().context("Failed to flush output")?;

    Ok(())
}
