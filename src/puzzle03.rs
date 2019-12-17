//! Solution to puzzle03
//!
//! Contains the code for creating and managing Grids, the structure used for
//! recording the positions through which individual wires have passed, and for
//! using these Grids to solve the two tasks for day 3.

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

/// Types of movement
///
/// Each of these represents a movement in one of the cardinal directions
/// (represented here as up, down, left, and right, rather than NESW). Each can
/// be passed a value to represent the distance to travel in that direction.
pub enum Movement{
    RIGHT(i64),
    LEFT(i64),
    UP(i64),
    DOWN(i64)
}

/// Structure for holding the positions of a wire
///
/// Grids are essentially collections of points representing where a wire has
/// passed through; each grid records the positions of a single wire, and can be
/// compared to other grids once populated to find the solution for both of day
/// 3's tasks.
pub struct Grid{
    wires: HashMap<Vec<i64>, i64>,
    current_pos: Vec<i64>
}

/// Finds the largest integer in a string.
///
/// Finds and returns the largest integer value found in a string contained in a
/// file passed to this function as an argument.
pub fn largestValue(path: &str) -> i64{
    let mut input = File::open(path)
      .expect("largestValue failed to open input file");
    let mut input_buffer = String::new();
    let mut largest: i64 = 0;

    input
      .read_to_string(&mut input_buffer)
      .expect("Failed to read from input file");
    
    // Separates the string into a list of integers to be iterated through.
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
    /// Constructor
    pub fn grid() -> Grid{
        let grid: Grid = Grid{
            wires: HashMap::new(),
            current_pos: vec![0; 2]
        };
        return grid;
    }

    /// Returns this Grid's current_pos.
    ///
    /// Returns the current position of this grid's wire as an integer array
    /// length 2.
    pub fn get_pos(&self) -> Vec<i64>{
        return vec![self.current_pos[0], self.current_pos[1]];
    }

    /// Adds another move to this Grid's wire.
    ///
    /// Simulates another step in a wire's movement from origin. Takes a
    /// Movement enum as input along with a record of the current step count for
    /// that wire, adds the new points passed through to this grid's storage,
    /// then finally returns the steps taken + 1. This extra step is a bug dealt
    /// with in the puzzle solving functions of this module. This could be
    /// fixed in future.
    ///
    /// # Examples
    /// 
    /// In the below example, a wire movement of upwards 50 is simulated on a
    /// brand new Grid.
    /// ```
    /// let mut grid: Grid = Grid::grid();
    /// grid.move_wire(Movement::UP(50));
    /// // Returns "0, 50"
    /// println!("{}, {}", grid.get_pos()[0], grid.get_pos()[1]);
    /// ```
    fn move_wire(&mut self, movement: Movement, current_steps: i64) -> i64{
        let mut new_steps: i64 = current_steps;
        match movement{
            Movement::RIGHT(distance) =>
                // Iterates through every step between the current wire position
                // and its final resting place after the movement.
                for i in
                  self.current_pos[0]..(self.current_pos[0] + distance + 1){
                    self.current_pos[0] = i;
                    if !self.wires.contains_key(
                      &vec![self.current_pos[0], self.current_pos[1]]
                    ){
                        // Populates the Grid's storage with coordinates passed
                        // through and the steps taken to get there.
                        self.wires.insert(
                          vec![self.current_pos[0], self.current_pos[1]],
                          new_steps
                        );
                    }else{
                        *self.wires.get_mut(
                          &vec![self.current_pos[0], self.current_pos[1]]
                        ).unwrap() += 1;
                    }
                    new_steps += 1;
                },
            Movement::LEFT(distance) =>
                for i in
                  ((self.current_pos[0] - distance)..self.current_pos[0] + 1)
                  .rev()
                {
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

    /// Executes a seriues of instructions on this grid.
    ///
    /// Takes a list of instructions in the format `U50,D45,R5,etc`, parses them
    /// with regex, and calls move_wire on this Grid using the appropriate enum.
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

    /// Finds the closest interesection of two wires to the origin.
    ///
    /// Checks each set of coordinates passed through in grid1 againsts grid2
    /// and if there is an intersection, records whether or not this
    /// intersection is closer to the origin than the current best, and finally
    /// returns the overall best as an integer.
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

    /// Finds the closest interesection of two wires to the origin in terms of
    /// distance via wire.
    ///
    /// Does exactly the same as `compare()`, only uses the steps taken by each
    /// wire to reach that position (summed together) instead of manhattan
    /// distance to the origin to measure 'closeness' to the origin.
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

    /// Prints a Grid line by line
    ///
    /// Displays a Grid, mostly for debugging purposes.
    pub fn display(&self){
        for i in &self.wires{
            println!("{}, {} : {}", i.0[0], i.0[1], i.1);
        }
    }
}

/// Runs the solution for the second puzzle.
///
/// Runs the solution for the second puzzle of day 3, using the file located at
/// `path` as input. Each line of the input represents instructions for a
/// separate wire, which are then compared. In day 2, this comparison is done
/// with `compare_steps()`.
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

/// Runs the solution for the first puzzle.
///
/// Runs the solution for the first puzzle of day 3, using the file located at
/// `path` as input. Each line of the input represents instructions for a
/// separate wire, which are then compared. In day 1, this comparison is done
/// with `compare()`.
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
