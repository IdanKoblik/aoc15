fn main() {
    let input = "iwrupvqb";

    let mut i = 1;
    for j in 1..=2 {
        let prefix = if j == 1 { "00000" } else { "000000" };
        loop {
            let digest = md5::compute(format!("{input}{i}"));
            let hash = format!("{:x}", digest);

            if hash.starts_with(prefix) {
                println!("Part {j}: {i}");
                break;
            }

            i += 1;
        }
    }
}
