fn main() {
    let mut input = String::from("1321131112");
    
    for i in 0..50 {
        let mut target = None;
        let mut count = 0;
        let mut result = String::new();

        for c in input.chars() {
            if target.is_none() {
                target = Some(c);
                count += 1;
                continue;
            }

            if c == target.unwrap() {
                count += 1;
                continue;
            }

            if let Some(ch) = target {
                result += &format!("{count}{ch}");
            }
            target = Some(c);
            count = 1;
        }

        if let Some(ch) = target {
            result += &format!("{count}{ch}");
        }

        input = result;
        let len = input.len();

        match i {
            39 => println!("Part 1: {len}"),
            49 => println!("Part 2: {len}"),
            _ => {}
        }
    }
}
