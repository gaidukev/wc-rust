use std::{env, fs::metadata, process};
use std::io::{self, Read};

fn count_bytes(filename: &String) -> u64 {
    metadata(filename).unwrap().len()
}

fn count_lines(filename: &String) -> usize {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .count()
}

fn count_words(filename: &String) -> usize {
    std::fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .count()
}

fn count_chars(filename: &String) -> usize {
    std::fs::read_to_string(filename)
        .unwrap()
        .chars()
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments_count = args.len();

    // check for argument length
    if arguments_count < 2 {
        process::exit(1);
    }

    let filename: &String = &args[1];

    if arguments_count == 2 {
        // calculate all the results and print them nicely

        let line_count: usize = count_lines(filename);
        let word_count: usize = count_words(filename);
        let byte_count: u64 = count_bytes(filename);

        println!("{:>10} {:>10} {:>10}", line_count, word_count, byte_count);

    } else {
        let command_flag = &args[2];
        let valid_flags = ["-c", "-l", "-w", "-m"];
        if !valid_flags.contains(&command_flag.as_str()) {
            process::exit(1);
        }

        if command_flag == &String::from("-c") {
            let byte_count: u64 = count_bytes(filename);
            println!("{}", byte_count);
        } else if command_flag == &String::from("-l") {
            let line_count: usize = count_lines(filename);
            println!("{}", line_count);
        } else if command_flag == &String::from("-w") {
            let word_count: usize = count_words(filename);
            println!("{}", word_count);
        } else if command_flag == &String::from("-m") {
            // checking character count using assuming a UTF8 encoding of the input file
            let char_count: usize = count_chars(filename);
            println!("{}", char_count);

        }
    }
}
