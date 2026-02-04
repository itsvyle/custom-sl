use std::io::Write;
use std::time::Instant;
use std::{io::stdout, thread, time::Duration};

use anyhow::Result as AnyhowResult;
use clap::{CommandFactory, FromArgMatches, Parser};
use crossterm::queue;
use crossterm::{
    cursor,
    event::{self, Event, KeyEvent},
    execute,
    terminal::{self},
};
use prepare_text::*;
use unicode_width::UnicodeWidthStr;

mod prepare_text;
mod safe_print;
use safe_print::safe_print;

#[derive(Parser, Debug)]
#[command(version, about = "A program to make custom text fly accross the screen", long_about = None)]
struct Args {
    /// Number of times to scroll the text
    #[arg(long, default_value_t = 1)]
    scroll_loops: u32,

    /// Allow exiting the program by pressing any key
    #[arg(long, short = 'e', default_value_t = false)]
    allow_exit: bool,

    #[arg(long, short = 'w', default_value_t = false)]
    faster_text: bool,

    /// Frames per second
    #[arg(long, short = 'f', default_value_t = 60)]
    fps: u32,

    /// Use the small alphabet
    #[arg(long, short = 'l', default_value_t = false)]
    little: bool,

    /// Text to display
    #[arg(default_value = "Text Here")]
    text: String,
}

struct TerminalDrop;
impl Drop for TerminalDrop {
    fn drop(&mut self) {
        let mut out = stdout();
        terminal::disable_raw_mode().ok();
        execute!(out, terminal::LeaveAlternateScreen, cursor::Show).ok();
    }
}

fn main() -> AnyhowResult<()> {
    // Parse arguments, ignoring errors to allow flags that would be passed to sl (so that this sl can replace the old one, without implementing all its flags)
    let mut args =
        Args::from_arg_matches_mut(&mut Args::command().ignore_errors(true).get_matches())?;
    if args.faster_text && args.fps == 60 {
        args.fps = 120;
    }

    let alphabet = if args.little {
        alphabet_gen::generated_alphabet_small()
    } else {
        alphabet_gen::generated_alphabet_large()
    };
    let centered = center_text(&args.text.replace("\\n", "\n"));
    let big_text = render_big_text(&centered, &alphabet);

    let text_width = big_text
        .iter()
        .map(|line| UnicodeWidthStr::width(line.as_str()))
        .max()
        .unwrap_or(0) as i16;
    let mut frame_time = Duration::from_millis(1000 / args.fps as u64);

    let mut out = stdout();
    terminal::enable_raw_mode()?;
    execute!(
        out,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;
    let _terminal_drop = TerminalDrop;

    let (mut width, mut height) = terminal::size()?;
    if big_text.len() > height as usize {
        drop(_terminal_drop);
        anyhow::bail!("Terminal height too small");
    }

    let mut x = width as i16;
    let mut loops_done = 0;

    while args.scroll_loops == 0 || loops_done < args.scroll_loops {
        let start = Instant::now();

        let (w, h) = terminal::size()?;
        width = w;
        height = h;

        queue!(out, terminal::Clear(terminal::ClearType::All))?;

        let y = height as i16 / 2 - big_text.len() as i16 / 2;

        for (dy, line) in big_text.iter().enumerate() {
            safe_print(&mut out, y + dy as i16, x, line, width, height)?;
        }

        out.flush()?;
        x -= 1;

        if x < -text_width {
            x = width as i16;
            loops_done += 1;
        }

        // Non-blocking input
        if event::poll(Duration::from_millis(0))?
            && let Event::Key(KeyEvent { .. }) = event::read()?
        {
            if args.allow_exit {
                break;
            }
            frame_time = frame_time.mul_f32(1.25);
        }

        let elapsed = start.elapsed();
        if elapsed < frame_time {
            thread::sleep(frame_time - elapsed);
        }
    }

    drop(_terminal_drop);
    Ok(())
}
