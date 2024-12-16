use glam::IVec2;
use pathfinding::prelude::*;
use std::{char, collections::HashSet};

struct Map {
    start: IVec2,
    end: IVec2,
    walls: HashSet<IVec2>,
}

fn main() {
    let input = include_str!("input.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(map: &Map) -> u32 {
    let result = dijkstra(
        &(map.start, IVec2::X),
        |(position, direction)| {
            let next_position = position + direction;
            if map.walls.contains(&next_position) {
                vec![
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            } else {
                vec![
                    ((next_position, *direction), 1),
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            }
        },
        |&(pos, _)| pos == map.end,
    )
    .expect("A valid path");

    result.1
}

fn part2(map: &Map) -> u32 {
    let (result, _cost) = astar_bag(
        &(map.start, IVec2::X),
        |(position, direction)| {
            let next_position = position + direction;
            if map.walls.contains(&next_position) {
                vec![
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            } else {
                vec![
                    ((next_position, *direction), 1),
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            }
        },
        |_| 0,
        |&(pos, _)| pos == map.end,
    )
    .expect("A valid path");

    let set = result
        .flat_map(|path| path.into_iter().map(|(position, _)| position))
        .collect::<HashSet<IVec2>>();

    set.len() as u32
}

fn get_start(map: &str) -> Result<IVec2, String> {
    for (y, row) in map.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == 'S' {
                return Ok(IVec2::new(x as i32, y as i32));
            }
        }
    }

    Err("Invalid start".to_string())
}

fn get_end(map: &str) -> Result<IVec2, String> {
    for (y, row) in map.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == 'E' {
                return Ok(IVec2::new(x as i32, y as i32));
            }
        }
    }

    Err("Invalid end point".to_string())
}

fn get_walls(map: &str) -> HashSet<IVec2> {
    let mut walls: HashSet<IVec2> = HashSet::new();

    for (y, row) in map.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '#' {
                walls.insert(IVec2::new(x as i32, y as i32));
            }
        }
    }

    walls
}

fn parse_input(input: &str) -> Map {
    let mut parsed_input: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        parsed_input.push(line.chars().collect());
    }

    let start = get_start(input).expect("Invalid starting point");
    let end = get_end(input).expect("Invalid ending point");
    let walls = get_walls(input);

    Map { start, end, walls }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("sample1-input.txt");
        let map = parse_input(input);
        assert_eq!(map.start, IVec2::new(1, 13));
        assert_eq!(map.end, IVec2::new(13, 1));
        assert_eq!(map.walls.len(), 121);

        let input = include_str!("sample2-input.txt");
        let map = parse_input(input);
        assert_eq!(map.start, IVec2::new(1, 15));
        assert_eq!(map.end, IVec2::new(15, 1));
        assert_eq!(map.walls.len(), 157);
    }

    #[test]
    fn test_get_start() {
        let input = include_str!("sample1-input.txt");
        let start = get_start(input);
        assert_eq!(start, Ok(IVec2::new(1, 13)));

        let input = include_str!("sample2-input.txt");
        let start = get_start(input);
        assert_eq!(start, Ok(IVec2::new(1, 15)));
    }

    #[test]
    fn test_get_end() {
        let input = include_str!("sample1-input.txt");
        let end = get_end(input);
        assert_eq!(end, Ok(IVec2::new(13, 1)));

        let input = include_str!("sample2-input.txt");
        let end = get_end(input);
        assert_eq!(end, Ok(IVec2::new(15, 1)));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("sample1-input.txt");
        let input = parse_input(input);

        assert_eq!(part1(&input), 7036);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("sample1-input.txt");
        let map = parse_input(input);
        assert_eq!(part2(&map), 45);
    }
}
