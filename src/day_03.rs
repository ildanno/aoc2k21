use std::cmp::Ordering;
use crate::input;

pub fn print_solution() {
    // let data = input::read("data/03.txt");

    println!("# Day 3");
    // println!("Part 1: {}", solve_part_1(&data));
    // println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(report: &Vec<&str>) -> i32 {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..report[0].len() {
        let sum = report
            .iter()
            .map(|&x| x.get(i..i+1).expect("Out of range"))
            .map(|x| x.parse::<u8>().expect("Not a valid digit"))
            .sum::<u8>() as f64;

        gamma.push_str(match sum > (report.len() / 2) as f64 {
            true => { "1" }
            false => { "0" }
        });

        epsilon.push_str(match sum > (report.len() / 2) as f64 {
            true => { "0" }
            false => { "1" }
        });
    }

    binstr_to_int(&gamma[..]) * binstr_to_int(&epsilon[..])
}

// fn solve_part_2(p0: &_) -> _ {
//     todo_!()
// }

fn binstr_to_int(bin: &str) -> i32 {
    i32::from_str_radix(bin, 2).expect("Not a binary number!")
}

mod tests {
    use crate::day_03::{binstr_to_int, solve_part_1};

    #[test]
    fn test_binstr_to_int() {
        assert_eq!(binstr_to_int("10110"), 22);
        assert_eq!(binstr_to_int("01001"), 9);
    }

    #[test]
    fn test_solve_part_1() {
        let report = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];

        assert_eq!(solve_part_1(&report), 198)
    }
}
