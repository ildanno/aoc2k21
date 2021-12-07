use crate::input;

pub fn print_solution() {
    let data = parse_input(
        input::read("data/06.txt")
            .get(0)
            .expect("Empty")
    );

    println!("# Day 6");
    println!("Part 1: {}", solve(&data, 80));
    // println!("Part 2: {}", solve(&data, 256)); // warning: slow
}

fn parse_input(data: &String) -> Vec<u128> {
    data
        .split(",")
        .map(|x| x.trim().parse().expect("Not an integer"))
        .collect::<Vec<u128>>()
}

fn solve(data: &Vec<u128>, i: i32) -> u128 {
    (0..i)
        .fold(data.clone(), |x, n| {
            let mut new_status = x.iter()
                .map(|&x| { if x == 0 { 6 } else { x - 1 } })
                .collect::<Vec<u128>>();

            let ready = x.iter().filter(|&&x| x == 0).count();
            let born = vec![8 as u128; ready];

            new_status.extend(born.iter().cloned());
            return new_status;
        }).len() as u128
}

mod tests {
    use crate::day_06::solve;
    use crate::input;

    #[test]
    fn test_solve_part_1() {
        let input: Vec<u128> = vec![3,4,3,1,2];
        assert_eq!(solve(&input, 80), 5934);
    }

    #[test]
    fn test_solve_part_2() {
        let input: Vec<u128> = vec![3,4,3,1,2];
        assert_eq!(solve(&input, 256), 26984457539);
    }
}
