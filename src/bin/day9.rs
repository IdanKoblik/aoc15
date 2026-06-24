use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input9.txt").unwrap();

    let mut cities = HashSet::new();
    let mut dist = HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        let a = parts[0];
        let b = parts[2];
        let d: usize = parts[4].parse().unwrap();

        cities.insert(a);
        cities.insert(b);

        dist.insert((a, b), d);
        dist.insert((b, a), d);
    }

    let cities: Vec<_> = cities.into_iter().collect();

    let mut shortest = usize::MAX;
    let mut longest = 0;

    for route in cities.iter().permutations(cities.len()) {
        let total: usize = route
            .windows(2)
            .map(|pair| {
                let a = *pair[0];
                let b = *pair[1];
                dist[&(a, b)]
            })
            .sum();

        shortest = shortest.min(total);
        longest = longest.max(total);
    }

    println!("Part 1: {}", shortest);
    println!("Part 2: {}", longest);
}
