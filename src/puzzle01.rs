use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn process(x: u64) -> u64{
    let mut temp = x / 3;
    if temp >= 2{
        temp = temp - 2;
    }else{
        temp = 0;
    }
    return temp;
}

fn fuel_for(x: u64) -> u64{
    let mut total: u64 = 0;
    let mut next = x;
    while process(next) > 0{
        total = total + process(next);
        next = process(next);
    }
    return total;
}

pub fn calculate_fuel(path: &str) -> u64{
    let mut input = File::open(path).expect("Failed to open input file");
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).expect("Failed to read from input file");
    let mut total: u64 = 0;
    for i in Regex::new(r"\s+").unwrap().split(&input_buffer[..]){
        let unprocessed: u64 = i.to_string().parse().expect("Unable to parse int from string");
        let processed: u64 = process(unprocessed);
        let additional: u64 = fuel_for(processed);
        total = total + processed + additional;
        //print!("{} -> {} + {} : {}\n", unprocessed, processed, additional, total);
    }
    //print!("Final result: {}\n\n", total);
    return total;
}