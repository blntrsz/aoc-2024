pub mod day_5 {
    use std::cmp::Ordering;

    pub fn _part_1(rules: Vec<String>, updates: Vec<String>) -> i32 {
        let mut sum = 0;
        for update in updates {
            let mut is_valid = true;
            for rule in rules.clone() {
                let result = validate_rule(rule, update.to_string());
                if !result {
                    is_valid = false;
                }
            }

            if is_valid {
                let result = _get_middle_update(update);

                sum += result;
            }
        }

        return sum;
    }

    pub fn _get_middle_update(update: String) -> i32 {
        let update_items: Vec<i32> = update
            .split(",")
            .map(|u| u.parse::<i32>().unwrap())
            .collect();
        return update_items[update_items.len() / 2];
    }

    pub fn validate_rule(rule: String, update: String) -> bool {
        let digits: Vec<i32> = rule
            .split("|")
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect();
        let left = digits.get(0).unwrap();
        let right = digits.get(1).unwrap();

        let update_items: Vec<i32> = update
            .split(",")
            .map(|u| u.parse::<i32>().unwrap())
            .collect();

        let left_index = update_items
            .iter()
            .position(|&x| {
                return x == left.clone();
            })
            .unwrap_or(999);
        let right_index = update_items
            .iter()
            .position(|&x| x == right.clone())
            .unwrap_or(999);

        if left_index == 999 || right_index == 999 {
            return true;
        }

        return left_index < right_index;
    }

    pub fn part_2(rules: Vec<String>, updates: Vec<String>) -> i32 {
        let mut sum = 0;
        for update in updates {
            let mut is_valid = true;
            for rule in rules.clone() {
                let result = validate_rule(rule, update.to_string());
                if !result {
                    is_valid = false;
                }
            }

            if is_valid {
                continue;
            }

            let mut update_items: Vec<i32> = update
                .split(",")
                .map(|u| u.parse::<i32>().unwrap())
                .collect();

            update_items.sort_by(|a, b| {
                for rule in rules.clone() {
                    let digits: Vec<i32> = rule
                        .split("|")
                        .map(|c| c.to_string().parse::<i32>().unwrap())
                        .collect();
                    let left = digits.get(0).unwrap().clone();
                    let right = digits.get(1).unwrap().clone();

                    if left == *a && right == *b {
                        return Ordering::Greater;
                    }

                    if right == *a && left == *b {
                        return Ordering::Less;
                    }

                    continue;
                }

                return Ordering::Equal;
            });

            println!("{:?}", update_items);

            sum += update_items[update_items.len() / 2];
        }

        return sum;
    }
}
