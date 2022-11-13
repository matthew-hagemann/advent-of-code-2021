use std::fs;

const LENGTH: usize = 12;

fn main() {
    let input: Vec<u32> = fs::read_to_string("input.txt")
        .expect("Unable to  read file.")
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect();

    let (gamma, epsilon) = (
        get_rate_from_vec(&input, true),
        get_rate_from_vec(&input, false),
    );

    println!("Part 1: {}", gamma * epsilon);

    let (ogr, csr) = (find_rating(&input, true), find_rating(&input, false));

    println!("Part 2: {}", ogr * csr);
}

fn get_rate_from_vec(input: &Vec<u32>, b: bool) -> u32 {
    input
        .iter()
        .fold(vec![0; LENGTH], |acc, bin| {
            let mut t = acc.clone();
            for (i, _el) in acc.clone().iter().enumerate() {
                t[i] = {
                    if bin & (1 << (LENGTH - i - 1)) > 1 {
                        acc[i] + 1
                    } else {
                        acc[i]
                    }
                }
            }
            t
        })
        .iter()
        .fold(0, |r: u32, v| {
            if b == (&(v * 2) >= &(input.len())) {
                !(!r << 1)
            } else {
                r << 1
            }
        })
}

fn find_rating(input: &Vec<u32>, b: bool) -> u32 {
    let mut pos = 1;
    let mut vec: Vec<u32> = input.clone();
    while vec.len() > 1 {
        let pos_avg = vec.iter().fold(0, |a, v| {
            if v & (1 << (LENGTH - pos)) > 1 {
                a + 1
            } else {
                a
            }
        });
        vec = vec
            .clone()
            .into_iter()
            .filter(|x| {
                if b == (pos_avg * 2 >= vec.len()) {
                    x & (1 << (LENGTH - pos)) > 0
                } else {
                    !(x & (1 << (LENGTH - pos)) > 0)
                }
            })
            .collect();
        pos += 1;
    }
    vec[0]
}
