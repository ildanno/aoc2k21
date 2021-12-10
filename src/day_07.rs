use std::cmp::min;
use crate::input;

pub fn print_solution() {
    let input = input::read("data/07.txt");
    let data = parse_input(&input);

    println!("# Day 07");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn parse_input(input: &Vec<String>) -> Vec<i64> {
    input
        .get(0)
        .expect("Empty")
        .split(",")
        .map(|x| x.trim().parse().expect("Not an integer"))
        .collect::<Vec<i64>>()
}

fn solve_part_1(data: &Vec<i64>) -> i64 {
    let &min = data.iter().min().expect("Empty");
    let &max = data.iter().max().expect("Empty");

    (min..=max)
        .map(|i| fuel_cost(&data, i))
        .min()
        .expect("Empty")
}

fn solve_part_2(data: &Vec<i64>) -> i64 {
    0
}

fn fuel_cost(data: &Vec<i64>, position: i64) -> i64 {
    data.iter().fold(0, |c, &x| c + (position-x).abs())
}

mod tests {
    use crate::day_07::{fuel_cost, parse_input, solve_part_1, solve_part_2};
    use crate::input;

    fn get_input() -> Vec<i64> {
        vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    }

    #[test]
    fn test_fuel_cost() {
        let input = get_input();
        assert_eq!(fuel_cost(&input, 2), 37);
        assert_eq!(fuel_cost(&input, 1), 41);
        assert_eq!(fuel_cost(&input, 3), 39);
        assert_eq!(fuel_cost(&input, 10), 71);
    }

    #[test]
    fn test_solve_part_1() {
        let input = get_input();
        assert_eq!(solve_part_1(&input), 37);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input();
        assert_eq!(solve_part_2(&input), 1);
    }

    #[test]
    fn test_golden_master() {
        let input = input::read("data/07.txt");
        let data = parse_input(&input);
        assert_eq!(solve_part_1(&data), 344535);
        // assert_eq!(solve_part_2(&data), 1);
    }
}
