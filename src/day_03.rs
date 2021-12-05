use std::cmp::Ordering;
use crate::input;

pub fn print_solution() {
    let data = input::read("data/03.txt");

    println!("# Day 3");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(report: &Vec<String>) -> i32 {
    let mut gamma = String::new();

    for i in 0..report[0].len() {
        let digits = digits_at_position(report, i);

        let most_common_value = most_common_value(digits);

        gamma.push_str(most_common_value);
    }

    let mut epsilon = reverse_binary(&gamma);

    binstr_to_int(&gamma[..]) * binstr_to_int(&epsilon[..])
}

fn solve_part_2(report: &Vec<String>) -> i32 {
    let len = report[0].len();

    let co2_scrubber_rating = *(0..len)
        .into_iter()
        .fold(report.clone(), |list, i| {
            if list.len() == 1 { return list }
            let mcv = most_common_value(digits_at_position(&list, i));
            list
                .iter()
                .filter(|record| &record[i..i + 1] == mcv)
                .map(|x| String::from(x))
                .collect::<Vec<String>>()
        })
        .iter()
        .map(|x| binstr_to_int(&x[..]))
        .collect::<Vec<i32>>()
        .get(0)
        .expect("Empty list");

    let oxygen_generator_rating= *(0..len)
        .into_iter()
        .fold(report.clone(), |list, i| {
            if list.len() == 1 { return list }
            let mcv = less_common_value(digits_at_position(&list, i));
            list
                .iter()
                .filter(|record| &record[i..i + 1] == mcv)
                .map(|x| String::from(x))
                .collect::<Vec<String>>()
        })
        .iter()
        .map(|x| binstr_to_int(&x[..]))
        .collect::<Vec<i32>>()
        .get(0)
        .expect("Empty list");

    co2_scrubber_rating * oxygen_generator_rating
}

fn digits_at_position(report: &Vec<String>, i: usize) -> Vec<u32> {
    report
        .iter()
        .map(|x| { x.get(i..i + 1).expect("Out of range") })
        .map(|x| { x.parse::<u32>().expect("Not a valid digit") })
        .collect::<Vec<u32>>()
}

fn reverse_binary(number: &String) -> String {
    number
        .chars()
        .map(|x| match x { '0' => '1', _ => '0'})
        .collect::<String>()
}

fn most_common_value(digits: Vec<u32>) -> &'static str {
    criteria_common_value(digits, |sum, length| sum >= length)
}

fn less_common_value(digits: Vec<u32>) -> &'static str {
    criteria_common_value(digits, |sum, length| sum < length)
}

fn criteria_common_value(digits: Vec<u32>, strategy: fn(f64, f64) -> bool) -> &'static str {
    let sum = digits
        .iter()
        .sum::<u32>() as f64;

    match strategy(sum, (digits.len() as f64 / 2.0)) {
        true => { "1" }
        false => { "0" }
    }
}

fn binstr_to_int(bin: &str) -> i32 {
    i32::from_str_radix(bin, 2).expect("Not a binary number!")
}

mod tests {
    use crate::day_03::{binstr_to_int, less_common_value, most_common_value, reverse_binary, solve_part_1, solve_part_2};
    use crate::input;

    #[test]
    fn test_binstr_to_int() {
        assert_eq!(binstr_to_int("10110"), 22);
        assert_eq!(binstr_to_int("01001"), 9);
    }

    #[test]
    fn test_reverse_binary() {
        assert_eq!(reverse_binary(&String::from("10110")), String::from("01001"));
        assert_eq!(reverse_binary(&String::from("01001")), String::from("10110"));
    }

    #[test]
    fn test_solve_part_1() {
        let report = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        assert_eq!(solve_part_1(&report), 198)
    }

    #[test]
    fn test_solve_part_2() {
        let report = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        assert_eq!(solve_part_2(&report), 230)
    }

    #[test]
    fn test_golden_master() {
        let data = input::read("data/03.txt");
        assert_eq!(solve_part_1(&data), 2003336);
        assert_eq!(solve_part_2(&data), 1877139);
    }

    #[test]
    fn test_most_common_value() {
        assert_eq!(most_common_value(vec![0, 1]), "1");
        assert_eq!(most_common_value(vec![0, 1, 1]), "1");
        assert_eq!(most_common_value(vec![0, 0, 1, 1]), "1");
        assert_eq!(most_common_value(vec![0, 0, 0, 1, 1]), "0");
        assert_eq!(most_common_value(vec![0, 0, 0, 0, 1, 1]), "0");
    }

    #[test]
    fn test_less_common_value() {
        assert_eq!(less_common_value(vec![0, 1]), "0");
        assert_eq!(less_common_value(vec![0, 1, 1]), "0");
        assert_eq!(less_common_value(vec![0, 0, 1, 1]), "0");
        assert_eq!(less_common_value(vec![0, 0, 0, 1, 1]), "1");
        assert_eq!(less_common_value(vec![0, 0, 0, 0, 1, 1]), "1");
    }
}
