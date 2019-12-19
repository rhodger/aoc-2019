//! Solutions for puzzle 04

use regex::Regex;

/// Converts an integer into an array
///
/// Converts the 6 character integer x into an array of 6 integers, each of
/// which is a separate digit of x, then returns this array. Struggles with
/// incorrect inputs as no validation is done.
///
/// # Examples
///
/// To convert the integer 123456:
/// ```
/// let preformat: u64 = 123456;
///
/// let formatted: [u64; 6] = to_array(preformatted);
/// for i in formatted{
///     println!("{}", i);
/// }
/// ```
/// returns:
/// ```text
/// 1
/// 2
/// 3
/// 4
/// 5
/// 6
/// ```
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

/// Checks if a number is valid
///
/// Tests an array representing a 6 digit number to see if it is valid according
/// to puzzle 041 rules. This function is designed to work in tandem with
/// to_array as it requires the integer to split into an array of its digits,
/// which must be length 6. Returns a boolean representing whether the integer
/// passed the test.
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

/// Solves puzzle 041
///
/// Takes two integers representing low and high bounds as input and returns the
/// amount of valid values between these that exist. Takes a long time to run.
pub fn puzzle01(bound_low: u64, bound_high: u64) -> u64{
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