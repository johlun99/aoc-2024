use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let (rules, page_numbers) = parse_input(input);

    match part1(rules.clone(), page_numbers.clone()) {
        Ok(ans) => println!("Part 1: {}", ans),
        Err(msg) => println!("Err on p1: {}", msg),
    }

    match part2(rules.clone(), page_numbers.clone()) {
        Ok(ans) => println!("Part 2: {}", ans),
        Err(msg) => println!("Err on p2: {}", msg),
    }
}

fn part1(rules: HashMap<u32, Vec<u32>>, page_numbers: Vec<Vec<u32>>) -> Result<u32, String> {
    let mut valid_pages: Vec<Vec<u32>> = Vec::new();

    for pages in page_numbers {
        let mut valid = true;

        for i in 0..pages.len() {
            if let Some(rule) = rules.get(&pages[i]) {
                for j in (0..i).rev() {
                    if rule.contains(&pages[j]) {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if valid {
            valid_pages.push(pages);
        }
    }

    let mut middles: Vec<u32> = Vec::new();

    for v in valid_pages {
        middles.push(v[v.len() / 2]);
    }

    Ok(middles.iter().sum())
}

fn part2(rules: HashMap<u32, Vec<u32>>, page_numbers: Vec<Vec<u32>>) -> Result<u32, String> {
    let mut invalid_pages: Vec<Vec<u32>> = Vec::new();

    for pages in page_numbers {
        let mut valid = true;

        for i in 0..pages.len() {
            if let Some(rule) = rules.get(&pages[i]) {
                for j in (0..i).rev() {
                    if rule.contains(&pages[j]) {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if !valid {
            invalid_pages.push(pages.to_vec());
        }
    }

    let mut middles: Vec<u32> = Vec::new();
    for mut pages in invalid_pages {
        rearrange_pages(&mut pages, &rules);
        middles.push(pages[pages.len() / 2]);
    }

    Ok(middles.iter().sum())
}

fn rearrange_pages(page_numbers: &mut [u32], rules: &HashMap<u32, Vec<u32>>) {
    let mut i = 0;
    while i < page_numbers.len() {
        if let Some(rule) = rules.get(&page_numbers[i]) {
            for j in 0..i {
                if rule.contains(&page_numbers[j]) {
                    page_numbers.swap(i, j);
                    i = 0;
                    break;
                }
            }
        }

        i += 1;
    }
}

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut page_numbers: Vec<Vec<u32>> = Vec::new();

    let mut parts = input.split("\n\n");

    for line in parts.next().expect("Invalid rules..").lines() {
        let (left, right) = line.split_once('|').unwrap();

        rules
            .entry(left.parse::<u32>().expect("invalid left..."))
            .and_modify(|v| v.push(right.parse::<u32>().expect("Invalid right...")))
            .or_insert(vec![right.parse::<u32>().expect("invalid right...")]);
    }

    for line in parts.next().expect("Invalid page_numbers...").lines() {
        page_numbers.push(
            line.split(',')
                .map(|n| n.parse::<u32>().expect("Invalid page..."))
                .collect::<Vec<u32>>(),
        );
    }

    (rules, page_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("short-sample-input.txt");
        let input = parse_input(input);
        let expected_output = (
            HashMap::from([(47, vec![53]), (97, vec![13])]),
            vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]],
        );

        assert_eq!(input, expected_output);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt");
        let (rules, page_numbers) = parse_input(input);

        assert_eq!(part1(rules, page_numbers), Ok(143));
    }

    #[test]
    fn test_rearrange_pages() {
        let input = include_str!("sample-input.txt");
        let (rules, _) = parse_input(input);
        let mut pages = vec![75, 97, 47, 61, 53];
        rearrange_pages(&mut pages, &rules);

        assert_eq!(pages, vec![97, 75, 47, 61, 53]);

        let mut pages = vec![61, 13, 29];
        rearrange_pages(&mut pages, &rules);
        assert_eq!(pages, vec![61, 29, 13]);

        let mut pages = vec![97, 13, 75, 29, 47];
        rearrange_pages(&mut pages, &rules);
        assert_eq!(pages, vec![97, 75, 47, 29, 13]);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("sample-input.txt");
        let (rules, page_numbers) = parse_input(input);

        assert_eq!(part2(rules, page_numbers), Ok(123));
    }
}
