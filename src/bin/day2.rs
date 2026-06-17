use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input2.txt").expect("open failed"));

    let mut sum = 0;
    let mut ribbon = 0;
    for line in f.lines() {
        let line = line.expect("failed to parse line");

        let dims: Vec<i32> = line.split('x')
            .map(|x| x.parse().unwrap())
            .collect();

        let (l, w, h) = (dims[0], dims[1], dims[2]);
        let mut sides = [l, w, h];
        sides.sort();

        let slack = sides[0] * sides[1];

        sum += 2*l*w + 2*w*h + 2*h*l + slack;
        ribbon += 2*sides[0] + 2*sides[1] + l*w*h;
    }

    println!("Part 1: {sum}");
    println!("Part 2: {ribbon}");
}
