use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let f = BufReader::new(File::open("input5.txt").expect("open failed"));

    let mut first_sum = 0;
    let mut second_sum = 0;
    for line in f.lines() {
        let line = line.expect("failed to parse line");
        if is_nice_part1(&line) {
            first_sum += 1;
        }

        if is_nice_part2(&line) {
            second_sum += 1;
        }
    }

    println!("Part 1: {first_sum}");
    println!("Part 1: {second_sum}");
}

fn is_nice_part1(line: &str) -> bool {
    if ["ab", "cd", "pq", "xy"].iter().any(|s| line.contains(s)) {
        return false;
    }

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let vowel_count = line
        .chars()
        .filter(|c| vowels.contains(c))
        .count();

    if vowel_count < 3 {
        return false;
    }

    let chars: Vec<char> = line.chars().collect();

    let has_double = chars
        .windows(2)
        .any(|pair| pair[0] == pair[1]);

    return has_double;
}

fn is_nice_part2(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();

    let has_repeated_pair = chars.windows(2).enumerate().any(|(i, pair)| {
        chars[i + 2..]
            .windows(2)
            .any(|other| other == pair)
    });

    if !has_repeated_pair {
        return false;
    }

    let has_repeat_with_gap = chars
        .windows(3)
        .any(|triple| triple[0] == triple[2]);

    return has_repeat_with_gap;
}

