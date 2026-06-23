use std::collections::HashMap;

struct Electrohyperultramicromultiprocessorinterconnectcircuitizationology {
    rules: HashMap<String, Vec<String>>,
    cache: HashMap<String, u16>,
}

impl Electrohyperultramicromultiprocessorinterconnectcircuitizationology {
    fn new(rules: HashMap<String, Vec<String>>) -> Self {
        return Self {
            rules,
            cache: HashMap::new(),
        }
    }

    fn eval(&mut self, ex: &str) -> u16 {
        if let Ok(num) = ex.parse::<u16>() {
            return num;
        }

        if self.cache.get(ex) != None {
            return *self.cache.get(ex).unwrap();
        }

        let expr = self.rules.get(ex).unwrap().clone();
        let mut value = 0;
        if expr.len() == 1 {
            return self.eval(expr[0].as_str());
        }

        if expr[0] == "NOT" {
            value = !self.eval(expr[1].as_str());
        }

        let op = &expr[1];
        if op == "AND" {
            value = self.eval(expr[0].as_str()) & self.eval(expr[2].as_str());
        }

        if op == "OR" {
            value = self.eval(expr[0].as_str()) | self.eval(expr[2].as_str());
        }

        if op == "LSHIFT" {
            value = self.eval(expr[0].as_str()) << self.eval(expr[2].as_str());
        }

        if op == "RSHIFT" {
            value = self.eval(expr[0].as_str()) >> self.eval(expr[2].as_str());
        }

        self.cache.insert(ex.to_string(), value);
        return value;
    }
}

fn parse(lines: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in lines.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();

        if parts.len() == 2 {
            let key = parts[1].to_string();
            let values = parts[0]
                .split_whitespace()
                .map(String::from)
                .collect();

            map.insert(key, values);
        }
    }

    return map
}

fn main() {
    let rules = parse(std::fs::read_to_string("input7.txt").unwrap().as_str());
    let mut circut = Electrohyperultramicromultiprocessorinterconnectcircuitizationology::new(rules);
    let a = circut.eval("a");
    println!("Part 1: {a}");

    circut.cache.clear();
    circut.cache.insert(String::from("b"), a);

    let a = circut.eval("a");
    println!("Part 2: {a}");
}
