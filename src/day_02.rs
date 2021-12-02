use crate::input;

pub fn print_solution() {
    let data = get_input();

    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn get_input() -> Vec<(String, u32)> {
    input::read("data/02.txt")
        .iter()
        .map(parse_command)
        .collect()
}

fn parse_commands(input: &Vec<String>) -> Vec<(String, u32)> {
    input
        .iter()
        .map(parse_command)
        .collect()
}

fn parse_command(x: &String) -> (String, u32) {
    let p = x
        .split(" ")
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    let direction = p.get(0).expect("eh");
    let length = p.get(1).expect("meh").parse::<u32>().expect("mah");

    (String::from(direction), length)
}

fn solve_part_1(commands: &Vec<(String, u32)>) -> u32 {
    let mut position_x = 0;
    let mut position_y = 0;

    for command in commands {
        let direction = &command.0;
        let lenght = command.1;

        if direction.eq(&String::from("forward")) { position_x += lenght }
        else if direction.eq(&String::from("down")) { position_y += lenght }
        else if direction.eq(&String::from("up")) { position_y -= lenght }
    }

    return position_y * position_x;
}

fn solve_part_2(commands: &Vec<(String, u32)>) -> u32 {
    let mut position_x = 0;
    let mut position_y = 0;
    let mut aim = 0;

    for command in commands {
        let direction = &command.0;
        let lenght = command.1;

        if direction.eq(&String::from("forward")) { position_x += lenght; position_y += aim * lenght }
        else if direction.eq(&String::from("down")) { aim += lenght }
        else if direction.eq(&String::from("up")) { aim -= lenght }
    }

    return position_y * position_x;
}

mod tests {
    use crate::day_02::{get_input, parse_commands, solve_part_1, solve_part_2};

    #[test]
    fn test_solve_part_1() {
        let commands = vec![
            (String::from("forward"), 5),
            (String::from("down"), 5),
            (String::from("forward"), 8),
            (String::from("up"), 3),
            (String::from("down"), 8),
            (String::from("forward"), 2),
        ];
        assert_eq!(150, solve_part_1(&commands))
    }

    #[test]
    fn test_solve_part_2() {
        let commands = vec![
            (String::from("forward"), 5),
            (String::from("down"), 5),
            (String::from("forward"), 8),
            (String::from("up"), 3),
            (String::from("down"), 8),
            (String::from("forward"), 2),
        ];
        assert_eq!(900, solve_part_2(&commands))
    }

    #[test]
    fn test_parse_commands() {
        let input = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];

        assert_eq!(
            parse_commands(&input),
            vec![
                (String::from("forward"), 5),
                (String::from("down"), 5),
                (String::from("forward"), 8),
                (String::from("up"), 3),
                (String::from("down"), 8),
                (String::from("forward"), 2),
            ]
        )
    }

    #[test]
    fn test_golden_master_solution() {
        assert_eq!(solve_part_1(&get_input()), 1868935)
    }
}
