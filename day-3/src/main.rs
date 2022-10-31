use itertools::join;
use std::fs;

fn main() {
    let gamma: Vec<u32> = fs::read_to_string("input.txt")
        .expect("Unable to  read file.")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .fold(vec![0; 6], |mut acc, vec| {
            acc[0] += vec[0];
            acc[1] += vec[1];
            acc[2] += vec[2];
            acc[3] += vec[3];
            acc[4] += vec[4];
            acc[5] += vec[5];
            acc
        })
        .iter()
        .map(|v| if v > &500 { 1 } else { 0 })
        .collect();

    let epsilon: Vec<u32> = gamma
        .clone()
        .iter()
        .map(|v| if v.eq(&1) { 0 } else { 1 })
        .collect();

    println!("{:?}", gamma);
    println!("{:?}", epsilon);
}
