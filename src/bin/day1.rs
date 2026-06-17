use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input1.txt").expect("open failed"));

    let mut floor = 0;
    let mut basement = -1;

    for line in f.lines() {
        for (i, c) in line.expect("lines failed").chars().enumerate() {
            match c {
                ')' => floor -= 1,
                '(' => floor += 1,
                _ => continue
            };


            if floor == -1 && basement == -1 {
                basement = (i + 1) as i32;
            }
        }

    }

    println!("Part 1: {floor}");
    println!("Part 2: {basement}");
}
