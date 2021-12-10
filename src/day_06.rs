use std::cmp::max;
use std::collections::HashMap;
use crate::input;

pub fn print_solution() {
    let data = parse_input(
        input::read("data/06.txt")
            .get(0)
            .expect("Empty")
    );

    println!("# Day 6");
    println!("Part 1: {}", solve(&data, 80));
    println!("Part 2: {}", solve(&data, 256));
}

fn parse_input(data: &String) -> Vec<u128> {
    data
        .split(",")
        .map(|x| x.trim().parse().expect("Not an integer"))
        .collect::<Vec<u128>>()
}

fn _solve_slow(data: &Vec<u128>, i: i32) -> u128 {
    (0..i)
        .fold(data.clone(), |x, _n| {
            let mut new_status = x.iter()
                .map(|&x| { if x == 0 { 6 } else { x - 1 } })
                .collect::<Vec<u128>>();

            let ready = x.iter().filter(|&&x| x == 0).count();
            let born = vec![8 as u128; ready];

            new_status.extend(born.iter().cloned());
            return new_status;
        }).len() as u128
}

fn solve(data: &Vec<u128>, day: u128) -> u128 {
    let mut cache = HashMap::new();

    data
        .iter()
        .fold(0, |sum, &init| sum + project(init, day, &mut cache))
}

fn project(initial_state: u128, day: u128, mut cache: &mut HashMap<(u128, u128), u128>) -> u128 {
    // first is generated at day "initial_state+1" (initial day is day 0)
    if initial_state >= day { return 1 }

    let key = (initial_state, day);
    if cache.contains_key(&key) { return *cache.get(&key).expect("Item not found") }

    // then a first generation member is generated every 7 days (at "initial_state+1+7", "initial_state+1+14", ..)
    // so n after initial_state+1+7*N days
    const PERIOD: u128 = 7;
    let n = (day - initial_state - 1) / PERIOD + 1;

    // second+ generation members are generated in a similar way after ?? days
    let result = 1 + (0..n)
        .fold(0, |sum, x| {
            sum + project(8, day - initial_state - 1 - PERIOD * x, cache)
        });

    cache.insert(key, result);

    return result;
}

mod tests {
    use crate::day_06::{project, solve};

    #[test]
    fn test_solve() {
        let input: Vec<u128> = vec![3,4,3,1,2];
        assert_eq!(solve(&input, 80), 5934);
    }
}
