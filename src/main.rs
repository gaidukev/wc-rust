use std::{env, fs::metadata, process};
use std::io::{self, Read};

fn count_bytes(file_content: &String) -> usize {
    file_content
        .as_bytes()
        .len()  
}

fn count_lines(file_content: &String) -> usize {
    file_content
        .lines()
        .count()
}

fn count_words(file_content: &String) -> usize {
    file_content
        .split_whitespace()
        .count()
}

fn count_chars(file_content: &String) -> usize {
    file_content
        .chars()
        .count()
}   

fn read_data_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn read_data_from_file(filename: &String) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn no_flag(file_content: &String) {
    // calculate all the results and print them nicely
    let line_count: usize = count_lines(file_content);
    let word_count: usize = count_words(file_content);
    let byte_count: usize = count_bytes(file_content);

    println!("{:>10} {:>10} {:>10}", line_count, word_count, byte_count);    
}

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
    }
}

fn determine_file_and_flag(args: &Vec<String>, valid_flags: [&'static str; 4]) -> (String, String) {
    if valid_flags.contains(&args[1].as_str()) {
        (args[2].clone(), args[1].clone())
    } else {
        (args[1].clone(), args[2].clone())
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

        }
    }
}
