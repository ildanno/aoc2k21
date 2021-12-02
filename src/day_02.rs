use crate::input;

#[derive(Debug, PartialEq)]
enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
}

struct Position { x: u32, y: u32, a: u32 }
impl Position {
    fn multiply(&self) -> u32 { self.x * self.y }
}

pub fn print_solution() {
    let data = get_input();

    println!("# Day 2");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn get_input() -> Vec<Command> {
    input::read("data/02.txt")
        .iter()
        .map(|x| parse_command(x).expect("Invalid command"))
        .collect()
}

fn parse_command(x: &String) -> Result<Command, &str> {
    let p = x
        .split(" ")
        .map(|x| x)
        .collect::<Vec<&str>>();

    let &direction = p
        .get(0)
        .expect("Cannot get direction from command string");

    let length = p
        .get(1)
        .expect("Cannot get length from command string")
        .parse::<u32>()
        .expect("Cannot parse length from command string");

    match direction {
        "up" => Ok(Command::Up(length)),
        "down" => Ok(Command::Down(length)),
        "forward" => Ok(Command::Forward(length)),
        _ => Err("Invalid command")
    }
}

fn solve_part_1(commands: &Vec<Command>) -> u32 {
    commands
        .iter()
        .fold(Position{x: 0, y: 0, a: 0}, |p, command| {
            match command {
                Command::Up(lenght) => Position{ y: p.y - lenght, ..p },
                Command::Down(lenght) => Position{ y: p.y + lenght, ..p },
                Command::Forward(lenght) => Position{ x: p.x + lenght, ..p }
            }
        })
        .multiply()
}

fn solve_part_2(commands: &Vec<Command>) -> u32 {
    commands
        .iter()
        .fold(Position{x: 0, y: 0, a: 0}, |p, command| {
            match command {
                Command::Up(lenght) => Position{ a: p.a - lenght, ..p },
                Command::Down(lenght) => Position{ a: p.a + lenght, ..p },
                Command::Forward(lenght) => Position{ x: p.x + lenght, y: p.y + p.a * lenght, ..p }
            }
        })
        .multiply()
}

mod tests {
    use crate::day_02::{Command, get_input, parse_command, solve_part_1, solve_part_2};

    #[test]
    fn test_solve_part_1() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        assert_eq!(150, solve_part_1(&commands))
    }

    #[test]
    fn test_solve_part_2() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        assert_eq!(900, solve_part_2(&commands))
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command(&String::from("forward 5")),
            Ok(Command::Forward(5))
        );

        assert_eq!(
            parse_command(&String::from("down 8")),
            Ok(Command::Down(8))
        );

        assert_eq!(
            parse_command(&String::from("up 3")),
            Ok(Command::Up(3))
        );

        assert_eq!(
            parse_command(&String::from("back 6")),
            Err("Invalid command")
        )
    }

    #[test]
    fn test_golden_master_solution() {
        let data = get_input();
        assert_eq!(solve_part_1(&data), 1868935);
        assert_eq!(solve_part_2(&data), 1965970888)
    }
}
