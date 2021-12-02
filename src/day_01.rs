use crate::input;

pub fn print_solution() {
    let data = get_input();

    println!("# Day 1");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn get_input() -> Vec<u32> {
    input::read("data/01.txt")
        .iter()
        .map(|x| x.parse().expect("Cannot parse"))
        .collect::<Vec<u32>>()
}

fn solve_part_1(depths: &Vec<u32>) -> u32 {
    return window_solution(depths, 1);
}

fn solve_part_2(depths: &Vec<u32>) -> u32 {
    return window_solution(depths, 3);
}

fn window_solution(depths: &Vec<u32>, window_size: u32) -> u32 {
    let windows = (0..depths.len()-(window_size as usize-1))
        .collect::<Vec<usize>>()
        .iter()
        .map(|&i| Vec::from(&depths[i..i+window_size as usize]))
        .collect::<Vec<Vec<u32>>>();

    let mut count = 0;

    for i in 1..windows.len() {
        let w1 = windows[i].iter().fold(0, |x, y| x + y);
        let w2 = windows[i - 1].iter().fold(0, |x, y| x + y);

        if w1 > w2 {
            count+=1
        }
    }

    return count
}

mod tests {
    use crate::day_01::{get_input, solve_part_1, solve_part_2};
    use crate::input;

    #[test]
    fn test_solve_part1() {
        let data: Vec<u32> = Vec::from([199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(solve_part_1(&data), 7)
    }

    #[test]
    fn test_solve_part2() {
        let data: Vec<u32> = Vec::from([199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(solve_part_2(&data), 5)
    }

    #[test]
    fn test_golden_master_solution() {
        let data = get_input();

        assert_eq!(solve_part_1(&data), 1288);
        assert_eq!(solve_part_2(&data), 1311)
    }
}
