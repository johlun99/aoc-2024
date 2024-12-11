fn main() {
    let input = include_str!("input.txt");
    let input = parse_input(input);

    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&input, 25));
    println!("Input: {:?}", input);
    println!("Part 2: {}", part1(&input, 75));
}

fn part1(input: &[u64], iterations: u32) -> u64 {
    let mut current_input = input.to_vec();

    for i in 0..iterations {
        println!("i: {}", i);

        current_input = rearrange_row(&current_input);
    }

    current_input.len() as u64
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .filter_map(|num| num.parse::<u64>().ok())
        .collect()
}

fn rearrange_row(row: &Vec<u64>) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::with_capacity(row.len() * 1);

    for num in row {
        if *num == 0 {
            output.push(1);
        } else if num.to_string().chars().count() % 2 == 0 {
            let num_as_string = num.to_string();
            let mid = num_as_string.len() / 2;

            let (first, second) = num_as_string.split_at(mid);

            output.push(first.parse::<u64>().expect("Invalid first num..."));
            output.push(second.parse::<u64>().expect("Invalid first num..."));
        } else {
            output.push(num * 2024);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("short-sample-input.txt");
        let parsed_input = parse_input(input);
        let expected_input = vec![0, 1, 10, 99, 999];

        assert_eq!(parsed_input, expected_input);
    }

    #[test]
    fn test_rearrange_row() {
        let input = vec![0, 1, 10, 99, 999];
        assert_eq!(rearrange_row(&input), vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt");
        let input = parse_input(input);

        assert_eq!(part1(&input, 6), 55312);
    }
}
