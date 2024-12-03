use regex::Regex;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = include_str!("input.txt").to_string();
    let parsed_input = parse_input(&input);
    let conditional_input = parse_conditional(&input);

    println!("\n===================");
    println!("Part 1: {}", calculate(parsed_input));
    println!("Part 2: {}", calculate(conditional_input));
    println!("===================");
    println!("Elapsed: {:.2?}", now.elapsed());
    println!("===================\n");
}

fn calculate(input: Vec<(i32, i32)>) -> String {
    let mut result = 0;

    for (x, y) in input {
        result += x * y;
    }

    result.to_string()
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut expressions = Vec::new();

    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();

        expressions.push((x, y));
    }

    expressions
}

fn parse_conditional(input: &str) -> Vec<(i32, i32)> {
    let mut conditional_string = String::from("");
    let mut valid = true;

    for i in 0..input.len() {
        if input[i..].starts_with("do()") {
            valid = true;
        } else if input[i..].starts_with("don't()") {
            valid = false;
        }

        if valid {
            conditional_string.push(input[i..].chars().next().unwrap());
        }
    }

    parse_input(conditional_string.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("sample-input.txt").to_string();
        let parsed_input = parse_input(&input);

        assert_eq!(parsed_input, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt").to_string();
        let parsed_input = parse_input(&input);

        assert_eq!(calculate(parsed_input), 161.to_string());
    }

    #[test]
    fn parse_input_conditionally() {
        let input = include_str!("sample-input-2.txt").to_string();
        let parsed_input = parse_conditional(&input);

        assert_eq!(parsed_input, vec![(2, 4), (8, 5)]);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("sample-input-2.txt").to_string();
        let parsed_input = parse_conditional(&input);

        assert_eq!(calculate(parsed_input), 48.to_string());
    }
}
