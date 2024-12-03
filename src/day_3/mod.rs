pub mod day_3 {
    use regex::Regex;

    pub fn _part_1(line: String) -> i32 {
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let mut sum = 0;

        for mat in re.find_iter(&line) {
            let clean_input = mat.as_str().replace("mul(", "").replace(")", "");
            println!("{:?}", mat);
            let result: Vec<&str> = clean_input.split(",").collect();

            println!("{}", result[0]);

            let first = result[0].parse::<i32>().unwrap();
            let second = result[1].parse::<i32>().unwrap();

            sum += first * second;
        }

        return sum;
    }

    pub fn part_2(line: String) -> i32 {
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let mut sum = 0;

        for mat in re.find_iter(&line) {
            let clean_input = mat.as_str().replace("mul(", "").replace(")", "");
            let result: Vec<&str> = clean_input.split(",").collect();

            let first = result[0].parse::<i32>().unwrap();
            let second = result[1].parse::<i32>().unwrap();

            sum += first * second;
        }

        let dont_re = Regex::new(r"don't\(\).*?(do\(\)|\n)").unwrap();

        for dont_mat in dont_re.find_iter(&line) {
            let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
            println!("{:?}", dont_mat.as_str());

            for mat in re.find_iter(&dont_mat.as_str()) {
                let clean_input = mat.as_str().replace("mul(", "").replace(")", "");
                let result: Vec<&str> = clean_input.split(",").collect();

                let first = result[0].parse::<i32>().unwrap();
                let second = result[1].parse::<i32>().unwrap();

                sum = sum - (first * second);
            }
        }

        return sum;
    }
}
