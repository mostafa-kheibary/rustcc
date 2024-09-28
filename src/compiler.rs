use core::panic;
use regex::{Match, Regex};
use std::collections::HashMap;

pub fn compile(_output_name: &str, source_code: &String, running_mode: Option<&str>) {
    println!("compiling the source code ...");

    // Running compiler partial with options
    if let Some(matched) = running_mode {
        match matched {
            "lex" => tokenizer_lexer(source_code),
            _ => (),
        }
    } else {
        // Run the hole compilation
        tokenizer_lexer(source_code);
    }
}

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

    let mut input = source_code.as_str();

    while !input.is_empty() {
        input = input.trim_start();
        let mut max_size: usize = 0;
        let mut token: Option<(Match<'_>, &str)> = None;

        for (key, value) in tokens.clone().into_iter() {
            let token_regex = Regex::new(value).unwrap();
            if let Some(matched) = token_regex.find_at(input, 0) {
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
            input = &input[matched.len()..];
        } else if !input.is_empty() {
            panic!("Compiler Error: Invalid token");
        }
    }
    println!("{:#?}", valid_tokens)
}
