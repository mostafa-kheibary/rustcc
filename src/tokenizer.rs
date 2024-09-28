use core::panic;
use regex::{Match, Regex};
use std::collections::HashMap;

pub fn tokenizer_lexer(source_code: &String) {
    let tokens: HashMap<&str, &str> = HashMap::from([
        ("Identifier", r"[a-zA-Z_]\w*\b"),
        ("Constant", r"[0-9]+\b"),
        ("int keyword", r"int\b"),
        ("void keyword", r"void\b"),
        ("return keyword", r"return\b"),
        ("Open parenthesis", r"\("),
        ("Close parenthesis", r"\)"),
        ("Open brace", r"\{"),
        ("Close brace", r"\}"),
        ("Semicolon", r";"),
    ]);
    let mut valid_tokens: Vec<(&str, &str)> = Vec::new();

    let input = source_code.as_str();
    let input_lines = input.split("\n");

    for (index, original_line) in input_lines.enumerate() {
        let mut line = original_line;
        while !line.is_empty() {
            line = line.trim_start();
            let mut max_size: usize = 0;
            let mut token: Option<(Match<'_>, &str)> = None;

            for (key, value) in tokens.clone().into_iter() {
                let token_regex = Regex::new(value).unwrap();
                if let Some(matched) = token_regex.find_at(line, 0) {
                    // Check match for any tokens
                    if matched.len() > max_size && matched.start() == 0 {
                        max_size = matched.len();
                        token = Some((matched, key));
                    }
                    // Check keyword
                    if let Some((old_matched, _old_key)) = token {
                        if matched.start() == 0
                            && key.contains("keyword") == true
                            && old_matched.len() == matched.len()
                        {
                            max_size = matched.len();
                            token = Some((matched, key));
                        }
                    }
                };
            }
            if let Some((matched, key)) = token {
                valid_tokens.push((key, matched.as_str()));
                line = &line[matched.len()..];
            } else if !line.is_empty() {
                tokenizer_error_handler(index + 1, original_line);
            }
        }
    }
    println!("{:#?}", valid_tokens)
}

fn tokenizer_error_handler(line: usize, source_line: &str) {
    panic!(
        "
    Compiler Error: Invalid token

    {} |    {}
    ",
        line, source_line
    );
}
