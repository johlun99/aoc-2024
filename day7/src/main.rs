fn main() {
    use std::time::Instant;
    let now_total = Instant::now();

    let input = include_str!("input.txt");
    let parsed_input = parse_input(input);

    println!("\nDay 7");
    println!("=======================");
    let now_p = Instant::now();
    println!("Part 1: {}", solve(&parsed_input, false));
    println!("Elapsed: {:.2?}\n", now_p.elapsed());

    let now_p = Instant::now();
    println!("Part 2: {}", solve(&parsed_input, true));
    println!("Elapsed: {:.2?}", now_p.elapsed());
    println!("=======================");
    println!("Elapsed: {:.2?}\n", now_total.elapsed());
}

fn solve(input: &Vec<(u64, Vec<u64>)>, part2: bool) -> u64 {
    let mut found_combinations = Vec::new();
    let mut total = 0;

    for (answer, values) in input {
        let mut combinations = find_combinations(answer, values, part2);

        let len = combinations.len();
        if len > 0 {
            total += answer;
        }

        found_combinations.append(&mut combinations);
    }

    total
}

fn find_combinations(answer: &u64, values: &Vec<u64>, include_join: bool) -> Vec<String> {
    let len = values.len();
    let mut found = Vec::new();

    let operators: i32 = if include_join { 3 } else { 2 };

    for bm in 0..(operators.pow((len - 1) as u32)) {
        let mut current = values[0];
        let mut exp = values[0].to_string();
        let mut mask = bm;

        for i in 0..(len - 1) {
            // 0: +, 1: *, 2: ||
            let operator = mask % operators;
            mask /= operators;

            match operator {
                0 => {
                    // +
                    current += values[i + 1];
                    exp.push_str(&format!(" + {}", values[i + 1]));
                }
                1 => {
                    // *
                    current *= values[i + 1];
                    exp.push_str(&format!(" * {}", values[i + 1]));
                }
                2 => {
                    // join(||)
                    let j = format!("{}{}", current, values[i + 1])
                        .parse::<u64>()
                        .expect("Invalid joined value");

                    current = j;
                    exp.push_str(&format!(" || {}", values [i + 1]));
                }
                _ => panic!("Should be impossible o.O"),
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
    fn test_find_combinations_part1() {
        assert_eq!(find_combinations(&190, &vec![10, 19], false), vec!["10 * 19"]);
        assert_eq!(find_combinations(&3267, &vec![81, 40, 27], false), vec!["81 * 40 + 27", "81 + 40 * 27"]);
        assert_eq!(find_combinations(&292, &vec![11, 6, 16, 20], false), vec!["11 + 6 * 16 + 20"]);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt");
        let parsed_input = parse_input(input);

        assert_eq!(solve(&parsed_input, false), 3749);
    }

    #[test]
    fn test_find_combinations_part2() {
        assert_eq!(find_combinations(&156, &vec![15, 6], true), vec!["15 || 6"]);
        assert_eq!(find_combinations(&7290, &vec![6, 8, 6, 15], true), vec!["6 * 8 || 6 * 15"]);
        assert_eq!(find_combinations(&192, &vec![17, 8, 14], true), vec!["17 || 8 + 14"]);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("sample-input.txt");
        let parsed_input = parse_input(input);

        assert_eq!(solve(&parsed_input, true), 11387);
    }
}
