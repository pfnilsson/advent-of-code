mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    for day in 1..=5 {
        println!("Day {}", day);

        match day {
            1 => day01::solve(),
            2 => day02::solve(),
            3 => day03::solve(),
            4 => day04::solve(),
            5 => day05::solve(),
            _ => println!("Day {} solution not implemented yet.", day),
        }
    }
}
