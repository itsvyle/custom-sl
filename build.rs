use serde::Deserialize;
use std::{env, fs, path::PathBuf};

#[derive(Deserialize)]
struct AlphabetJson {
    #[serde(rename = "TEXT_HEIGHT")]
    text_height: usize,
    #[serde(rename = "LINES_SEP_COUNT")]
    lines_sep_count: usize,
    #[serde(rename = "ALPHABET")]
    alphabet: std::collections::BTreeMap<String, Vec<String>>,
}

fn do_alphabet_file(file: &str, name: &str) -> String {
    println!("cargo:rerun-if-changed={}", file);

    let json = fs::read_to_string(file).unwrap();
    let parsed: AlphabetJson = serde_json::from_str(&json).unwrap();

    let mut out = String::new();

    out.push_str(&format!(
        "pub fn generated_alphabet_{}() -> Alphabet {{\n",
        name
    ));
    out.push_str("    let mut letters: HashMap<char, Vec<&'static str>> = HashMap::new();\n\n");

    for (key, glyph) in parsed.alphabet {
        let mut chars = key.chars();

        let ch = chars.next().expect("Empty alphabet key");
        if chars.next().is_some() {
            panic!("Alphabet key {:?} is not a single character", key);
        }

        let escaped = match ch {
            '\'' => "\\'".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            c if c.is_control() => {
                panic!("Control character {:?} not allowed in alphabet", c);
            }
            c => c.to_string(),
        };

        out.push_str(&format!("    letters.insert('{}', vec![\n", escaped));

        for line in glyph {
            out.push_str(&format!("        {:?},\n", line));
        }

        out.push_str("    ]);\n\n");
    }

    out.push_str(&format!(
        "    Alphabet {{ letters, letter_height: {}, line_spacing: {} }}\n",
        parsed.text_height, parsed.lines_sep_count
    ));
    out.push_str("}\n");

    out
}

fn main() {
    let mut out = String::new();
    out.push_str("use std::collections::HashMap;\n\n");
    out += &do_alphabet_file("src/alphabet-large.json", "large");
    out += &do_alphabet_file("src/alphabet-small.json", "small");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("alphabet_generated.rs"), out).unwrap();
}
