use crate::input;

pub fn part1() {
    let data = input::read("data/1-1.txt")
        .iter()
        .map(|x| x.parse().expect("Cannot parse"))
        .collect::<Vec<u32>>();

    // println!("{}", solve_part1(data));
    println!("{}", solve_part2(data));
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

pub fn part2() {
    let data = input::read("data/1-1.txt")
        .iter()
        .map(|x| x.parse().expect("Cannot parse"))
        .collect::<Vec<u32>>();

    println!("{}", solve_part2(data))
}

fn solve_part2(depths: Vec<u32>) -> u32 {
    let mut count = 0;
    let mut windows = vec![];

    for i in 0..depths.len()-2 {
        windows.push([depths[i], depths[i+1], depths[i+2]])
    }

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
    use crate::day_01::{solve_part1, solve_part2};

    #[test]
    fn test_solve_part1() {
        let data: Vec<u32> = Vec::from([199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(solve_part1(data), 7)
    }

    #[test]
    fn test_solve_part2() {
        let data: Vec<u32> = Vec::from([199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(solve_part2(data), 5)
    }
}
