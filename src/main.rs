use clap::Parser;
use std::{fs, path::Path, process};

/// Converts a binary file to its hex representation
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the input file
    input_file: String,
}

fn main() {
    let args = Args::parse();

    let file_path_str = args.input_file.as_str();
    validate_arguments(file_path_str);

    let bytes = get_file_content(file_path_str);
    let result = hex::encode(bytes);
    println!("{}", result);
}

fn validate_arguments(file_path_str: &str) {
    let file_path = Path::new(file_path_str);
    if !file_path.exists() {
        eprintln!("The given file path [{}] doesn't exist", file_path_str);
        process::exit(exitcode::DATAERR);
    }
    if !file_path.is_file() {
        eprintln!("The given file path [{}] is not a file", file_path_str);
        process::exit(exitcode::DATAERR);
    }
}

fn get_file_content(path: &str) -> Vec<u8> {
    fs::read(&path).expect("Cannot read file")
}
