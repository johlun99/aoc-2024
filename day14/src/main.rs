#[derive(Clone, Debug, PartialEq)]
struct Guard {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Guard {
    fn take_step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        /*
                if self.x >= WIDTH {
                    self.x -= WIDTH;
                } else if self.y >= HEIGHT {
                    self.y -= HEIGHT;
                } else if self.x < 0 {
                    self.x += WIDTH;
                } else if self.y < 0 {
                    self.y += HEIGHT;
                }
        */

        if self.x >= WIDTH {
            self.x -= WIDTH;
        }
        if self.y >= HEIGHT {
            self.y -= HEIGHT;
        }
        if self.x < 0 {
            self.x += WIDTH;
        }
        if self.y < 0 {
            self.y += HEIGHT;
        }
    }
}

#[cfg(not(test))]
const WIDTH: i64 = 101;
#[cfg(not(test))]
const HEIGHT: i64 = 103;

#[cfg(test)]
const WIDTH: i64 = 11;
#[cfg(test)]
const HEIGHT: i64 = 7;

fn main() {
    let input = include_str!("input.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(input.clone(), 100));
}

fn part1(mut guards: Vec<Guard>, seconds: u32) -> u64 {
    for _s in 0..seconds {
        for g in &mut guards {
            g.take_step();
        }
    }

    let q_guard = get_guards_in_quadrents(&guards);

    let mut answer = 0;

    for g in q_guard {
        if answer == 0 {
            answer = g;
            continue;
        }

        answer *= g;
    }

    answer
}

fn get_guards_in_quadrents(guards: &Vec<Guard>) -> Vec<u64> {
    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;

    let mut count_q1 = 0;
    let mut count_q2 = 0;
    let mut count_q3 = 0;
    let mut count_q4 = 0;

    for g in guards {
        if g.x < mid_x && g.y < mid_y {
            count_q1 += 1;
        } else if g.x > mid_x && g.y < mid_y {
            count_q2 += 1;
        } else if g.x < mid_x && g.y > mid_y {
            println!("Q4 X: {} Y: {}", g.x, g.y);
            count_q3 += 1;
        } else if g.x > mid_x && g.y > mid_y {
            count_q4 += 1;
        } else {
            println!("Found X: {} Y: {}", g.x, g.y);
        }
    }

    vec![count_q1, count_q2, count_q3, count_q4]
}

fn parse_input(input: &str) -> Vec<Guard> {
    let mut guards: Vec<Guard> = Vec::new();

    for line in input.lines() {
        let parts = line.split_once(' ');

        match parts {
            Some((first, second)) => {
                let left = first
                    .split_once('=')
                    .expect("Invalid Position")
                    .1
                    .split_once(',')
                    .expect("Invalid position coordinates");

                let (x, y) = (
                    left.0
                        .parse::<i64>()
                        .expect("Invalid number for x-coordinate"),
                    left.1
                        .parse::<i64>()
                        .expect("Invalid number for y-coordinate"),
                );

                let right = second
                    .split_once('=')
                    .expect("Invalid Position")
                    .1
                    .split_once(',')
                    .expect("Invalid position coordinates");

                let (vx, vy) = (
                    right
                        .0
                        .parse::<i64>()
                        .expect("Invalid number for x-velocity"),
                    right
                        .1
                        .parse::<i64>()
                        .expect("Invalid number for y-velocity"),
                );

                guards.push(Guard { x, y, vx, vy });
            }
            None => {
                panic!("Invalid position!");
            }
        }
    }

    guards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("short-sample-input.txt");

        assert_eq!(
            parse_input(input),
            vec![
                Guard {
                    x: 0,
                    y: 4,
                    vx: 3,
                    vy: -3
                },
                Guard {
                    x: 6,
                    y: 3,
                    vx: -1,
                    vy: -3
                },
                Guard {
                    x: 10,
                    y: 3,
                    vx: -1,
                    vy: 2
                }
            ]
        );
    }

    /*
    #[test]
    fn test_mini() {
        let input = include_str!("min-sample-input.txt");
        let input = parse_input(input);

        assert_eq!(part1(input, 5), 2);
    }
    */

    #[test]
    fn test_part1() {
        let input = include_str!("sample-input.txt");
        let input = parse_input(input);

        dbg!(WIDTH);
        dbg!(HEIGHT);

        assert_eq!(part1(input.clone(), 100), 12);
    }
}
