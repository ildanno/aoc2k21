use crate::input;

pub fn print_solution() {
    let data = input::read("data/04.txt");
    let game = parse_input(&data);

    println!("# Day 4");
    println!("Part 1: {}", solve_part_1(&game));
    // println!("Part 2: {}", solve_part_2(&data));
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    called: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Board {
    cells: [[Cell; 5]; 5],
}

impl Board {
    fn mark(&self, number: u32) -> Self {
        let mut cells = self.cells;
        for i in 0..5 {
            for j in 0..5 {
                if cells[i][j].number == number {
                    cells[i][j].mark()
                }
            }
        }

        Board{ cells }
    }

    fn is_winner(&self) -> bool {
        for i in 0..5 {
            let mut w_line = true;
            let mut w_col = true;
            for j in 0..5 {
                w_line = w_line && self.cells[i][j].marked;
                w_col = w_col && self.cells[j][i].marked;
            }
            if w_line || w_col { return true }
        }

        false
    }

    fn sum(&self) -> u32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.cells[i][j].marked { sum = sum + self.cells[i][j].number }
            }
        }

        sum
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Cell {
    number: u32,
    marked: bool,
}

impl Cell {
    fn new(number: u32) -> Self {
        Self { number, marked: false }
    }

    fn mark(&mut self) {
        self.marked = true
    }
}

fn parse_input(input: &Vec<String>) -> Game {
    let input = input
        .iter()
        .filter(|x| ! x.trim().is_empty())
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    let called = input
        .get(0)
        .expect("Empty list")
        .split(",")
        .map(|x| x.trim().parse().expect("Not an integer"))
        .collect::<Vec<u32>>();

    let parse_cells = |s: &String| -> [Cell; 5] {
        let cells = s
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| Cell::new(x.parse().expect("Not an integer")))
            .collect::<Vec<Cell>>();

        [
            *cells.get(0).expect("Not a valid Cell"),
            *cells.get(1).expect("Not a valid Cell"),
            *cells.get(2).expect("Not a valid Cell"),
            *cells.get(3).expect("Not a valid Cell"),
            *cells.get(4).expect("Not a valid Cell"),
        ]
    };

    let boards = (1..=(input.len()-1)/5)
        .into_iter()
        .fold(vec![], |boards, board_number| {
            let i = board_number - 1;
            let mut boards = boards;
            boards.push(Board {
                cells: [
                    parse_cells(&input[i*5+1]),
                    parse_cells(&input[i*5+2]),
                    parse_cells(&input[i*5+3]),
                    parse_cells(&input[i*5+4]),
                    parse_cells(&input[i*5+5]),
                ],
            });
            boards
        });

    return Game{ called, boards };
}

fn solve_part_1(game: &Game) -> u32 {
    let mut last = 0;
    let boards = game.clone()
        .called
        .iter()
        .fold(game.clone().boards, |boards, &num| {
            let one_is_winner = |bb: &Vec<Board>| {
                bb
                    .iter()
                    .fold(false, |w, b| w || b.is_winner())
            };

            if one_is_winner(&boards) { return boards }

            let boards = boards
                .iter()
                .map(|&b| b.mark(num))
                .collect::<Vec<Board>>();

            if one_is_winner(&boards) { last = num }

            return boards
        });

    boards
        .iter()
        .filter(|b| b.is_winner())
        .fold(0, |r, b| b.sum()) * last
}

// fn solve_part_2(report: &Vec<String>) -> i32 {
// }

mod tests {
    use crate::day_04::{Board, Cell, Game, parse_input, solve_part_1};
    use crate::input;

