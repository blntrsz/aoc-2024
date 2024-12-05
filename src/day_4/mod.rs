pub mod day_4 {
    pub fn _part_1(lines: Vec<String>) -> i32 {
        let matrix: Vec<Vec<String>> = lines
            .iter()
            .map(|line| {
                line.split("")
                    .filter(|x| x.to_string() != "")
                    .map(|x| x.to_string())
                    .collect()
            })
            .collect();
        let mut sum = 0;

        for (y, y_value) in matrix.iter().enumerate() {
            for (x, x_value) in y_value.iter().enumerate() {
                if x_value.to_string() != "X" {
                    continue;
                }

                sum += _calculate_horizontal(y_value.clone(), x, 1);
                sum += _calculate_horizontal(y_value.clone(), x, -1);
                sum += _calculate_vertical(matrix.clone(), x, y, 1);
                sum += _calculate_vertical(matrix.clone(), x, y, -1);

                sum += _calculate_diagonal(matrix.clone(), x, y, 1, 1);
                sum += _calculate_diagonal(matrix.clone(), x, y, -1, -1);
                sum += _calculate_diagonal(matrix.clone(), x, y, -1, 1);
                sum += _calculate_diagonal(matrix.clone(), x, y, 1, -1);
            }
        }

        return sum;
    }

    pub fn _calculate_horizontal(y_value: Vec<String>, x: usize, directon: i32) -> i32 {
        let m = y_value
            .get(((x as i32) + (1 * directon)) as usize)
            .map_or("WRONG".to_string(), |v| v.to_string());
        let a = y_value
            .get(((x as i32) + (2 * directon)) as usize)
            .map_or("WRONG".to_string(), |v| v.to_string());
        let s = y_value
            .get(((x as i32) + (3 * directon)) as usize)
            .map_or("WRONG".to_string(), |v| v.to_string());

        let mas = m + &a + &s;
        if mas == "MAS" {
            return 1;
        }

        return 0;
    }

    pub fn _calculate_vertical(matrix: Vec<Vec<String>>, x: usize, y: usize, directon: i32) -> i32 {
        let m = matrix
            .get(((y as i32) + (1 * directon)) as usize)
            .unwrap_or(&vec![])
            .get(x)
            .unwrap_or(&"WRONG".to_string())
            .clone();
        let a = matrix
            .get(((y as i32) + (2 * directon)) as usize)
            .unwrap_or(&vec![])
            .get(x)
            .unwrap_or(&"WRONG".to_string())
            .clone();
        let s = matrix
            .get(((y as i32) + (3 * directon)) as usize)
            .unwrap_or(&vec![])
            .get(x)
            .unwrap_or(&"WRONG".to_string())
            .clone();

        let mas = m + &a + &s;
        if mas == "MAS" {
            return 1;
        }

        return 0;
    }

    pub fn _calculate_diagonal(
        matrix: Vec<Vec<String>>,
        x: usize,
        y: usize,
        x_directon: i32,
        y_directon: i32,
    ) -> i32 {
        let m = matrix
            .get(((y as i32) + (1 * y_directon)) as usize)
            .unwrap_or(&vec![])
            .get(((x as i32) + (1 * x_directon)) as usize)
            .unwrap_or(&"WRONG".to_string())
            .clone();
        let a = matrix
            .get(((y as i32) + (2 * y_directon)) as usize)
            .unwrap_or(&vec![])
            .get(((x as i32) + (2 * x_directon)) as usize)
            .unwrap_or(&"WRONG".to_string())
            .clone();
        let s = matrix
            .get(((y as i32) + (3 * y_directon)) as usize)
            .unwrap_or(&vec![])
            .get(((x as i32) + (3 * x_directon)) as usize)
            .unwrap_or(&"WRONG".to_string())
            .clone();

        let mas = m + &a + &s;
        if mas == "MAS" {
            return 1;
        }

        return 0;
    }

    pub fn _part_2(lines: Vec<String>) -> i32 {
        let matrix: Vec<Vec<String>> = lines
            .iter()
            .map(|line| {
                line.split("")
                    .filter(|x| x.to_string() != "")
                    .map(|x| x.to_string())
                    .collect()
            })
            .collect();
        let mut sum = 0;

        for (y, y_value) in matrix.iter().enumerate() {
            for (x, x_value) in y_value.iter().enumerate() {
                if x_value.to_string() != "A" {
                    continue;
                }

                let one_one = matrix
                    .get(((y as i32) + 1) as usize)
                    .unwrap_or(&vec![])
                    .get(((x as i32) + 1) as usize)
                    .unwrap_or(&"WRONG".to_string())
                    .clone();
                let mone_mone = matrix
                    .get(((y as i32) - 1) as usize)
                    .unwrap_or(&vec![])
                    .get(((x as i32) - 1) as usize)
                    .unwrap_or(&"WRONG".to_string())
                    .clone();
                let mone_one = matrix
                    .get(((y as i32) - 1) as usize)
                    .unwrap_or(&vec![])
                    .get(((x as i32) + 1) as usize)
                    .unwrap_or(&"WRONG".to_string())
                    .clone();
                let one_mone = matrix
                    .get(((y as i32) + 1) as usize)
                    .unwrap_or(&vec![])
                    .get(((x as i32) - 1) as usize)
                    .unwrap_or(&"WRONG".to_string())
                    .clone();

                let map = vec![one_one.clone(), mone_mone.clone(), mone_one, one_mone];

                let m: Vec<&String> = map.iter().filter(|c| c.to_string() == "M").collect();
                let s: Vec<&String> = map.iter().filter(|c| c.to_string() == "S").collect();

                if m.len() == 2 && s.len() == 2 && one_one != mone_mone {
                    sum += 1;
                }
            }
        }

        return sum;
    }
}
