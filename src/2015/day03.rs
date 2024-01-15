use advent_of_code::utils;
use std::collections::HashSet;

struct Santa {
    position: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Santa {
    fn new() -> Self {
        let position = (0, 0);
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        visited.insert(position);

        Self { position, visited }
    }

    fn make_move(&mut self, direction: &char) {
        match direction {
            '>' => {
                self.position.0 += 1;
            }
            '<' => {
                self.position.0 -= 1;
            }
            '^' => {
                self.position.1 += 1;
            }
            'v' => {
                self.position.1 -= 1;
            }
            _ => panic!("Invalid direction"),
        }
        self.visited.insert(self.position);
    }
}

fn part1(data: &str) -> usize {
    let mut santa = Santa::new();

    for direction in data.chars() {
        santa.make_move(&direction)
    }

    santa.visited.len()
}

fn part2(data: &str) -> usize {
    let mut santa = Santa::new();
    let mut robo_santa = Santa::new();

    for (i, direction) in data.char_indices() {
        if i % 2 == 0 {
            santa.make_move(&direction);
        } else {
            robo_santa.make_move(&direction);
        }
    }

    let union: HashSet<(i32, i32)> = santa.visited.union(&robo_santa.visited).cloned().collect();

    union.len()
}

pub fn solve() {
    let data = utils::read_input_file("2015", "03");

    utils::display_result(part1(&data), part2(&data))
}
