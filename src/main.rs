mod common;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let file = common::common::read_file("src/day_4/input.txt");

    let lines = common::common::string_to_lines(file);

    let result = day_4::day_4::part_2(lines);

    println!("The value of x is: {}", result);
}