    #[test]
    fn test_parse_input() {
        let data = input::read("data/04-test.txt");
        assert_eq!(
            parse_input(&data),
            Game {
                called: vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1],
                boards: vec![
                    Board {
                        cells: [
                            [Cell::new(22), Cell::new(13), Cell::new(17), Cell::new(11), Cell::new(0)],
                            [Cell::new(8), Cell::new(2), Cell::new(23), Cell::new(4), Cell::new(24)],
                            [Cell::new(21), Cell::new(9), Cell::new(14), Cell::new(16), Cell::new(7)],
                            [Cell::new(6), Cell::new(10), Cell::new(3), Cell::new(18), Cell::new(5)],
                            [Cell::new(1), Cell::new(12), Cell::new(20), Cell::new(15), Cell::new(19)],
                        ]
                    },
                    Board {
                        cells: [
                            [Cell::new(3), Cell::new(15), Cell::new(0), Cell::new(2), Cell::new(22)],
                            [Cell::new(9), Cell::new(18), Cell::new(13), Cell::new(17), Cell::new(5)],
                            [Cell::new(19), Cell::new(8), Cell::new(7), Cell::new(25), Cell::new(23)],
                            [Cell::new(20), Cell::new(11), Cell::new(10), Cell::new(24), Cell::new(4)],
                            [Cell::new(14), Cell::new(21), Cell::new(16), Cell::new(12), Cell::new(6)],
                        ]
                    },
                    Board {
                        cells: [
                            [Cell::new(14), Cell::new(21), Cell::new(17), Cell::new(24), Cell::new(4)],
                            [Cell::new(10), Cell::new(16), Cell::new(15), Cell::new(9), Cell::new(19)],
                            [Cell::new(18), Cell::new(8), Cell::new(23), Cell::new(26), Cell::new(20)],
                            [Cell::new(22), Cell::new(11), Cell::new(13), Cell::new(6), Cell::new(5)],
                            [Cell::new(2), Cell::new(0), Cell::new(12), Cell::new(3), Cell::new(7)],
                        ]
                    },
                ]
            }
        )
    }

    #[test]
    fn test_mark_board() {
        let mut board = Board {
            cells: [
                [Cell::new(22), Cell::new(13), Cell::new(17), Cell::new(11), Cell::new(0)],
                [Cell::new(8), Cell::new(2), Cell::new(23), Cell::new(4), Cell::new(24)],
                [Cell::new(21), Cell::new(9), Cell::new(14), Cell::new(16), Cell::new(7)],
                [Cell::new(6), Cell::new(10), Cell::new(3), Cell::new(18), Cell::new(5)],
                [Cell::new(1), Cell::new(12), Cell::new(20), Cell::new(15), Cell::new(19)],
            ]
        };

        assert_eq!(
            board.mark(17),
            Board {
                cells: [
                    [Cell::new(22), Cell::new(13), Cell{ number: 17, marked: true}, Cell::new(11), Cell::new(0)],
                    [Cell::new(8), Cell::new(2), Cell::new(23), Cell::new(4), Cell::new(24)],
                    [Cell::new(21), Cell::new(9), Cell::new(14), Cell::new(16), Cell::new(7)],
                    [Cell::new(6), Cell::new(10), Cell::new(3), Cell::new(18), Cell::new(5)],
                    [Cell::new(1), Cell::new(12), Cell::new(20), Cell::new(15), Cell::new(19)],
                ]
            }
        )
    }

    #[test]
    fn test_board_is_winner_row() {
        let board = Board {
            cells: [
                [Cell::new(22), Cell::new(13), Cell::new(17), Cell::new(11), Cell::new(0)],
                [Cell::new(8), Cell::new(2), Cell::new(23), Cell::new(4), Cell::new(24)],
                [Cell::new(21), Cell::new(9), Cell::new(14), Cell::new(16), Cell::new(7)],
                [Cell::new(6), Cell::new(10), Cell::new(3), Cell::new(18), Cell::new(5)],
                [Cell::new(1), Cell::new(12), Cell::new(20), Cell::new(15), Cell::new(19)],
            ]
        };

        assert_eq!(board.is_winner(), false);
        let non_winner = board
            .mark(22)
            .mark(2)
            .mark(14)
            .mark(18)
            .mark(19)
            .mark(16)
            .mark(7)
            .mark(21);

        assert_eq!(non_winner.is_winner(), false);
        assert_eq!(non_winner.mark(9).is_winner(), true);
    }

    #[test]
    fn test_board_is_winner_col() {
        let board = Board {
            cells: [
                [Cell::new(22), Cell::new(13), Cell::new(17), Cell::new(11), Cell::new(0)],
                [Cell::new(8), Cell::new(2), Cell::new(23), Cell::new(4), Cell::new(24)],
                [Cell::new(21), Cell::new(9), Cell::new(14), Cell::new(16), Cell::new(7)],
                [Cell::new(6), Cell::new(10), Cell::new(3), Cell::new(18), Cell::new(5)],
                [Cell::new(1), Cell::new(12), Cell::new(20), Cell::new(15), Cell::new(19)],
            ]
        };

        assert_eq!(board.is_winner(), false);
        let non_winning = board
            .mark(2)
            .mark(14)
            .mark(18)
            .mark(19)
            .mark(13)
            .mark(9)
            .mark(10);

        assert_eq!(non_winning.is_winner(), false);
        assert_eq!(non_winning.mark(12).is_winner(), true);
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            solve_part_1(&parse_input(&input::read("data/04-test.txt"))),
            4512
        )
    }

    #[test]
    fn test_golden_master() {
        assert_eq!(
            solve_part_1(&parse_input(&input::read("data/04.txt"))),
            8580
        )
    }
}
