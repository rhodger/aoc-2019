use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn open_file(path: &str) -> String{
    let mut input = File::open(path).expect("Failed to open input file");
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).expect("Failed to read from input file");
    return input_buffer;
}

fn display(data: Vec<u64>){
    for i in data.iter(){
        print!("{}\n", i);
    }
}

fn execute(mut data: Vec<u64>) -> Vec<u64>{
    let mut i: u16 = 0;
    while i < data.len() as u16{
        if data[i as usize] == 1{
            let x: u64 = data[(i + 1) as usize];
            let y: u64 = data[(i + 2) as usize];
            let z: u64 = data[(i + 3) as usize];
            print!("{}({}) = {} + {}\n", z, data[z as usize], data[x as usize], data[y as usize]);
            data[z as usize] = data[x as usize] + data[y as usize];
        }else if data[i as usize] == 2{
            let x: u64 = data[(i + 1) as usize];
            let y: u64 = data[(i + 2) as usize];
            let z: u64 = data[(i + 3) as usize];
            print!("{}({}) = {} * {}\n", z, data[z as usize], data[x as usize], data[y as usize]);
            data[z as usize] = data[x as usize] * data[y as usize];
        }else if data[i as usize] == 99{
            i = data.len() as u16;
        }
        i = i + 4;
    }
    return data;
}

pub fn puzzle021(path: &str) -> u64{
    let data: &str = &open_file(path)[..];
    let mut vec: Vec<u64> = Vec::new();
    for i in Regex::new(r",").unwrap().split(data){
        vec.push(i.parse::<u64>().expect("Failed to parse string"));
    }
    vec = execute(vec);
    return vec[0 as usize];
}

pub fn puzzle022(path: &str, target: u64) -> u64{
    let data: &str = &open_file(path)[..];
    let mut vec: Vec<u64> = Vec::new();
    for i in Regex::new(r",").unwrap().split(data){
        vec.push(i.parse::<u64>().expect("Failed to parse string"));
    }
    let mut n: u64 = 0;
    let mut v: u64 = 0;
    for i in 0..100{
        for j in 0..100{
            vec[1 as usize] = i;
            vec[2 as usize] = j;
            vec = execute(vec);
            if vec[0 as usize] == target{
                n = i;
                v = j;
            }
        }
    }
    return (vec[0 as usize] * 100) + vec[1 as usize];
}