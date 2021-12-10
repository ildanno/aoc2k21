use std::cmp::{max, min};
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
        .map(|i| fuel_cost_constant(&data, i))
        .min()
        .expect("Empty")
}

fn solve_part_2(data: &Vec<i64>) -> i64 {
    let &min = data.iter().min().expect("Empty");
    let &max = data.iter().max().expect("Empty");

    (min..=max)
        .map(|i| fuel_cost_linear(&data, i))
        .min()
        .expect("Empty")
}

fn fuel_cost_constant(data: &Vec<i64>, position: i64) -> i64 {
    data.iter().fold(0, |c, &x| c + (position-x).abs())
}

fn fuel_cost_linear(data: &Vec<i64>, position_end: i64) -> i64 {
    data
        .iter()
        .fold(0, |carry_cost, &position_start| {
            let range = (min(position_start, position_end)..max(position_start, position_end));
            let cost_move_grab = range.fold((1, 0), |a, _y| (a.0 + 1, a.1 + a.0)).1;
            carry_cost + cost_move_grab
        })
}

mod tests {
    use crate::day_07::{fuel_cost_constant, fuel_cost_linear, parse_input, solve_part_1, solve_part_2};
    use crate::input;

    fn get_input() -> Vec<i64> {
        vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    }

    #[test]
    fn test_fuel_cost_constant() {
        let input = get_input();
        assert_eq!(fuel_cost_constant(&input, 2), 37);
        assert_eq!(fuel_cost_constant(&input, 1), 41);
        assert_eq!(fuel_cost_constant(&input, 3), 39);
        assert_eq!(fuel_cost_constant(&input, 10), 71);
    }

    #[test]
    fn test_fuel_cost_linear() {
        let input = get_input();
        assert_eq!(fuel_cost_linear(&input, 5), 168);
        assert_eq!(fuel_cost_linear(&input, 2), 206);
    }

    #[test]
    fn test_solve_part_1() {
        let input = get_input();
        assert_eq!(solve_part_1(&input), 37);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input();
        assert_eq!(solve_part_2(&input), 168);
    }

    #[test]
    fn test_golden_master() {
        let input = input::read("data/07.txt");
        let data = parse_input(&input);
        assert_eq!(solve_part_1(&data), 344535);
        assert_eq!(solve_part_2(&data), 95581659);
    }
}
