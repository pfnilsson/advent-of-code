use advent_of_code::utils;

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(dimenstions_str: &str) -> Self {
        let dimensions: Vec<u32> = dimenstions_str
            .split('x')
            .map(|dim| dim.parse().unwrap())
            .collect();

        Self {
            length: dimensions[0],
            width: dimensions[1],
            height: dimensions[2],
        }
    }

    fn area(&self) -> u32 {
        2 * (self.length * self.width + self.width * self.height + self.height * self.length)
    }

    fn smallest_side(&self) -> u32 {
        let side1 = self.length * self.width;
        let side2 = self.width * self.height;
        let side3 = self.height * self.length;

        side1.min(side2).min(side3)
    }

    fn wrapping_paper_needed(&self) -> u32 {
        self.area() + self.smallest_side()
    }

    fn bow_ribbon_length(&self) -> u32 {
        self.length * self.height * self.width
    }

    fn wrap_ribbon_length(&self) -> u32 {
        let mut dimensions = vec![self.length, self.width, self.height];
        dimensions.sort();

        let shortest1 = dimensions[0];
        let shortest2 = dimensions[1];

        2 * shortest1 + 2 * shortest2
    }

    fn ribbon_needed(&self) -> u32 {
        self.bow_ribbon_length() + self.wrap_ribbon_length()
    }
}

fn parse_presents(presents_raw: &str) -> Vec<Present> {
    presents_raw
        .lines()
        .map(|line| Present::new(line))
        .collect()
}

pub fn solve() {
    let data = utils::read_input_file("2015", "02");
    let presents = parse_presents(&data);

    let mut total_wrapping_paper = 0;
    let mut total_ribbon = 0;

    for present in &presents {
        total_wrapping_paper += present.wrapping_paper_needed();
        total_ribbon += present.ribbon_needed();
    }

    utils::display_result(total_wrapping_paper, total_ribbon)
}
