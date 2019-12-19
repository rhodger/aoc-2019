//! Solution to puzzle 02
//!
//! Provides two public functions, `puzzle021()` and `puzzle022()` for providing
//! the separate solutions for the first and second versions of the puzzle for
//! day 2.

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

/// Opens a file at a given path and returns the contents.
///
/// Opens the file at a given path and returns its contents as a String.
///
/// # Examples
/// 
/// ```
/// let data: &str = open_file(str);
/// 
/// //Intuitively convert from String to &str
/// print!("File contents:\n{}", data);
/// ```
fn open_file(path: &str) -> String{
    let mut input = File::open(path).expect("Failed to open input file");
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)
      .expect("Failed to read from input file");
    return input_buffer;
}

/// Prints a vector containing u64s.
///
/// Takes a vector of u64s and prints its contents seperated by newlines. Mostly
/// for debugging purposes.
#[allow(dead_code)]
fn display(data: Vec<u64>){
    for i in data.iter(){
        print!("{}\n", i);
    }
}

/// Computes the intcode passed to it as a vector and returns the computed
/// vector.
/// 
/// Takes a vector containing a list of integers representative of the
/// integer-based computer described in puzzle 02 and executes it appropriately.
/// It returns the fully computed version of the vector.
///
/// # Examples
///
/// The intcode `10140000`, stored as the vector vec, could be processed as
/// such:
/// ```
/// // vec is a vector containing the above intcode
/// let processed: Vec<u64> = execute(vec);
/// print!("Contents of the processed intcode:\n");
/// display(processed);
/// ```
/// This would return the intcode `20141000`.
fn execute(mut data: Vec<u64>) -> Vec<u64>{
    let mut i: u16 = 0;
    while i < data.len() as u16{
        if data[i as usize] == 1{
            // Addition Opcode
            let x: u64 = data[(i + 1) as usize];
            let y: u64 = data[(i + 2) as usize];
            let z: u64 = data[(i + 3) as usize];
            if (x < data.len() as u64)
              && (y < data.len() as u64)
              && (z < data.len() as u64){
                // print!("{}({}) = {} + {}\n", z, data[z as usize],
                //   data[x as usize], data[y as usize]);
                data[z as usize] = data[x as usize] + data[y as usize];
            }
        }else if data[i as usize] == 2{
            // Multiplication Opcode
            let x: u64 = data[(i + 1) as usize];
            let y: u64 = data[(i + 2) as usize];
            let z: u64 = data[(i + 3) as usize];
            if (x < data.len() as u64)
              && (y < data.len() as u64)
              && (z < data.len() as u64){
                // print!("{}({}) = {} * {}\n", z, data[z as usize],
                //   data[x as usize], data[y as usize]);
                data[z as usize] = data[x as usize] * data[y as usize];
            }
        }else if data[i as usize] == 99{
            // End of program Opcode
            i = data.len() as u16;
        }
        i = i + 4;
    }
    return data;
}

/// Solves the first version of puzzle 02
///
/// Takes a path to a file containing the intcode input for this puzzle and
/// returns the value at position 0 of the processed version of the code.
pub fn puzzle021(path: &str) -> u64{
    let data: &str = &open_file(path)[..];
    let mut vec: Vec<u64> = Vec::new();
    for i in Regex::new(r",").unwrap().split(data){
        vec.push(i.parse::<u64>().expect("Failed to parse string"));
    }
    vec = execute(vec);
    return vec[0 as usize];
}

/// Solves the second version of puzzle 02
///
/// Takes a path to a file containing the intcode input for this puzzle and
/// executes this intcode using every possible value for positions 1 and 2,
/// checking each time if the value at position 0 of the resulting intcode match
/// a given target (also passed as an argument). It then returns the highest
/// possible pair of values that achieve this target, combined in the way
/// described in the puzzle description (`pos1 * 100 + pos2`).
pub fn puzzle022(path: &str, target: u64) -> u64{
    let mut n: u64 = 0;
    let mut v: u64 = 0;
    for i in 0..100{
        for j in 0..100{
            print!("{} + {} ", i, j);
            let data = &open_file(path)[..];
            let mut vec = Vec::new();
            for i in Regex::new(r",").unwrap().split(data){
                vec.push(i.parse::<u64>().expect("Failed to parse string"));
            }
            print!(".");
            vec[1 as usize] = i;
            vec[2 as usize] = j;
            vec = execute(vec);
            print!(".");
            if vec[0 as usize] as u64 == target{
                n = i;
                v = j;
                print!("!");
            }
            print!(".Done\n");
        }
    }
    print!("{} + {}\n", n, v);
    return n * 100 + v;
}
