use std::collections::HashMap;

fn get_number(line: &str) -> usize {
    let mut sums_holder = vec![];

    let numeric_map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for (idx, ci) in line.chars().enumerate() {
        if ci.is_numeric() {
            sums_holder.push(ci);
        } else {
            let mut current_sub_str = String::new();

            for cy in line[idx..].chars() {
                current_sub_str = format!("{}{}", current_sub_str, cy);

                if let Some(num) = numeric_map.get(current_sub_str.as_str()) {
                    sums_holder.push(*num);
                    current_sub_str.clear();
                }
            }
        }
    }

    let result_str = format!(
        "{}{}",
        sums_holder.first().expect("no first"),
        sums_holder.last().expect("no last")
    );

    result_str.parse::<usize>().expect("parsing err")
}

fn sum_lines(lines: &str) -> Option<usize> {
    lines
        .lines()
        .map(|line| get_number(line))
        .reduce(|a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example_a() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        if let Some(v) = sum_lines(input) {
            println!("{v}");
        }
    }

    #[test]
    fn test_a() {
        let input = fs::read_to_string("./src/one/input_a.txt").unwrap();

        if let Some(v) = sum_lines(&input) {
            println!("{v}");
        }
    }

    #[test]
    fn test_example_b() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        if let Some(res) = sum_lines(input) {
            println!("{res}");
        }
    }

    #[test]
    fn text_b() {
        let input = fs::read_to_string("./src/one/input_b.txt").unwrap();

        if let Some(v) = sum_lines(&input) {
            println!("{v}");
        }
    }
}
