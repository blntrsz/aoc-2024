mod common;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    let rules = common::common::read_file("src/day_5/rules.txt");
    let updates = common::common::read_file("src/day_5/updates.txt");

    let rule_lines = common::common::string_to_lines(rules);
    let update_lines = common::common::string_to_lines(updates);

    let result = day_5::day_5::part_2(rule_lines, update_lines);

    println!("The value of x is: {}", result);
}
