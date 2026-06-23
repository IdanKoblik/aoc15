fn main() {
    let input = std::fs::read_to_string("input8.txt").unwrap();

    let mut code_sum = 0;
    let mut mem_sum = 0;
    let mut new_sum = 0;

    for line in input.lines() {
        code_sum += line.len();
        new_sum += 6;

        let bytes = line.as_bytes();
        let mut i = 1;

        while i < bytes.len() - 1{
            if bytes[i] == b'\\' {
                match bytes[i + 1] {
                    b'\\' | b'"' => {
                        mem_sum += 1;
                        new_sum += 4;
                        i += 2;
                    }
                    b'x' => {
                        mem_sum += 1;
                        new_sum += 5;
                        i += 4; // \xAB
                    }
                    _ => unreachable!(),
                }
            } else {
                mem_sum += 1;
                new_sum += 1;
                i += 1;
            }
        }
    }

    let total = code_sum - mem_sum;
    println!("Part 1: {total}");

    let total = new_sum - code_sum;
    println!("Part 2: {total}");
}
