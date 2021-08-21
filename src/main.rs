use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead, stdin};

fn process_lines<T: BufRead + Sized>(reader: T, pattern: &str) {
    let reg = Regex::new(pattern).unwrap();
    let mut nothing_found = true;

    for (index, line) in reader.lines().enumerate() {
        match reg.find(&line.unwrap()) {
            Some(_) => {
                println!("Found pattern '{}' in line n {}!", pattern, index);
                nothing_found = false;
            },
            None => {},
        }
    }

    if nothing_found == true {
        println!("Nothing found !");
    }
}

fn main() {
    let pattern_arg_name = "pattern";
    let file_path_arg_name = "file_path";
    let fallback_path = "-";

    let arguments = App::new("grep-lite")
        .about("Search in the quote")
        .arg(
            Arg::with_name(pattern_arg_name)
                .help("What do you search for.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name(file_path_arg_name)
                .help("Which file do you want to process ?")
                .takes_value(true)
                .required(false)
        )
        .get_matches();

    let pattern = arguments.value_of(pattern_arg_name).unwrap();
    let path = arguments.value_of(file_path_arg_name).unwrap_or(fallback_path);

    if path == fallback_path {
        println!("Type your text here:");
        let stdin = stdin();
        let reader = stdin.lock();
        process_lines(reader, pattern);
    } else {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        process_lines(reader, pattern);
    }
}
