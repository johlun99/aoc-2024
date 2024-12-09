fn main() {
    let input = include_str!("input.txt");
    let mut input = parse_input(input);

    println!("Part 1: {}", part1(&mut input));
}

fn part1(input: &mut Vec<String>) -> u64 {
    println!("Initial input: {:?}", input);
    rearrange_memory(input);
    println!("Rearanged: {:?}", input);

    let mut answer: u64 = 0;

    for i in 0..input.len() {
        match input[i].parse::<u64>() {
            Ok(num) => answer += i as u64 * num,
            Err(_) => break
        }
    }

    answer
}

fn rearrange_memory(mem: &mut Vec<String>) {
    let mut left = 0;
    let mut right = mem.len() - 1;

    while left < right {
        while left < mem.len() && mem[left] != "." {
            left += 1;
        }

        while right > 0 && mem[right] == "." {
            right -= 1;
        }

        if left < right {
            mem.swap(left, right);
        }
    }
}

fn parse_input(input: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    let nums: Vec<u32> = input.chars().filter_map(|ch| ch.to_digit(10)).collect();
    let mut mem_index: u32 = 0;

    for i in 0..nums.len() {
        if i % 2 == 0 {
            for _ in 0..nums[i] {
                output.push(mem_index.to_string());
            }

            mem_index += 1;

        } else {
            for _ in 0..nums[i] {
                output.push(".".to_string());
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "12345";
        let parsed_input = parse_input(input);

        assert_eq!(parsed_input, vec!["0", ".", ".", "1", "1", "1", ".", ".", ".", ".", "2", "2", "2", "2", "2"]);
    }

    #[test]
    fn test_rearrange_mem() {
        let mut input = vec!["0", ".", ".", "1", "1", "1", ".", ".", ".", ".", "2", "2", "2", "2", "2"];
        let mut input = input.iter().map(|&s| s.to_string()).collect();

        rearrange_memory(&mut input);

        assert_eq!(input, 
            vec!["0", "2", "2", "1", "1", "1", "2", "2", "2", ".", ".", ".", ".", ".", "."]
        );
    }

    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt");
        let input = parse_input(input);

        assert_eq!(part1(&mut input.clone()), 1928);
    }
    //2333133121414131402
}
