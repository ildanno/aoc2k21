use crate::input;

pub fn print_solution() {
    let data = get_input();

    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", "");
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
    let mut position = (0, 0);
    for command in commands {
        if command.0.eq(&String::from("forward")) { position.1 += command.1 }
        else if command.0.eq(&String::from("down")) { position.0 += command.1 }
        else if command.0.eq(&String::from("up")) { position.0 -= command.1 }
    }

    return position.0 * position.1;
}

mod tests {
    use crate::day_02::{parse_commands, solve_part_1};

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
}
