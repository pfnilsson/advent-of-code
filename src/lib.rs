pub mod utils {
    use std::fs;
    use std::path::PathBuf;

    pub fn read_input_file(year: &str, day: &str) -> String {
        let mut path = PathBuf::new();
        path.push("src");
        path.push(year);
        path.push("inputs");
        path.push(format!("day{}.txt", day));
        fs::read_to_string(path).unwrap()
    }

    pub fn display_result<T: std::fmt::Display>(part1: T, part2: T) {
        println!("  Part 1: {}", part1);
        println!("  Part 2: {}", part2);
    }
}
