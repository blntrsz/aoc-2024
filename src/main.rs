mod common;
mod day_1;

fn main() {
    let file = common::common::read_file("src/day_1/input.txt");
    let lines = common::common::string_to_lines(file);

    let result = day_1::day_1::part_2(lines.clone());

    println!("The value of x is: {}", result);
}
