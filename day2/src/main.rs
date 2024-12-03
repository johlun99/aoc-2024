fn main() {
    use std::time::Instant;

    let input = include_str!("input.txt");
    let parsed_input = parse_input(String::from(input));

    let now = Instant::now();

    println!("Part1: {}", part1(&parsed_input));
    println!("Elapsed: {:.2?}", now.elapsed());

    let now = Instant::now();

    println!("Part2: {}", part2(&parsed_input));
    println!("Elapsed: {:.2?}", now.elapsed());
}

fn part1(input: &Vec<Vec<u32>>) -> String {
    let mut safe_seq = 0;

    for seq in input {
        if check_levels(seq) {
            safe_seq += 1;
        }
    }

    safe_seq.to_string()
}

fn part2(input: &Vec<Vec<u32>>) -> String {
    let mut safe_seq = 0;

    for seq in input {
        if check_levels(seq) {
            safe_seq += 1;
            continue;
        }

        for i in 0..seq.len() {
            let mut modified_levels = seq.clone();
            modified_levels.remove(i);

            if check_levels(&modified_levels) {
                safe_seq += 1;
                break;
            }
        }
    }

    safe_seq.to_string()
}

fn check_levels(levels: &[u32]) -> bool {
    let mut is_increasing: Option<bool> = None;

    for n in levels.windows(2) {
        let diff = n[1] as i32 - n[0] as i32;

        // Check if the diff is within bounds
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        match is_increasing {
            None => is_increasing = Some(diff > 0),
            Some(true) if diff <= 0 => return false,
            Some(false) if diff > 0 => return false,
            _ => (), // continue if consistent
        }
    }

    true
}

fn parse_input(input: String) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        let parsed_row: Result<Vec<u32>, _> = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|val| val.parse::<u32>())
            .collect();

        match parsed_row {
            Ok(numbers) => result.push(numbers),
            Err(e) => println!("Faild to parse a number: {}", e),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9 

     */
    #[test]
    fn test_parse_input() {
        let input = include_str!("sample-input.txt");
        let parsed_input = parse_input(input.to_string());

        assert_eq!(parsed_input, vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt");
        let parsed_input = parse_input(input.to_string());
        assert_eq!(part1(&parsed_input), 2.to_string());

        println!("Parsed input: {:?}", parsed_input);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("sample-input.txt");
        let parsed_input = parse_input(input.to_string());
        assert_eq!(part2(&parsed_input), 4.to_string());
    }
}
