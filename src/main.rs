use std::io::Write;
use std::time::Instant;
use std::{io::stdout, thread, time::Duration};

use anyhow::Result as AnyhowResult;
use clap::Parser;
use crossterm::queue;
use crossterm::{
    cursor,
    event::{self, Event, KeyEvent},
    execute,
    terminal::{self},
};
use prepare_text::*;

mod prepare_text;
mod safe_print;
use safe_print::safe_print;
use unicode_width::UnicodeWidthStr;

#[derive(Parser, Debug)]
#[command(version, about = "A program to make custom text fly accross the screen", long_about = None)]
struct Args {
    /// Number of times to scroll the text
    #[arg(long, default_value_t = 1)]
    scroll_loops: u32,

    #[arg(long, short = 'e', default_value_t = false)]
    allow_exit: bool,

    #[arg(long, short = 'f', default_value_t = 60)]
    fps: u32,

    #[arg(long, short = 'l', default_value_t = false)]
    little: bool,

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
    let args = Args::parse();

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
    let frame_time = Duration::from_millis(1000 / args.fps as u64);

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
            && args.allow_exit
        {
            break;
        }

        let elapsed = start.elapsed();
        if elapsed < frame_time {
            thread::sleep(frame_time - elapsed);
        }
    }

    drop(_terminal_drop);
    Ok(())
}
