use crate::input;

pub fn part1() {
    let data = input::read("data/1-1.txt")
        .iter()
        .map(|x| x.parse().expect("Cannot parse"))
        .collect::<Vec<u32>>();

    println!("{}", solve_part1(data))
}

fn solve_part1(depths: Vec<u32>) -> u32 {
    let mut count = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i-1] {
            count+=1
        }
    }
    return count
}

mod tests {
    use crate::day_01::solve_part1;

    #[test]
    fn test_solve_part1() {
        let data: Vec<u32> = Vec::from([199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(solve_part1(data), 7)
    }
}
