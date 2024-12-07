fn main() {
    let input = include_str!("input.txt");
    let parsed_input = parse_input(input);

    println!("Part 1: {}", part1(&parsed_input));
}

fn part1(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut found_combinations = Vec::new();
    let mut total = 0;

    println!("\n\n");
    for (answer, values) in input {
        let mut combinations = find_combinations(answer, values);

        let len = combinations.len();
        if len > 0 {
            total += answer;
        }

        found_combinations.append(&mut combinations);
    }

    total
}

/*
fn eval_left_to_right(equation: &str) -> u32 {
    let mut parts = equation.split_whitespace();
    let mut total = parts
        .next()
        .expect("Equation cannot be empty")
        .parse::<i32>()
        .expect("Invalid number");

    while let Some(operator) = parts.next() {
        if let Some(next_value) = parts.next() {
            let value = next_value.parse::<i32>().expect("Invalid number");

            match operator {
                "+" => total += value,
                "*" => total *= value,
                _ => panic!("Unsupported operator: {}", operator),
            }
        } else {
            panic!("Incomplete equation: Missing value after operation");
        }
    }

    total as u32
}
*
*/

fn find_combinations(answer: &u64, values: &Vec<u64>) -> Vec<String> {
    let len = values.len();
    let mut found = Vec::new();

    for bm in 0..(1 << (len - 1)) {
        let mut current = values[0];
        let mut exp = values[0].to_string();

        for i in 0..(len - 1) {
            if (bm & (1 << i)) != 0 {
                // + operator
                current += values[i + 1];
                exp.push_str(&format!(" + {}", values[i + 1]));
            } else {
                // * operator
                current *= values[i + 1];
                exp.push_str(&format!(" * {}", values[i + 1]));
            }
        }

        if current == answer.clone() {
            found.push(exp);
        }
    }

    found
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty()) // ignore empty lines
        .map(|line| {
            let mut parts = line.split(':');
            let answer = parts
                .next()
                .expect("Missing key")
                .trim()
                .parse::<u64>()
                .expect("Failed to parse key as u32");

            let values = parts
                .next()
                .expect("Missing value...")
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<u64>().expect("Failed to parse value as u32"))
                .collect();

            (answer, values)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("sample-input.txt");
        let parsed_input = parse_input(input);

        let expected_input = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];

        assert_eq!(parsed_input, expected_input);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt");
        let parsed_input = parse_input(input);

        assert_eq!(part1(&parsed_input), 3749);
    }
}
