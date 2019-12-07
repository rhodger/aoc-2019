use std::env;
use clap::*;

mod puzzle01;

fn main() {
    let matches = App::new("aoc-2019")
                          .version("0.2")
                          .author("SuedeGently")
                          .about("Advent of Code 2019")
                          .arg(Arg::with_name("PUZZLE")
                               .long("puzzle")
                               .value_name("puzzle")
                               .help("sets the desired puzzle")
                               .index(1)
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("FILE")
                                .long("path")
                                .value_name("Path to file")
                                .help("Sets the input file to use"))
                          .get_matches();
    
    if matches.value_of("PUZZLE").unwrap() == "01"{
        let path: &str = matches.value_of("FILE").unwrap_or("./content/input011.txt");
        print!("Fuel needed: {}\n\n", puzzle01::calculate_fuel(path));
    }else{
        print!("Didn't work :(\n");
    }
}