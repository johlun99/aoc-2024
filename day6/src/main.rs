use std::{char, clone, usize};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    // let input = include_str!("sample-input.txt");
    let input = include_str!("input.txt");
    let (position, map) = parse_input(input);

    let x = position.0;
    let y = position.1;

    println!("Part 1: {}", part1(&map, (x as i32, y as i32)));

    let mut count_loops = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            // print!("{}", x);
            let mut clone = map.clone();

            if clone[y][x] == '^' || clone[y][x] == '#' {
                continue;
            }

            /*
            println!("clone before:");
            for row in &clone {
                println!("{:?}", row);
            }
            */

            clone[y][x] = '#';

            /*
            println!("Clone after");
            for row in &clone {
                println!("{:?}", row);
            }
            */

            let res = part1(&clone, (x as i32, y as i32));

            // println!("Result: {}", res);
            // println!("Trying: x {} y {}", x, y);
            if res == -1 {
                count_loops += 1;
            }
        }

        // println!();
    }

    println!("Part 2: {}", count_loops);
}

fn part1(map: &[Vec<char>], position: (i32, i32)) -> i32 {
    let mut direction = Direction::Up;
    let mut steps = 0;
    let mut visited: Vec<(u32, u32)> = Vec::new();

    let mut x = 73;
    let mut y = 75;

    let mut visited_dir: Vec<((u32, u32), Direction)> = vec![((x as u32, y as u32), direction)];

    /*
    println!("This one!");
    for row in map {
        println!("{:?}", row);
    }
    */
    loop {
        match direction {
            Direction::Up => {
                if y > 0 && map[y as usize - 1][x as usize] == '#' {
                    direction = Direction::Right;
                } else {
                    y -= 1
                }
            }
            Direction::Down => {
                if (y as usize) < map.len() && map[y as usize + 1][x as usize] == '#' {
                    direction = Direction::Left;
                } else {
                    y += 1;
                }
            }
            Direction::Left => {
                if x > 0 && map[y as usize][x as usize - 1] == '#' {
                    direction = Direction::Up;
                } else {
                    x -= 1;
                }
            }
            Direction::Right => {
                if x as usize <= map[0].len() && map[y as usize][x as usize + 1] == '#' {
                    direction = Direction::Down;
                } else {
                    x += 1;
                }
            }
        }

        /*
        if x == 3 && y == 6 {
            println!("Debug data");
            println!("{:?}", visited_dir);
        }
        */

        if visited_dir.contains(&((x as u32, y as u32), direction)) {
            // println!("I'm here yaoooo!");
            return -1;
        }

        if x < 0 || y < 0 || x as usize >= map[0].len() - 1 || y as usize >= map.len() - 1 {
            // println!("X {} y {}", x, y);
            steps += 2;
            break;
        } else if !visited.contains(&(x as u32, y as u32)) {
            visited.push((x as u32, y as u32));
            visited_dir.push(((x as u32, y as u32), direction));
            steps += 1;
        }
    }

    // println!("{:?}", visited_dir);

    /*
    for (y, row) in map.into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            if *c != '^' && visited.contains(&(x as u32, y as u32)) {
                print!("X");
                continue;
            }

            print!("{}", c);
        }
        println!();
    }
    */

    steps
}

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let mut position = (0, 0);
    let mut output: Vec<Vec<char>> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut parsed_row: Vec<char> = Vec::new();

        for (x, c) in line.chars().enumerate() {
            parsed_row.push(c);

            if c == '^' {
                position.0 = x;
                position.1 = y;
            }
        }

        output.push(parsed_row);
    }

    (position, output)
}
