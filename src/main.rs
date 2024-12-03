mod common;
mod day_1;
mod day_2;
mod day_3;

fn main() {
    let file = common::common::read_file("src/day_3/input.txt");

    let result = day_3::day_3::part_2(file);

    println!("The value of x is: {}", result);
}
