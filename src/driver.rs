use crate::compiler;
use core::result::Result;
use std::{fs, process::Command};

const PREPROCESSED_FILE_NAME: &str = "preprocessed_file.c";
const ASSEMBLY_OUTPUT_FILE_NAME: &str = "assembly_file.s";

pub fn run(file_path: &str, running_mode: Option<&str>, only_assembly: bool) -> Result<(), String> {
    preprocessor(file_path, PREPROCESSED_FILE_NAME)?;

    let (output_name, source_code) = read_source_code(file_path)?;
    compiler::compile(ASSEMBLY_OUTPUT_FILE_NAME, &source_code, running_mode);

    if !only_assembly {
        assembly_linker(ASSEMBLY_OUTPUT_FILE_NAME, output_name)?;
    }
    Ok(())
}

pub fn read_source_code(file_path: &str) -> Result<(String, String), String> {
    let source_code =
        fs::read_to_string(PREPROCESSED_FILE_NAME).expect("Failed to read the preprocessed file");
    fs::remove_file(PREPROCESSED_FILE_NAME).expect("Failed to remove the preprocessed file");

    let mut output_vector: Vec<&str> = file_path.split("/").collect();
    output_vector = output_vector
        .last()
        .expect("Error While reading filename")
        .split('.')
        .collect();
    let output_name = output_vector
        .first()
        .expect("Error While reading filename")
        .to_string();

    Ok((output_name, source_code))
}

pub fn assembly_linker(
    input_assembly_source: &str,
    output_executable: String,
) -> Result<(), String> {
    println!("linking the compiled assembly to a executable with gcc ...");

    let output = Command::new("gcc")
        .arg(input_assembly_source)
        .arg("-o")
        .arg(output_executable)
        .spawn();

    let mut child = match output {
        Ok(child) => child,
        Err(error) => return Err(format!("Compiler Driver Error: {:#?}", error)),
    };
    let exit_status = match child.wait() {
        Ok(exit_status) => exit_status,
        Err(error) => return Err(format!("Failed to wait for assembly process: {:#?}", error)),
    };
    if exit_status.success() == true {
        Ok(())
    } else {
        Err(format!("Failed to run the assembly linker"))
    }
}

pub fn preprocessor(file_path: &str, output_file: &str) -> Result<(), String> {
    println!("running the preprocessor with gcc ...");

    let output = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(file_path)
        .arg("-o")
        .arg(output_file)
        .spawn();

    let mut child = match output {
        Ok(child) => child,
        Err(error) => return Err(format!("Compiler Driver Error: {:#?}", error)),
    };
    let exit_status = match child.wait() {
        Ok(exit_status) => exit_status,
        Err(error) => return Err(format!("Failed to wait for process: {:#?}", error)),
    };
    if exit_status.success() == true {
        Ok(())
    } else {
        Err(format!("Failed to run the preprocessor"))
    }
}
