use std::fs;

#[derive(Hash, Eq, PartialEq, Clone, Default)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    // Input: x,y
    fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split(',').collect();

        return Self {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap()
        }
    }
}

fn main() {
    let input: String = fs::read_to_string("input6.txt").unwrap();
    let mut on = vec![false; 1000*1000];
    let mut power = vec![0u32; 1000*1000];

    for line in input.lines() {
        let mut iter = line.split_whitespace().peekable();
        iter.next_if_eq(&"turn");
        let action = iter.next().unwrap();
        let start = Pos::parse(iter.next().unwrap());
        iter.next().unwrap();
        let end = Pos::parse(iter.next().unwrap());


        for i in start.y..=end.y {
            for j in start.x..=end.x {
                let index = j + i * 1000;

                match action {
                    "on" => {
                        on[index] = true;
                        power[index] += 1;
                    },
                    "off" => {
                        on[index] = false;
                        power[index] = power[index].saturating_sub(1);
                    },
                    "toggle" => {
                        on[index] = !on[index];
                        power[index] += 2;
                    },
                    _ => {}
                };
            }
        }
    }

    let count = on.iter().filter(|v| **v).count();
    let sum: u32 = power.iter().sum();
    println!("Part 1: {count}");
    println!("Part 2: {sum}");
}
