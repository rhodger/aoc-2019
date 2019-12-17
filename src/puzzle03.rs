use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

pub enum Movement{
    RIGHT(i64),
    LEFT(i64),
    UP(i64),
    DOWN(i64)
}

pub struct Grid{
    wires: HashMap<Vec<i64>, i64>,
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
            wires: HashMap::new(),
            current_pos: vec![0; 2]
        };
        return grid;
    }

    pub fn get_pos(&self) -> Vec<i64>{
        return vec![self.current_pos[0], self.current_pos[1]];
    }

    fn move_wire(&mut self, movement: Movement, current_steps: i64) -> i64{
        let mut new_steps: i64 = current_steps;
        match movement{
            Movement::RIGHT(distance) =>
                for i in self.current_pos[0]..(self.current_pos[0] + distance + 1){
                    self.current_pos[0] = i;
                    if !self.wires.contains_key(&vec![self.current_pos[0], self.current_pos[1]]){
                        self.wires.insert(vec![self.current_pos[0], self.current_pos[1]], new_steps);
                    }else{
                        *self.wires.get_mut(&vec![self.current_pos[0], self.current_pos[1]]).unwrap() += 1;
                    }
                    new_steps += 1;
                },
            Movement::LEFT(distance) =>
                for i in ((self.current_pos[0] - distance)..self.current_pos[0] + 1).rev(){
                    self.current_pos[0] = i;
                    if !self.wires.contains_key(&vec![self.current_pos[0], self.current_pos[1]]){
                        self.wires.insert(vec![self.current_pos[0], self.current_pos[1]], new_steps);
                    }else{
                        *self.wires.get_mut(&vec![self.current_pos[0], self.current_pos[1]]).unwrap() += 1;
                    }
                    new_steps += 1;
                },
            Movement::UP(distance) =>
                for i in self.current_pos[1]..(self.current_pos[1] + distance + 1){
                    self.current_pos[1] = i;                    
                    if !self.wires.contains_key(&vec![self.current_pos[0], self.current_pos[1]]){
                        self.wires.insert(vec![self.current_pos[0], self.current_pos[1]], new_steps);
                    }else{
                        *self.wires.get_mut(&vec![self.current_pos[0], self.current_pos[1]]).unwrap() += 1;
                    }
                    new_steps += 1;
                },
            Movement::DOWN(distance) =>
                for i in ((self.current_pos[1] - distance)..self.current_pos[1] + 1).rev(){
                    self.current_pos[1] = i;
                    if !self.wires.contains_key(&vec![self.current_pos[0], self.current_pos[1]]){
                        self.wires.insert(vec![self.current_pos[0], self.current_pos[1]], new_steps);
                    }else{
                        *self.wires.get_mut(&vec![self.current_pos[0], self.current_pos[1]]).unwrap() += 1;
                    }
                    new_steps += 1;
                }
        }
        return new_steps;
    }

    fn execute(&mut self, instructions: &str){
        let mut steps: i64 = 0;
        for j in Regex::new(r"\w\d+").unwrap().captures_iter(&instructions){
            for k in j[0].lines(){
                match &k[..1]{
                    "R" => steps = self.move_wire(Movement::RIGHT(k[1..].parse::<i64>().unwrap()), steps) - 1,
                    "L" => steps = self.move_wire(Movement::LEFT(k[1..].parse::<i64>().unwrap()), steps) - 1,
                    "U" => steps = self.move_wire(Movement::UP(k[1..].parse::<i64>().unwrap()), steps) - 1,
                    "D" => steps = self.move_wire(Movement::DOWN(k[1..].parse::<i64>().unwrap()), steps) - 1,
                    _ => println!("Invalid character")
                }
                println!("Took {} steps", steps);
            }
        }
    }

    fn compare(grid1: &Grid, grid2: &Grid) -> i64{
        let mut best: i64 = 0;
        for (i, j) in &grid1.wires{
            if grid2.wires.contains_key(i){
                if best == 0 || ((i[0].abs() + i[1].abs()) < best && (i[0].abs() + i[1].abs()) != 0){
                    best = i[0].abs() + i[1].abs();
                }
                println!("Checked {}, {}", i[0], i[1]);
            }
        }
        return best;
    }

    fn compare_steps(grid1: &Grid, grid2: &Grid) -> i64{
        let mut best: i64 = 0;
        for (i, j) in &grid1.wires{
            if grid2.wires.contains_key(i){
                println!("Intersection @ {}, {}", i[0], i[1]);
                if (i[0] != 0 || i[1] != 0) && ((j + grid2.wires.get(i).unwrap() < best) || best == 0){
                    println!("New best: {} & {}", j, grid2.wires.get(i).unwrap());
                    best = (j + grid2.wires.get(i).unwrap());
                }
            }
        }
        return best;
    }

    pub fn display(&self){
        for i in &self.wires{
            println!("{}, {} : {}", i.0[0], i.0[1], i.1);
        }
    }
}

pub fn puzzle2(path: &str) -> i64{
    let mut grid1: Grid = Grid::grid();
    let mut grid2: Grid = Grid::grid();

    let mut input = File::open(path)
      .expect("Grid::puzzle1() failed to open input file");
    let mut input_buffer = String::new();
    let mut largest: i64 = 0;

    input
      .read_to_string(&mut input_buffer)
      .expect("Failed to read from input file");
    
    let mut wire_instructions: std::str::Lines = input_buffer.lines();
    let mut wire1: &str = wire_instructions.next().unwrap();
    let mut wire2: &str = wire_instructions.next().unwrap();

    println!("Building first wire...");
    grid1.execute(wire1);
    println!("Building second wire...");
    grid2.execute(wire2);

    println!("Finding best match...");

    return Grid::compare_steps(&grid1, &grid2);
}

pub fn puzzle1(path: &str) -> i64{
    let mut grid1: Grid = Grid::grid();
    let mut grid2: Grid = Grid::grid();

    let mut input = File::open(path)
      .expect("Grid::puzzle1() failed to open input file");
    let mut input_buffer = String::new();
    let mut largest: i64 = 0;

    input
      .read_to_string(&mut input_buffer)
      .expect("Failed to read from input file");
    
    let mut wire_instructions: std::str::Lines = input_buffer.lines();
    let mut wire1: &str = wire_instructions.next().unwrap();
    let mut wire2: &str = wire_instructions.next().unwrap();

    println!("Building first wire...");
    grid1.execute(wire1);
    println!("Building second wire...");
    grid2.execute(wire2);

    println!("Finding best match...");

    return Grid::compare(&grid1, &grid2);
}