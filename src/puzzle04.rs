use regex::Regex;

fn to_array(x: u64) -> [u64; 6]{
    let preformat: &str = &x.to_string();
    let mut out: [u64; 6] = [0,0,0,0,0,0];

    let mut pos: u64 = 0;
    for i in Regex::new(r"\w{1}").unwrap().captures_iter(preformat){
        out[pos as usize] = i[0].parse::<u64>().unwrap();
        pos += 1;
    }

    return out;
}

fn valid(x: [u64; 6]) -> bool{
    let mut repetition: bool = false;
    let mut ordered: bool = true;

    for i in 0..5{
        if x[i as usize] > x[(i + 1) as usize]{
            ordered = false;
        }else if  x[i as usize] == x[(i + 1) as usize]{
            repetition = true;
        }
    }

    if ordered && repetition{
        return true;
    }

    return false;
}

pub fn puzzle04(bound_low: u64, bound_high: u64) -> u64{
    let result: Vec<u64> = vec![0;6];
    let mut password_count: u64 = 0;
    let mut progress: u64 = 0;

    for i in bound_low..bound_high{
        if valid(to_array(i)){
            println!("{}", i);
            password_count += 1;
        }
        let mut temp: u64 = ((i - bound_low) * 100) / (bound_high - bound_low);
        if temp != progress{
            progress = temp;
            println!("[{}%]", progress);
        }
    }

    return password_count;
}