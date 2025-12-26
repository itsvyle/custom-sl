use std::io::Write;
use std::time::Instant;
use std::{io::stdout, thread, time::Duration};

use anyhow::{Context as _, Result as AnyhowResult};
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

#[derive(Parser, Debug)]
#[command(version, about = "A program to make custom text fly accross the screen", long_about = None)]
struct Args {
    /// Number of times to scroll the text
    #[arg(long, default_value_t = 1)]
    scroll_loops: u32,

    #[arg(long, short = 'e', default_value_t = false)]
    allow_exit: bool,

    #[arg(long, short = 'f', default_value_t = 30)]
    fps: u32,

    #[arg(long, short = 'l', default_value_t = false)]
    little: bool,

    #[arg(default_value = "Text Here")]
    text: String,
}

fn main() -> AnyhowResult<()> {
    let args = Args::parse();

    let alphabet = alphabet_gen::generated_alphabet();
    let centered = center_text(&args.text);
    let big_text = render_big_text(&centered, &alphabet);
    let text_width = big_text.first().map(|l| l.len()).unwrap_or(0) as i16;
    let frame_time = Duration::from_millis(1000 / args.fps as u64);

    terminal::enable_raw_mode()?;
    let mut out = stdout();

    execute!(
        out,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;

    let (mut width, mut height) = crossterm::terminal::size()?;
    if big_text.len() > height as usize {
        terminal::disable_raw_mode()?;
        execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
        anyhow::bail!("Terminal height too small");
    }

    let mut x = width as i16;
    let mut loops_done = 0;

    while args.scroll_loops == 0 || loops_done < args.scroll_loops {
        let start = Instant::now();

        let (w, h) = crossterm::terminal::size()?;
        width = w;
        height = h;

        queue!(out, terminal::Clear(terminal::ClearType::All))?;

        let y_base = height as i16 / 2 - big_text.len() as i16 / 2;

        for (i, line) in big_text.iter().enumerate() {
            safe_print(&mut out, y_base + i as i16, x, line, width, height);
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

    terminal::disable_raw_mode()?;
    execute!(out, terminal::LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}
