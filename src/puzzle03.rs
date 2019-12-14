pub enum Movement{
    right(i64),
    left(i64),
    up(u64),
    down(u64)
}

pub struct Grid{
    wires: Vec<i64>,
    currentPos: Vec<i64>
}

impl Grid{
    pub fn Grid() -> Grid{
        let grid: Grid = Grid{
            wires: vec![0;0],
            currentPos: vec![0; 2]
        };
        return grid;
    }

    pub fn move_wire(movement: Movement){
        match movement{
            Movement::right(distance) => println!("Distance: {}\n", distance),
            Movement::left(distance) => println!("Distance: {}\n", distance),
            Movement::up(distance) => println!("Distance: {}\n", distance),
            Movement::down(distance) => println!("Distance: {}\n", distance)
        }
    }
        
    
}