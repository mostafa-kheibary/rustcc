use std::env::args;

mod compiler;
mod driver;

fn main() {
    let argc: Vec<String> = args().collect();
    if argc.len() < 2 {
        panic!("Wrong number of argument, rustcc ./source_code.c ");
    }
    let mut running_mode: Option<&str> = None;
    let mut only_assembly: bool = false;
    let mut help_mode: bool = false;

    for arg in &argc[1..] {
        match arg.as_str() {
            "--help" => help_mode = true,
            "--lex" => running_mode = Some("lex"),
            "--parse" => running_mode = Some("parse"),
            "--codegen" => running_mode = Some("codegen"),
            "-S" => only_assembly = true,
            _ => (),
        }
    }
    if help_mode {
        println!("
        USAGE:
            rustcc ./source.c [OPTIONS]

        OPTIONS:
            --help      Show the list of commands
            --lex       Directs it to run the lexer, but stop before parsing
            --parse     Directs it to run the lexer and parser, but stop before assembly generation
            --codegen   Directs it to perform lexing, parsing, and assembly generation, but stop before code emission
            -S          directs your compiler to emit an assembly file, but not assemble or link it
        ");
        return;
    }

    let file_path: &str = argc[1].as_str();

    let output = driver::run(file_path, running_mode, only_assembly);
    match output {
        Ok(()) => println!("File Compiled Successfully"),
        Err(e) => println!("{}", e),
    }
}
