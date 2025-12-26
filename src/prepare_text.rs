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

pub fn render_big_text(text: &str, alphabet: &Alphabet) -> Vec<String> {
    let mut big_lines = Vec::new();
    let mut lines = vec![String::new(); alphabet.letter_height];

    for ch in text.to_uppercase().chars() {
        if ch == '\n' {
            big_lines.append(&mut lines);

            for _ in 0..alphabet.line_spacing {
                big_lines.push(String::new());
            }

            lines = vec![String::new(); alphabet.letter_height];
            continue;
        }

        let glyph = alphabet.get(ch);

        for i in 0..alphabet.letter_height {
            lines[i].push_str(glyph[i]);
            lines[i].push_str("  ");
        }
    }

    big_lines.extend(lines);
    big_lines
}
