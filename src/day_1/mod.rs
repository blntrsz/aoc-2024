pub mod day_1 {
    use std::collections::HashMap;

    pub fn _part_1(lines: Vec<String>) -> i32 {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        let lines_length = lines.len();

        for line in lines {
            let locations: Vec<&str> = line.split("   ").collect();
            if let Ok(left_num) = locations[0].parse::<i32>() {
                left.push(left_num);
            }
            if let Ok(right_num) = locations[1].parse::<i32>() {
                right.push(right_num);
            }
        }

        let mut sum = 0;

        println!("{}", lines_length);
        for _ in 0..lines_length {
            let mut left_min_value = 0;

            if let Some((min_index, _)) = left.iter().enumerate().min_by_key(|&(_, value)| value) {
                left_min_value = left.remove(min_index);
                println!("Removed minimum value: {}", left_min_value);
            }

            let mut right_min_value = 0;

            if let Some((min_index, _)) = right.iter().enumerate().min_by_key(|&(_, value)| value) {
                right_min_value = right.remove(min_index);
                println!("Removed minimum value: {}", right_min_value);
            }

            sum += (left_min_value - right_min_value).abs();
        }

        return sum;
    }

    pub fn part_2(lines: Vec<String>) -> i32 {
        let mut left: Vec<i32> = vec![];
        let mut right: HashMap<i32, i32> = HashMap::new();

        for line in lines {
            let locations: Vec<&str> = line.split("   ").collect();
            if let Ok(left_num) = locations[0].parse::<i32>() {
                left.push(left_num);
            }
            if let Ok(right_num) = locations[1].parse::<i32>() {
                *right.entry(right_num).or_insert(0) += 1;
            }
        }

        let mut sum = 0;

        for value in left {
            let count = right.get(&value).unwrap_or(&0);

            sum += value * count;
        }

        return sum;
    }
}
