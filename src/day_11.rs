use crate::input;

#[derive(Debug, PartialEq)]
struct Grid {
    points: Vec<Vec<i8>>
}

impl Grid {
    fn increase(&mut self, column: i8, line: i8) {
        if self.points[line as usize][column as usize] >= 10 { return; }
        self.points[line as usize][column as usize]+=1
    }

    fn increase_all(&mut self) {
        for line in 0..=9 {
            for column in 0..=9 {
                self.increase(column, line)
            }
        }
    }

    fn adjacents(column: i8, line: i8) -> Vec<(i8, i8)> {
        vec![
            (line -1, column -1),
            (line -1, column),
            (line -1, column +1),
            (line, column +1),
            (line +1, column +1),
            (line +1, column),
            (line +1, column -1),
            (line, column -1),
        ]
            .iter()
            .filter(|p| {
                p.0>=0 && p.0<10 && p.1>=0 && p.1<10
            })
            .map(|x| (x.0,x.1))
            .collect::<Vec<(i8, i8)>>()
    }

    fn search(&self, value: i8) -> Vec<(i8, i8)> {
        let mut points = vec![];
        for line in 0..=9 {
            for column in 0..=9 {
                if self.points[line][column] == value {
                    points.push((line as i8, column as i8))
                }
            }
        }
        Vec::from(points)
    }

    fn flash(&mut self, line: i8, column: i8) -> i64 {
        if self.points[line as usize][column as usize] == 11 {
            return 0
        }
        self.points[line as usize][column as usize] = 11; // flashing
        let sum: i64 = Grid::adjacents(column, line)
            .iter()
            .map(|adjacent| {
                self.increase(adjacent.1, adjacent.0);
                return if self.points[adjacent.0 as usize][adjacent.1 as usize] == 10 {
                    self.flash(adjacent.0, adjacent.1)
                } else {
                    0
                }
            })
            .sum();

        return sum + 1 as i64;
    }

    fn flash_all(&mut self) -> i64 {
        let flashes = self
            .search(10)
            .iter()
            .map(|x| self.flash(x.0, x.1))
            .sum();

        for x in self.search(11) {
            self.points[x.0 as usize][x.1 as usize] = 0;
        }

        return flashes;
    }
}

pub fn print_solution() {
    let input = input::read("data/11.txt");
    let mut data = parse_input(&input);

    println!("# Day 11");
    println!("Part 1: {}", solve_part_1(&mut data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn parse_input(input: &Vec<String>) -> Grid {
    Grid {
        points: input
            .iter()
            .map(|x| {
                x.chars()
                    .map(|c| c.to_string().parse().expect("Not an integer"))
                    .collect::<Vec<i8>>()
            })
            .collect::<Vec<Vec<i8>>>()
    }
}

fn solve_part_1(grid: &mut Grid) -> i64 {
    (1..=100)
        .into_iter()
        .fold(0, |sum, step| -> i64 {
            grid.increase_all();
            let flashes = grid.flash_all();
            sum + flashes
        })
}

fn solve_part_2(data: &Grid) -> i64 {
    0
}

mod tests {
    use crate::day_11::{Grid, parse_input, solve_part_1, solve_part_2};
    use crate::input;

    fn get_input() -> Grid {
        Grid {
            points: vec![
                vec![5,4,8,3,1,4,3,2,2,3],
                vec![2,7,4,5,8,5,4,7,1,1],
                vec![5,2,6,4,5,5,6,1,7,3],
                vec![6,1,4,1,3,3,6,1,4,6],
                vec![6,3,5,7,3,8,5,4,7,8],
                vec![4,1,6,7,5,2,4,6,4,5],
                vec![2,1,7,6,8,4,1,7,2,1],
                vec![6,8,8,2,8,8,1,1,3,4],
                vec![4,8,4,6,8,4,8,5,5,4],
                vec![5,2,8,3,7,5,1,5,2,6],
            ]
        }
    }

    #[test]
    fn test_parse() {
        let input = vec![
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ];
        let expected = get_input();

        assert_eq!(parse_input(&input), expected);
    }

    #[test]
    fn test_adjacents() {
        assert_eq!(Grid::adjacents(0, 0), vec![(0,1), (1,1), (1,0)]);
        assert_eq!(Grid::adjacents(9, 9), vec![(8,8), (8,9), (9,8)]);
        assert_eq!(Grid::adjacents(4, 4), vec![(3,3), (3,4), (3,5), (4,5), (5,5), (5,4), (5,3), (4,3)]);
    }

    #[test]
    fn test_solve_part_1() {
        let mut input = get_input();
        assert_eq!(solve_part_1(&mut input), 1656);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input();
        assert_eq!(solve_part_2(&input), 1);
    }

    #[test]
    fn test_golden_master() {
        let input = input::read("data/11.txt");
        let mut data = parse_input(&input);
        assert_eq!(solve_part_1(&mut data), 1673);
        assert_eq!(solve_part_2(&data), 1);
    }
}
