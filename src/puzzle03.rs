use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub enum Movement{
    RIGHT(i64),
    LEFT(i64),
    UP(i64),
    DOWN(i64)
}

pub struct Grid{
    wires: Vec<i64>,
    current_pos: Vec<i64>
}

pub fn largestValue(path: &str) -> i64{
    let mut input = File::open(path)
      .expect("largestValue failed to open input file");
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

    fn move_wire(&mut self, movement: Movement){
        match movement{
            Movement::RIGHT(distance) =>
                for i in self.current_pos[0]..(self.current_pos[0] + distance){
                    // push to array
                },
            Movement::LEFT(distance) => println!("Left: {}\n", distance),
            Movement::UP(distance) => println!("Up: {}\n", distance),
            Movement::DOWN(distance) => println!("Down: {}\n", distance)
        }
    }

    pub fn execute(&mut self, path: &str){
        let mut input = File::open(path)
          .expect("Grid::execute() failed to open input file");
        let mut input_buffer = String::new();
        let mut largest: i64 = 0;

        input
          .read_to_string(&mut input_buffer)
          .expect("Failed to read from input file");
        
        for i in input_buffer.lines(){
            for j in Regex::new(r"\w\d+").unwrap().captures_iter(&input_buffer[..]){
                for k in j[0].lines(){
                    match &k[..1]{
                        "R" => self.move_wire(Movement::RIGHT(k[1..].parse::<i64>().unwrap())),
                        "L" => self.move_wire(Movement::LEFT(k[1..].parse::<i64>().unwrap())),
                        "U" => self.move_wire(Movement::UP(k[1..].parse::<i64>().unwrap())),
                        "D" => self.move_wire(Movement::RIGHT(k[1..].parse::<i64>().unwrap())),
                        _ => println!("Invalid character")
                    }
                }
            }
        }
    }
}