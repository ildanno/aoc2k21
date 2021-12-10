use crate::input;

pub fn print_solution() {
    let input = input::read("data/xx.txt");
    let data = parse_input(&input);

    println!("# Day xx");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn parse_input(input: &Vec<String>) -> Vec<i64> {
    input
        .iter()
        .map(|x| x.trim().parse().expect("Not an integer"))
        .collect::<Vec<i64>>()
}

fn solve_part_1(data: &Vec<i64>) -> i64 {
    0
}

fn solve_part_2(data: &Vec<i64>) -> i64 {
    0
}

mod tests {
    use crate::day_xx::{parse_input, solve_part_1, solve_part_2};
    use crate::input;

    fn get_input() -> Vec<i64> {
        vec![]
    }

    #[test]
    fn test_solve_part_1() {
        let input = get_input();
        assert_eq!(solve_part_1(&input), 1);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input();
        assert_eq!(solve_part_2(&input), 1);
    }

    #[test]
    fn test_golden_master() {
        let input = input::read("data/xx.txt");
        let data = parse_input(&input);
        assert_eq!(solve_part_1(&data), 1);
        assert_eq!(solve_part_2(&data), 1);
    }
}
