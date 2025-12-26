use serde::Deserialize;
use std::{env, fs, path::PathBuf};

#[derive(Deserialize)]
struct AlphabetJson {
    TEXT_HEIGHT: usize,
    LINES_SEP_COUNT: usize,
    ALPHABET: std::collections::BTreeMap<String, Vec<String>>,
}

fn do_alphabet_file(file: &str) {
    println!("cargo:rerun-if-changed={}", file);

    let json = fs::read_to_string(file).unwrap();
    let parsed: AlphabetJson = serde_json::from_str(&json).unwrap();

    let mut out = String::new();

    out.push_str("use std::collections::HashMap;\n\n");
    out.push_str("pub fn generated_alphabet() -> Alphabet {\n");
    out.push_str("    let mut letters: HashMap<char, Vec<&'static str>> = HashMap::new();\n\n");

    for (key, glyph) in parsed.ALPHABET {
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
        parsed.TEXT_HEIGHT, parsed.LINES_SEP_COUNT
    ));
    out.push_str("}\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("alphabet_generated.rs"), out).unwrap();
}

fn main() {
    do_alphabet_file("src/alphabet-large.json");
    // do_alphabet_file("src/alphabet-small.json");
}
