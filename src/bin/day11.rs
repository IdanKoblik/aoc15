fn inc(p: &mut [u8]) {
    for c in p.iter_mut().rev() {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            break;
        }
    }
}

fn valid(p: &[u8]) -> bool {
    let straight = p.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
    let no_bad = !p.iter().any(|&c| matches!(c, b'i' | b'o' | b'l'));

    let mut pairs = 0;
    let mut i = 0;
    while i + 1 < p.len() {
        if p[i] == p[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    return straight && no_bad && pairs >= 2;
}

fn next_pass(s: &str) -> String {
    let mut p = s.as_bytes().to_vec();

    loop {
        inc(&mut p);
        if valid(&p) {
            return String::from_utf8(p).unwrap();
        }
    }
}

fn main() {
    let first_pass = next_pass("hepxcrrq");
    let second_pass = next_pass(first_pass.as_str());
    println!("Part 1: {first_pass}");
    println!("Part 2: {second_pass}");
}
