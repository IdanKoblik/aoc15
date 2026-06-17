use std::collections::HashSet;
use std::fs;

#[derive(Hash, Eq, PartialEq, Clone, Default)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    let data: String = fs::read_to_string("input3.txt").unwrap();
    let mut cords: HashSet<Pos> = HashSet::new();
    let mut pos = Pos::default();
    let mut robot = Pos::default();

    cords.insert(pos.clone());

    for c in data.chars() {
        match c {
            '^' => pos.y += 1,
            'v' => pos.y -= 1,
            '>' => pos.x += 1,
            '<' => pos.x -= 1,
            _ => continue
        }

        cords.insert(pos.clone());
        std::mem::swap(&mut pos, &mut robot); // Used for part 2
    }

    let size = cords.len();
    println!("{size}")
}
