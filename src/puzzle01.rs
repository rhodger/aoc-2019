//! Solution to the first puzzle day 2 variant.
//! 
//! Unfortunately, the functionality for day 1 of this puzzle was not retained
//! when the solution for day 2 was designed. This problem does not/ will not
//! occur in future puzzles.

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

/// Returns the fuel required for an object.
/// 
/// Returns the fuel required minus the fuel neccessary for that fuel (and so
/// on). As this was only necessary for day 2, that is placed in a separate
/// function to maintain modularity.
fn process(x: u64) -> u64{
    let mut temp = x / 3;
    if temp >= 2{
        temp = temp - 2;
    }else{
        temp = 0;
    }
    return temp;
}

/// Returns the fuel required for the fuel of an object.
///
/// This is an iterative solution to the recurisve 'fuel for weight of fuel'
/// problem presented in day 2 of puzzle 1. This returns the additional fuel
/// required for a 'module' of a given mass once the weight of the fuel itself
/// is taken into account.
fn fuel_for(x: u64) -> u64{
    let mut total: u64 = 0;
    let mut next = x;
    while process(next) > 0{
        total = total + process(next);
        next = process(next);
    }
    return total;
}

/// This function returns the amount of fuel required for a 'spacecraft' made up of many 'modules'.
/// 
/// The string passed to this function must be a text file containing a list of
/// newline-separated values representing the weights of the various components
/// of a 'spacecraft' It will calculate the fuel required for each of these
/// 'modules' (including the fuel required for the fuel itself) summed as a u64.
pub fn calculate_fuel(path: &str) -> u64{
    let mut input = File::open(path).expect("Failed to open input file");
    let mut input_buffer = String::new();
    input
      .read_to_string(&mut input_buffer)
      .expect("Failed to read from input file");
    let mut total: u64 = 0;
    for i in Regex::new(r"\s+").unwrap().split(&input_buffer[..]){
        let unprocessed: u64 = i
          .to_string()
          .parse()
          .expect("Unable to parse int from string");

        // Calculate the fuel required for the module
        let processed: u64 = process(unprocessed);

        // Calculate the fuel required for the module's fuel
        let additional: u64 = fuel_for(processed);

        // Add the sum of these to the running total
        total = total + processed + additional;
    }
    return total;
}