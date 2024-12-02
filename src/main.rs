mod common;
mod day_1;
mod day_2;

fn main() {
    let file = common::common::read_file("src/day_2/input.txt");
    let lines = common::common::string_to_lines(file);

    let result = day_2::day_2::part_2(lines.clone());

    println!("The value of x is: {}", result);
}
