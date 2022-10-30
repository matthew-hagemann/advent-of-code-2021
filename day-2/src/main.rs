use std::fs;

fn main() {
    let mut horisontal_sum = 0;
    let mut vertical_sum = 0;

    let input = fs::read_to_string("input.txt")
        .expect("Unable to  read file.")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    for iter in input.iter() {
        match iter.split(" ").collect::<Vec<&str>>().as_slice() {
            ["forward", v] => horisontal_sum += v.parse::<i32>().unwrap(),
            ["down", v] => vertical_sum += v.parse::<i32>().unwrap(),
            ["up", v] => vertical_sum -= v.parse::<i32>().unwrap(),
            _ => panic!("Somthing went wrong in match statement."),
        }
    }

    println!("Part 1: {}", horisontal_sum * vertical_sum);

    let mut horisontal_sum = 0;
    let mut aim_sum = 0;
    let mut depth_sum = 0;

    for iter in input.iter() {
        match iter.split(" ").collect::<Vec<&str>>().as_slice() {
            ["forward", v] => {
                let val = v.parse::<i32>().unwrap();
                horisontal_sum += val;
                depth_sum += aim_sum * val;
            }
            ["down", v] => aim_sum += v.parse::<i32>().unwrap(),
            ["up", v] => aim_sum -= v.parse::<i32>().unwrap(),
            _ => panic!("Somthing went wrong in match statement."),
        }
    }

    println!("Part 2: {}", horisontal_sum * depth_sum);
}
