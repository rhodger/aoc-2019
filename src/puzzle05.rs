use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::io;

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_load(){
        let mut comp: Comp = Comp{
            mem: vec![0;0]
        };
        comp.load("./content/input051.txt");
        assert_eq!(comp.mem[0], 3);
        assert_eq!(comp.mem[1], 225);
        assert_eq!(comp.mem[2], 1);
    }

    #[test]
    fn test_constructor(){
        let mut comp: Comp = Comp::comp("./content/input051.txt");
    }

    #[test]
    fn test_add(){
        let mut comp: Comp = Comp::comp("./content/input051.txt");

		comp.add(01101, 2, 2, 0);
		println!("2 + 2 = {}", comp.mem[0]);
		assert_eq!(comp.mem[0], 4);

		comp.add(00001, 1, 2, 0);
		assert_eq!(comp.mem[0], 226);

		comp.add(01101, 2, -1, 0);
		assert_eq!(comp.mem[0], 1);
    }

	#[test]
	fn test_sub(){
        let mut comp: Comp = Comp::comp("./content/input051.txt");

		comp.sub(01198, 8, 3, 0);
		println!("{} - {} = {}", 8, 3, comp.mem[0]);
		assert_eq!(comp.mem[0], 5);

		comp.sub(00098, 1, 2, 0);
		println!("{} - {} = {}", comp.mem[1], comp.mem[2], comp.mem[0]);
		assert_eq!(comp.mem[0], 224);

		comp.sub(01198, 2, -1, 0);
		println!("{} - {} = {}", 2, -1, comp.mem[0]);
		assert_eq!(comp.mem[0], 3);
	}

	#[test]
	fn test_mul(){
    	let mut comp: Comp = Comp::comp("./content/input051.txt");

		comp.mul(01102, 2, 3, 0);
		println!("{} * {} = {}", 2, 3, comp.mem[0]);
		assert_eq!(comp.mem[0], 6);

		comp.mul(00002, 4, 5, 0);
		println!("{} * {} = {}", comp.mem[4], comp.mem[5], comp.mem[0]);
		assert_eq!(comp.mem[0], 36);

		comp.mul(01102, 2, -1, 0);
		println!("{} * {} = {}", 2, -1, comp.mem[0]);
		assert_eq!(comp.mem[0], -2);
	}

	#[test]
	fn test_put(){
		let mut comp: Comp = Comp::comp("./content/input051.txt");

		assert_eq!(comp.put(004, 0), 3);
		assert_eq!(comp.put(104, 69), 69);
		assert_eq!(comp.put(104, -9), -9);
	}
	
	#[test]
	#[ignore]
	fn test_get(){
		let mut comp: Comp = Comp::comp("./content/input051.txt");

		println!("Enter 69");
		assert_eq!(comp.get(003, 0), 69);

		println!("Enter 420");
		assert_eq!(comp.get(003, 0), 420);

		println!("Enter -9");
		assert_eq!(comp.get(003, 0), -9);
	}
}

pub struct Comp{
    mem: Vec<i64>
}

impl Comp{
    pub fn comp(path: &str) -> Comp{
        let mut comp: Comp = Comp{
            mem: vec![0;0]
        };
		comp.load(path);
        return comp;
    }

    fn load(&mut self, path: &str){
        let mut input = File::open(path).expect("Failed to open input file");
        let mut input_buffer = String::new();
        let mut out: Vec<i64> = vec![0;0];

        input.read_to_string(&mut input_buffer)
            .expect("Failed to read from input file");

        for i in Regex::new(r"\d+").unwrap().captures_iter(&input_buffer){
            out.push(i[0].parse::<i64>().unwrap());
        }

        self.mem = out;
    }

    fn add(&mut self, opcode: i64, x: i64, y: i64, z: i64){
        let s_opcode: &str = &format!("{:0>5}", opcode.to_string());
        let x_value: i64 = 0;

        let x_value: i64 = match s_opcode.chars().nth(2).unwrap(){
            '0' => self.mem[x as usize], 
            _ => x
        };

		let y_value: i64 = match s_opcode.chars().nth(1).unwrap(){
			'0' => self.mem[y as usize],
			_ => y
		};

		self.mem[z as usize] = x_value + y_value;
    }

	fn sub(&mut self, opcode: i64, x: i64, y: i64, z: i64){
        let s_opcode: &str = &format!("{:0>5}", opcode.to_string());
        let x_value: i64 = 0;

        let x_value: i64 = match s_opcode.chars().nth(2).unwrap(){
            '0' => self.mem[x as usize], 
            _ => x
        };

		let y_value: i64 = match s_opcode.chars().nth(1).unwrap(){
			'0' => self.mem[y as usize],
			_ => y
		};

		self.mem[z as usize] = x_value - y_value;
		
	}

	fn mul(&mut self, opcode: i64, x: i64, y: i64, z: i64){
        let s_opcode: &str = &format!("{:0>5}", opcode.to_string());
        let x_value: i64 = 0;

        let x_value: i64 = match s_opcode.chars().nth(2).unwrap(){
            '0' => self.mem[x as usize], 
            _ => x
        };

		let y_value: i64 = match s_opcode.chars().nth(1).unwrap(){
			'0' => self.mem[y as usize],
			_ => y
		};

		self.mem[z as usize] = x_value * y_value;
	}

	fn put(&self, opcode: i64, x: i64) -> i64{
        let s_opcode: &str = &format!("{:0>3}", opcode.to_string());
		let x_value: i64 = match s_opcode.chars().nth(0).unwrap(){
			'0' => self.mem[x as usize],
			_ => x
		};

		println!("{}", x_value);
		return x_value;
	}

	fn get(&mut self, opcode: i64, z: i64) -> i64{
		let mut input_text = String::new();
		
		io::stdin()
			.read_line(&mut input_text)
			.expect("failed to read from stdin");

		let trimmed = input_text.trim();
		let z_value = match trimmed.parse::<i64>() {
			Ok(i) => i,
			Err(..) => panic!("Non-integer input")
		};

		self.mem[z as usize] = z_value;
		return z_value;
	}
}
