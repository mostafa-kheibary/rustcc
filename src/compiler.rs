use crate::tokenizer;

pub fn compile(_output_name: &str, source_code: &String, running_mode: Option<&str>) {
    println!("compiling the source code ...");
    // Running compiler partial with options
    if let Some(matched) = running_mode {
        match matched {
            "lex" => tokenizer::tokenizer_lexer(source_code),
            _ => (),
        }
    } else {
        // Run the hole compilation
        tokenizer::tokenizer_lexer(source_code);
    }
}
