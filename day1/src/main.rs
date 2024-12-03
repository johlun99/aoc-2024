fn main() {
    let input = include_str!("input.txt");
    
    use std::time::Instant;
    let now = Instant::now();
    
    println!("Part 1: {}", part1(input.to_string()));
    println!("Elapsed: {:.2?}", now.elapsed());
    
    let now = Instant::now();
    
    println!("Part 2: {}", part2(input.to_string()));
    println!("Elapsed: {:.2?}", now.elapsed());
}

fn part1(input: String) -> String {
    let (mut left, mut right) = parse_input(input);
    
    left.sort();
    right.sort();
    
    let mut dis: Vec<u32> = Vec::new();

    for i in 0..left.len() {
        dis.push(left[i].abs_diff(right[i]));
    }
    
    dis.iter().sum::<u32>().to_string()
}

fn part2(input: String) -> String {
    let (left, right) = parse_input(input);
    let mut score: Vec<u32> = Vec::new();

    for &value in left.iter() {
        let times = right.iter()
            .filter(|&&x| x == value)
            .count() as u32;
        score.push(value * times);
    }

    score.iter().sum::<u32>().to_string()
}

fn parse_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    
    input.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        
        left.push(parts[0].parse::<u32>().unwrap());
        right.push(parts[1].parse::<u32>().unwrap());
    });

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt");
        assert_eq!(part1(String::from(input)), 11.to_string());
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("sample-input.txt");
        assert_eq!(part2(String::from(input)), 31.to_string());
    }
}