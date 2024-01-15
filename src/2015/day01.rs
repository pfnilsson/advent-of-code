use advent_of_code::utils;

pub fn solve() {
    let data = utils::read_input_file("2015", "01");
    let mut floor = 0;
    let mut found = false;
    let mut basement_index = -1;

    for (i, character) in data.chars().enumerate() {
        match character {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid character"),
        }

        if floor == -1 && !found {
            basement_index = (i + 1) as i32;
            found = true;
        }
    }

    utils::display_result(floor, basement_index)
}
