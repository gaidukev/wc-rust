use core::panic;
use std::env;
use std::io::{self, Read};

/// Counts the bytes in a string
/// 
/// # Arguments
/// * `file_content` - Reference to the string to count bytes in
/// # Returns
/// * `usize` - Number of bytes in the string
fn count_bytes(file_content: &String) -> usize {
    file_content
        .as_bytes()
        .len()  
}

/// Counts the lines in a string
/// 
/// # Arguments
/// * `file_content` - Reference to the string to count lines in
/// # Returns
/// * `usize` - Number of lines in the string
fn count_lines(file_content: &String) -> usize {
    file_content
        .lines()
        .count()
}

/// Counts the words in a string.
/// Determines words by splitting on whitespace.
/// 
/// # Arguments
/// * `file_content` - Reference to the string to count words in
/// # Returns
/// * `usize` - Number of words in the string
fn count_words(file_content: &String) -> usize {
    file_content
        .split_whitespace()
        .count()
}

/// Counts the characters in a string.
/// Returns the count of UTF-8 characters. 
/// 
/// # Arguments
/// * `file_content` - Reference to the string to count characters in
/// # Returns
/// * `usize` - Number of characters in the string
/// # Panics
/// * If the string is not valid UTF-8
fn count_chars(file_content: &String) -> usize {
    file_content
        .chars()
        .count()
}   

/// Reads data from standard input into a string
/// 
/// # Returns
/// * `String` - The data read from standard input
fn read_data_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

/// Reads data from a file into a string
/// 
/// # Arguments
/// * `filename` - Reference to the name of the file to read from
/// # Returns
/// * `String` - The data read from the file
fn read_data_from_file(filename: &String) -> String {
    std::fs::read_to_string(filename).unwrap()
}

/// Handles the case where no flag is provided.
/// Prints the line, word, and byte counts, in that order.
/// 
/// # Arguments
/// * `file_content` - Reference to the string whose content must be analyzed
fn no_flag(file_content: &String) {
    // calculate all the results and print them nicely
    let line_count: usize = count_lines(file_content);
    let word_count: usize = count_words(file_content);
    let byte_count: usize = count_bytes(file_content);

    println!("{:>10} {:>10} {:>10}", line_count, word_count, byte_count);    
}

/// Handles the case where a flag is provided.
/// Processes the file content based on the flag.
/// # Arguments
/// * `file_content` - The content of the file to be analyzed
/// * `flag` - Reference to the flag indicating the type of count to perform (-c, -l, -w, -m)
/// # Panics
/// * If an invalid flag is provided
fn handle_file_contents(file_content: String, flag: &String) {
    if flag == &String::from("-c") {
        let byte_count: usize = count_bytes(&file_content);
        println!("{}", byte_count);
    } else if flag == &String::from("-l") {
        let line_count: usize = count_lines(&file_content);
        println!("{}", line_count);
    } else if flag == &String::from("-w") {
        let word_count: usize = count_words(&file_content);
        println!("{}", word_count);
    } else if flag == &String::from("-m") {
        // checking character count using assuming a UTF8 encoding of the input file
        let char_count: usize = count_chars(&file_content);
        println!("{}", char_count);
    } else {
        panic!("Invalid flag provided");
    }
}

/// Determines which argument is the filename and which is the flag.
/// # Arguments
/// * `args` - Reference to the vector of command line arguments
/// * `valid_flags` - Array of valid flags
/// # Returns
/// * `(String, String)` - Tuple containing the filename and the flag, in that order
/// # Panics
/// * If neither argument is a valid flag
fn determine_file_and_flag(args: &Vec<String>, valid_flags: [&'static str; 4]) -> (String, String) {
    if valid_flags.contains(&args[1].as_str()) {
        (args[2].clone(), args[1].clone())
    } else if valid_flags.contains(&args[2].as_str()) {
        (args[1].clone(), args[2].clone())
    } else {
        panic!("Invalid flag provided");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let valid_flags = ["-c", "-l", "-w", "-m"];
    let arguments_count = args.len();

    match arguments_count {
        1 => {
            let input_data = read_data_from_stdin();
            no_flag(&input_data);
        }
        2 => {
            if !valid_flags.contains(&args[1].as_str()) {
                let input_data: String = read_data_from_file(&args[1]);
                no_flag(&input_data);
            } else {
                let input_data: String = read_data_from_stdin();
                let flag = &args[1];
                handle_file_contents(input_data, flag);
            }
        }
        3 => {
            let (filename, flag) = determine_file_and_flag(&args, valid_flags); 
            let file_content = read_data_from_file(&filename);
            handle_file_contents(file_content, &flag);
        }
        _ => {
            panic!("Invalid number of arguments provided");
        }
    }
}
