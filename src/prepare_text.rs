use font8x8::{BASIC_FONTS, BOX_FONTS, LATIN_FONTS, MISC_FONTS, UnicodeFonts};
use std::collections::HashMap;

pub mod alphabet_gen {
    use super::Alphabet;
    include!(concat!(env!("OUT_DIR"), "/alphabet_generated.rs"));
}

pub struct Alphabet {
    pub letters: HashMap<char, Vec<&'static str>>,
    pub letter_height: usize,
    pub line_spacing: usize,
}
impl Alphabet {
    pub fn get(&self, c: char) -> &Vec<&'static str> {
        self.letters
            .get(&c)
            .or_else(|| self.letters.get(&' '))
            .expect("Alphabet must contain space character")
    }
}

pub fn center_text(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let centered: Vec<String> = lines
        .into_iter()
        .map(|line| {
            let padding = max_len.saturating_sub(line.len());
            let left = padding / 2;
            let right = padding - left;
            format!("{}{}{}", " ".repeat(left), line, " ".repeat(right))
        })
        .collect();

    centered.join("\n")
}

macro_rules! try_font {
    ($fonts:expr, $c:expr) => {
        if let Some(glyph) = $fonts.get($c) {
            return glyph;
        }
    };
}

#[inline]
pub fn get_8x8_glyph(c: char) -> [u8; 8] {
    try_font!(BASIC_FONTS, c);
    try_font!(LATIN_FONTS, c);
    try_font!(MISC_FONTS, c);
    try_font!(BOX_FONTS, c);
    [0; 8]
}

pub fn render_big_text(text: &str, alphabet: &Alphabet) -> Vec<String> {
    let mut big_lines = Vec::new();
    let mut lines = vec![String::new(); alphabet.letter_height];

    for ch in text.chars() {
        if ch == '\n' {
            big_lines.append(&mut lines);

            for _ in 0..alphabet.line_spacing {
                big_lines.push(String::new());
            }

            lines = vec![String::new(); alphabet.letter_height];
            continue;
        }

        let glyph = get_8x8_glyph(ch);

        for (i, x) in glyph.iter().enumerate() {
            for bit in 0..8 {
                match *x & 1 << bit {
                    0 => lines[i].push(' '),
                    _ => lines[i].push('█'),
                }
            }
        }
    }

    big_lines.extend(lines);
    big_lines
}
