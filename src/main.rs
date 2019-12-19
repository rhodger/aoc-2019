use clap::*;

mod puzzle01;
mod puzzle02;
mod puzzle03;
mod puzzle04;

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
                          .arg(Arg::with_name("LOWBOUND")
                            .long("low")
                            .value_name("low_bound")
                            .help("Lower bound")
                            .short("l"))
                          .arg(Arg::with_name("HIGHBOUND")
                            .long("high")
                            .value_name("high_bound")
                            .help("Higher bound")
                            .short("hi"))
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
    }else if matches.value_of("PUZZLE").unwrap() == "031"{
        let path: &str = matches
          .value_of("FILE")
          .unwrap_or("./content/input031.txt");
        print!("{}\n", puzzle03::puzzle1(path));
    }else if matches.value_of("PUZZLE").unwrap() == "032"{
      let path: &str = matches
        .value_of("FILE")
        .unwrap_or("./content/input031.txt");
      print!("{}\n", puzzle03::puzzle2(path));
    }else if matches.value_of("PUZZLE").unwrap() == "041"{
      let lower_bound: u64 = matches.value_of("LOWBOUND").unwrap_or("111111").parse::<u64>().unwrap();
      let higher_bound: u64 = matches.value_of("HIGHBOUND").unwrap_or("999999").parse::<u64>().unwrap();

      if lower_bound == 111111{
        println!("Lower bound defaulting to 111111");
      }

      if higher_bound == 999999{
        println!("Higher bound defaulting to 999999");
      }

      println!("{}", puzzle04::puzzle04(lower_bound, higher_bound));
    }
}