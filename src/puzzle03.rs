use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub enum Movement{
    Right(i64),
    Left(i64),
    Up(u64),
    Down(u64)
}

pub struct Grid{
    wires: Vec<i64>,
    current_pos: Vec<i64>
}

pub fn largestValue(path: &str) -> i64{
    let mut input = File::open(path).expect("largestValue failed to open input file");
    let mut input_buffer = String::new();
    let mut largest: i64 = 0;

    input
      .read_to_string(&mut input_buffer)
      .expect("Failed to read from input file");
    
    for i in Regex::new(r"\d+").unwrap().captures_iter(&input_buffer[..]){
        for j in i[0].lines(){
            let value: i64 = j.parse::<i64>()
              .expect("Failed converting &str to int in largestValue()");
            if value > largest{
                largest = value;
            }
        }
    }

    return largest;
}

impl Grid{
    pub fn grid() -> Grid{
        let grid: Grid = Grid{
            wires: vec![0;0],
            current_pos: vec![0; 2]
        };
        return grid;
    }

    pub fn get_pos(&self) -> Vec<i64>{
        return vec![self.current_pos[0], self.current_pos[1]];
    }

    pub fn move_wire(movement: Movement){
        match movement{
            Movement::Right(distance) => println!("Distance: {}\n", distance),
            Movement::Left(distance) => println!("Distance: {}\n", distance),
            Movement::Up(distance) => println!("Distance: {}\n", distance),
            Movement::Down(distance) => println!("Distance: {}\n", distance)
        }
    }
}