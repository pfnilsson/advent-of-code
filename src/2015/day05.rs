use advent_of_code::utils;
use std::collections::HashSet;

fn is_nice1(vowels: &HashSet<char>, string: &str) -> bool {
    let forbidden_strings = vec!["ab", "cd", "pq", "xy"];

    for forbidden_string in &forbidden_strings {
        if string.contains(forbidden_string) {
            return false;
        }
    }

    let mut vowel_count = 0;
    let mut prev = '_';
    let mut repeats = false;

    for letter in string.chars() {
        if vowels.contains(&letter) {
            vowel_count += 1;
        }

        if letter == prev {
            repeats = true;
        }

        prev = letter;
    }

    if vowel_count < 3 {
        return false;
    }

    if !repeats {
        return false;
    }

    true
}

fn substring_appears_twice(main_string: &str, substring: &str) -> bool {
    if let Some(first_occurrence) = main_string.find(substring) {
        if let Some(_) = main_string[first_occurrence + substring.len()..].find(substring) {
            return true;
        }
    }
    false
}

fn substrings_len_two(string: &str) -> Vec<&str> {
    string
        .chars()
        .zip(string.chars().skip(1))
        .map(|(first, second)| &string[first as usize..=second as usize])
        .collect()
}

fn is_nice2(string: &str) -> bool {
    let mut prev = '_';
    let mut prevprev = '_';
    let mut repeats = false;

    for letter in string.chars() {
        if letter == prevprev {
            repeats = true;
            break;
        }
        prevprev = prev;
        prev = letter;
    }

    if !repeats {
        return false;
    }

    let substrings = substrings_len_two(string);

    for substring in substrings {
        if substring_appears_twice(string, substring) {
            return true;
        }
    }

    false
}

fn part1(data: &str) -> u32 {
    let vowels_vec: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let vowels: HashSet<char> = vowels_vec.into_iter().collect();
    let mut nice_strings = 0;
    for string in data.lines() {
        if is_nice1(&vowels, string) {
            nice_strings += 1;
        }
    }
    nice_strings
}

fn part2(data: &str) -> u32 {
    let mut nice_strings = 0;
    for string in data.lines() {
        if is_nice2(string) {
            nice_strings += 1;
        }
    }
    nice_strings
}

pub fn solve() {
    let data = utils::read_input_file("2015", "05");

    utils::display_result(part1(&data), part2(&data))
}
