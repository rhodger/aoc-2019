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