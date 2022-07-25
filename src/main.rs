use clap::Parser;
use std::{fs, path::Path, process};

/// Converts a binary file to its hex representation
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the input file
    input_file: String,
    /// Path of the converted file if set. Otherwise, will output to `stdout`.
    output_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input_file_path_str: String = args.input_file;
    let output_file_path_str: Option<String> = args.output_file;
    validate_arguments(&input_file_path_str, &output_file_path_str);

    let bytes = get_file_content(input_file_path_str);
    let hex_result = hex::encode(bytes);
    write_result(hex_result, output_file_path_str);
}

fn write_result(hex_result: String, output_file_path_str: Option<String>) {
    let file_path_string = output_file_path_str.unwrap();
    let file_path_str = file_path_string.as_str();
    match fs::write(&file_path_str, hex_result) {
        Ok(_) => {}
        Err(_) => {eprintln!("Could not write result in file {}", &file_path_str)}
    };
}

fn validate_arguments(input_file_path_str: &String, output_file_path_str: &Option<String>) {
    validate_input_file_path(input_file_path_str);
    validate_output_file_path(output_file_path_str);
}

fn validate_input_file_path(path_string: &String) {
    let path = Path::new(path_string.as_str());
    if !path.exists() {
        eprintln!("The given input file path [{}] doesn't exist", path_string);
        process::exit(exitcode::DATAERR);
    }
    if !path.is_file() {
        eprintln!("The given input file path [{}] is not a file", path_string);
        process::exit(exitcode::DATAERR);
    }
}

fn validate_output_file_path(path_str: &Option<String>) {
    if path_str.is_none() {
        return;
    }

    let path = Path::new(path_str.as_ref().unwrap().as_str());
    if path.exists() {
        eprintln!("The given output file path [{}] already exists", path_str.as_ref().unwrap());
        process::exit(exitcode::DATAERR);
    }
    match path.parent() {
        None => {
            eprintln!("The output file cannot be created at the given path [{}]", path_str.as_ref().unwrap());
            process::exit(exitcode::DATAERR);
        }
        Some(path) => {
            if !path.exists() | !path.is_dir() {
                eprintln!("The output given path [{}] is not an existing dir", path.to_str().unwrap());
                process::exit(exitcode::DATAERR);
            }
        }
    }
}

fn get_file_content(path: String) -> Vec<u8> {
    fs::read(&path).expect("Cannot read file")
}
