use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn process(x: u64) -> u64{
    (x / 3) - 2
}

pub fn calculateFuel(path: &str) -> u64{
    let mut input = File::open(path).expect("Failed to open input file");
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).expect("Failed to read from input file");
    let mut total: u64 = 0;
    for i in Regex::new(r"\s+").unwrap().split(&input_buffer[..]){
        let unprocessed: u64 = i.to_string().parse().expect("Unable to parse int from string");
        let processed: u64 = process(unprocessed);
        total = total + processed;
        //print!("{} -> {} : {}\n", unprocessed, processed, total);
    }
    //print!("Final result: {}\n\n", total);
    return total;
}