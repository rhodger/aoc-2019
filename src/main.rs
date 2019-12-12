use std::env;
use clap::*;

mod puzzle01;
mod puzzle02;

/// Handles command-line arguments
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
                                .short("p")
                                .value_name("Path to file")
                                .help("Sets the input file to use"))
                          .get_matches();
    
    if matches.value_of("PUZZLE").unwrap() == "01"{
        let path: &str = matches
          .value_of("FILE")
          .unwrap_or("./content/input011.txt");
        print!("Fuel needed: {}\n\n", puzzle01::calculate_fuel(path));
    }else if matches.value_of("PUZZLE").unwrap() == "021"{
        let path: &str = matches
          .value_of("FILE")
          .unwrap_or("./content/input021.txt");
        print!("{}\n", puzzle02::puzzle021(path));
    }else if matches.value_of("PUZZLE").unwrap() == "022"{
        let path: &str = matches
          .value_of("FILE")
          .unwrap_or("./content/input021.txt");
        print!("{}\n", puzzle02::puzzle022(path, 19690720));
    }else{
        print!("Didn't work :(\n");
    }
}
