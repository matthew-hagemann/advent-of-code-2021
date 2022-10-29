use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to  read file.")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let result = input
        .clone()
        .iter()
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count();

    println!("Part 1: {}", result);

    let result_2 = input
        .iter()
        .tuple_windows()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows()
        .filter(|(p, q)| p < q)
        .count();

    println!("Part 2: {}", result_2);
}
