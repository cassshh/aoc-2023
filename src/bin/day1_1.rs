use regex::Regex;

fn main() {
    // let str = include_str!("./input/day1.test.txt");
    let str = include_str!("./input/day1.txt");

    let regex = Regex::new(r"[0-9]").unwrap();

    let calibration: i32 = str
        .lines()
        .map(|cal| {
            let matches: Vec<_> = regex.find_iter(cal).map(|m| m.as_str()).collect();
            let value = format!("{}{}", matches.first().unwrap(), matches.last().unwrap());
            value.parse::<i32>().unwrap()
        })
        .sum();

    println!("Sum of all calibrations: {:?}", calibration);
}
