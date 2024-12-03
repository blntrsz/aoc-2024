pub mod day_2 {
    pub fn _part_1(lines: Vec<String>) -> i32 {
        let mut sum = 0;

        for line in lines {
            let levels: Vec<&str> = line.split(" ").collect();

            let mut previous_level = 0;
            let mut is_safe = true;
            let mut is_increasing = true;

            let levels_copy = levels.clone();

            for level in levels {
                if previous_level == 0 {
                    if let Ok(level) = level.parse::<i32>() {
                        previous_level = level;
                        if let Ok(next_level) = levels_copy[1].parse::<i32>() {
                            if level - next_level > 0 {
                                is_increasing = false
                            }
                        }
                        continue;
                    }
                }

                if let Ok(level) = level.parse::<i32>() {
                    let diff = if is_increasing {
                        level - previous_level
                    } else {
                        previous_level - level
                    };
                    previous_level = level;

                    if diff > 3 || diff < 1 {
                        is_safe = false;
                        break;
                    }

                    continue;
                }
            }

            if is_safe {
                sum += 1;
            }
        }
        return sum;
    }

    fn _parse_levels(levels: Vec<&str>) -> i32 {
        let mut previous_level = 0;
        let mut is_safe = true;
        let mut is_increasing = true;

        let levels_copy = levels.clone();

        for level in levels {
            if previous_level == 0 {
                if let Ok(level) = level.parse::<i32>() {
                    previous_level = level;
                    if let Ok(next_level) = levels_copy[1].parse::<i32>() {
                        if level - next_level > 0 {
                            is_increasing = false
                        }
                    }
                    continue;
                }
            }

            if let Ok(level) = level.parse::<i32>() {
                let diff = if is_increasing {
                    level - previous_level
                } else {
                    previous_level - level
                };
                previous_level = level;

                if diff > 3 || diff < 1 {
                    is_safe = false;
                    break;
                }

                continue;
            }
        }

        if is_safe {
            return 1;
        }

        return 0;
    }

    pub fn _part_2(lines: Vec<String>) -> i32 {
        let mut sum = 0;

        for line in lines {
            let levels: Vec<&str> = line.split(" ").collect();
            let mut line_sum = 0;

            line_sum += _parse_levels(levels.clone());

            for index in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(index);

                line_sum += _parse_levels(new_levels);
            }

            if line_sum > 0 {
                sum += 1;
            }
        }

        return sum;
    }
}
