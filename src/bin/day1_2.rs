use regex::Regex;

fn main() {
    // let str = include_str!("./input/day1_2.test.txt");
    let str = include_str!("./input/day1.txt");

    let regex = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let calibration: i32 = str
        .lines()
        .map(|cal| {
            let mut values: Vec<i32> = vec![];

            // iterate over each index of cal
            for i in 0..cal.len() {
                let found = regex.find_at(cal, i);
                if let Some(mat) = found {
                    let y = match mat.as_str() {
                        "one" => "1".to_string(),
                        "two" => "2".to_string(),
                        "three" => "3".to_string(),
                        "four" => "4".to_string(),
                        "five" => "5".to_string(),
                        "six" => "6".to_string(),
                        "seven" => "7".to_string(),
                        "eight" => "8".to_string(),
                        "nine" => "9".to_string(),
                        _ => mat.as_str().to_owned(),
                    };

                    values.push(y.parse::<i32>().unwrap());
                }
            }

            let value = format!(
                "{}{}",
                values.first().unwrap(),
                values.last().unwrap()
            );

            value.parse::<i32>().unwrap()
        })
        .sum();

    println!("Sum of all calibrations: {:?}", calibration);
}
