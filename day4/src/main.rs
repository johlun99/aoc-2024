use std::{char, usize};

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = include_str!("input.txt");
    let parsed_input = parse_input(input);

    let (part1, part2) = solve(&parsed_input);

    println!("\nDay 4");
    println!("===================");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("===================");
    println!("Elapsed: {:.2?}", now.elapsed());
    println!("===================\n");
}

fn solve(input: &[Vec<char>]) -> (u32, u32) {
    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'X' {
                part1 += check_for_xmas(input, x, y) as u32;
            } else if input[y][x] == 'A' {
                part2 += if check_for_x_mas(input, x, y) { 1 } else { 0 };
            }
        }
    }

    (part1, part2)
}

fn check_for_xmas(input: &[Vec<char>], x: usize, y: usize) -> usize {
    let mut found = 0;

    // Check left -> right
    if (x + 3 <= input[y].len())
        && (input[y][x + 1] == 'M' && input[y][x + 2] == 'A' && input[y][x + 3] == 'S')
    {
        found += 1;
    }

    // Check right -> left
    if (x as i32 - 3 >= 0)
        && (input[y][x - 1] == 'M' && input[y][x - 2] == 'A' && input[y][x - 3] == 'S')
    {
        found += 1;
    }

    // Check up -> down
    if (y + 3 < input.len())
        && (input[y + 1][x] == 'M' && input[y + 2][x] == 'A' && input[y + 3][x] == 'S')
    {
        found += 1;
    }

    // Check down -> up
    if (y as i32 - 3 >= 0)
        && (input[y - 1][x] == 'M' && input[y - 2][x] == 'A' && input[y - 3][x] == 'S')
    {
        found += 1;
    }

    // Check diagonally from top left to bottom right
    if (y + 3 < input.len() && x + 3 < input[y].len())
        && (input[y + 1][x + 1] == 'M' && input[y + 2][x + 2] == 'A' && input[y + 3][x + 3] == 'S')
    {
        found += 1;
    }

    // Check diagonally from bottom left to top right
    if (y as i32 - 3 >= 0 && x + 3 < input[y].len())
        && (input[y - 1][x + 1] == 'M' && input[y - 2][x + 2] == 'A' && input[y - 3][x + 3] == 'S')
    {
        found += 1;
    }

    // Check diagonally from top right to bottom left
    if (y + 3 < input.len() && x as i32 - 3 >= 0)
        && (input[y + 1][x - 1] == 'M' && input[y + 2][x - 2] == 'A' && input[y + 3][x - 3] == 'S')
    {
        found += 1;
    }

    // Check diagonally from bottom right to top left
    if (y as i32 - 3 >= 0 && x as i32 - 3 >= 0)
        && (input[y - 1][x - 1] == 'M' && input[y - 2][x - 2] == 'A' && input[y - 3][x - 3] == 'S')
    {
        found += 1;
    }

    found
}

fn check_for_x_mas(input: &[Vec<char>], x: usize, y: usize) -> bool {
    let mut found = false;

    if (x as i32 - 1 < 0 || x + 1 >= input[0].len()) || (y as i32 - 1 < 0 || y + 1 >= input.len()) {
        return false;
    }

    if is_one_m_and_one_s(input[y - 1][x - 1], input[y + 1][x + 1])
        && is_one_m_and_one_s(input[y - 1][x + 1], input[y + 1][x - 1])
    {
        found = true;
    }

    found
}

fn is_one_m_and_one_s(c1: char, c2: char) -> bool {
    (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        output.push(line.chars().collect::<Vec<char>>());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("sample-input.txt");
        let parsed_input = parse_input(&input);

        assert_eq!(
            parsed_input,
            vec![
                vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
                vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
                vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
                vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
                vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
                vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
                vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
                vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
                vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
                vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
            ]
        );
    }

    #[test]
    fn test_part1_sample_1() {
        let input = include_str!("sample-input.txt");
        let parsed_input = parse_input(&input);

        assert_eq!(solve(&parsed_input).0, 18);
    }

    #[test]
    fn test_part1_sample_2() {
        let input = include_str!("sample-input-2.txt");
        let parsed_input = parse_input(&input);

        assert_eq!(solve(&parsed_input).0, 4);
    }

    #[test]
    fn test_part2_sample_1() {
        let input = include_str!("sample-input-part2-1.txt");
        let parsed_input = parse_input(&input);

        assert_eq!(solve(&parsed_input).1, 1);
    }

    #[test]
    fn test_part2_sample_2() {
        let input = include_str!("sample-input-part2-2.txt");
        let parsed_input = parse_input(input);

        assert_eq!(solve(&parsed_input).1, 9);
    }
}
