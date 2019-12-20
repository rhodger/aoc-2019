use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

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
}

pub struct Comp{
    mem: Vec<i64>
}

impl Comp{
    pub fn comp(path: &str) -> Comp{
        let comp: Comp = Comp{
            mem: vec![0;0]
        };
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
}