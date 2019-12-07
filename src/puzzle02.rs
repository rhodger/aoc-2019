use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn openFile(path: &str) -> String{
    let mut input = File::open(path).expect("Failed to open input file");
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).expect("Failed to read from input file");
    return input_buffer;
}

fn execute(data: Vec<u64>) -> Vec<u64>{
    for i in data.iter(){
        print!("{}", i);
    }
    let v: Vec<u64> = Vec::new();
    return v;
}

pub fn puzzle021(path: &str) -> u64{
    let data: &str = &openFile(path)[..];
    let mut vec: Vec<u64> = Vec::new();
    for i in Regex::new(r",").unwrap().split(data){
        vec.push(i.parse::<u64>().expect("Failed to parse string"));
    }
    for i in vec.iter(){
        print!("Contains {}\n", i);
    }
    return 0;
}