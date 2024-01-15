use advent_of_code::utils;

use md5;

fn md5_hash(data: &[u8]) -> String {
    let hash = md5::compute(data);

    let mut hex_string = String::new();
    for byte in hash.iter() {
        hex_string.push_str(&format!("{:02x}", byte));
    }

    hex_string
}

fn find_with_start(start: &str, secret_key: &str) -> u32 {
    let mut i: u32 = 1;
    loop {
        let value = format!("{}{}", secret_key, i);
        let value_as_bytes = value.as_bytes();
        let hash = md5_hash(&value_as_bytes);

        if hash.starts_with(start) {
            return i;
        }

        i += 1;
    }
}

pub fn solve() {
    let secret_key = utils::read_input_file("2015", "04");

    utils::display_result(
        find_with_start("00000", &secret_key),
        find_with_start("000000", &secret_key),
    )
}
