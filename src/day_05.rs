use std::cmp::{max, Ordering};
use std::collections::HashSet;
use crate::input;

pub fn print_solution() {
    let data = input::read("data/05.txt");
    let lines = parse_input(&data);

    println!("# Day 5");
    println!("Part 1: {}", solve_part_1(&lines)); // warning: 13s exec time
    println!("Part 2: {}", solve_part_2(&lines)); // warning: 35s exec time!!
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point { x: i32, y: i32 }

#[derive(Debug, PartialEq, Copy, Clone)]
struct Line { start: Point, end: Point }
impl Line {
    fn covered_points(self) -> HashSet<Point> {
        let dist_x = self.end.x - self.start.x;
        let dist_y = self.end.y - self.start.y;
        let step_x = match dist_x.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1
        };
        let step_y = match dist_y.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1
        };
        let distance = max(dist_x.abs(), dist_y.abs());
        (0..=distance)
            .fold(HashSet::new(), |mut set, i| {
                set.insert(Point {x: self.start.x + (step_x * i), y: self.start.y + (step_y * i)});
                set
            })
    }
}

fn solve_part_1(lines: &Vec<Line>) -> i32 {
    let filtered = lines
        .iter()
        .filter(|l| { l.start.x == l.end.x || l.start.y == l.end.y })
        .map(|&l| l)
        .collect::<Vec<Line>>()
        ;

    solve_common(&filtered)
}

fn solve_common(filter: &Vec<Line>) -> i32 {
    let sets = filter
        .iter()
        .fold((HashSet::new(), HashSet::new()), |y, l| {
            let points = l.covered_points();
            let union = y.0
                .union(&points)
                .map(|p| Point { x: p.x, y: p.y })
                .collect::<HashSet<Point>>();

            let new_intersection = y.0
                .intersection(&points)
                .map(|p| Point { x: p.x, y: p.y })
                .collect::<HashSet<Point>>();

            let intersection = y.1
                .union(&new_intersection)
                .map(|p| Point { x: p.x, y: p.y })
                .collect::<HashSet<Point>>();

            (union, intersection)
        });

    sets.1.len() as i32
}

fn solve_part_2(lines: &Vec<Line>) -> i32 {
    solve_common(&lines)
}

fn parse_input(data: &Vec<String>) -> Vec<Line> {
    let strtopoint = |s: &str| {
        let p = s.split(",").map(|t| t.trim().parse().expect("Not a valid number")).collect::<Vec<i32>>();
        Point { x: p[0], y: p[1] }
    };

    let strtoline = |row: &String| {
        let l = row.split("->").map(strtopoint).collect::<Vec<Point>>();
        Line { start: Point { x: l[0].x, y: l[0].y }, end: Point { x: l[1].x, y: l[1].y } }
    };

    data
        .iter()
        .map(strtoline)
        .collect::<Vec<Line>>()
}

mod tests {
    use std::collections::HashSet;
    use crate::day_05::{Line, parse_input, Point, solve_part_1, solve_part_2};
    use crate::input;

    #[test]
    fn test_parse_input() {
        let data = vec![
            String::from("0,9 -> 5,9"),
            String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
            String::from("2,2 -> 2,1"),
            String::from("7,0 -> 7,4"),
            String::from("6,4 -> 2,0"),
            String::from("0,9 -> 2,9"),
            String::from("3,4 -> 1,4"),
            String::from("0,0 -> 8,8"),
            String::from("5,5 -> 8,2"),
        ];
        let list = vec![
            Line{ start: Point{x:0,y:9}, end: Point{x:5,y:9}},
            Line{ start: Point{x:8,y:0}, end: Point{x:0,y:8}},
            Line{ start: Point{x:9,y:4}, end: Point{x:3,y:4}},
            Line{ start: Point{x:2,y:2}, end: Point{x:2,y:1}},
            Line{ start: Point{x:7,y:0}, end: Point{x:7,y:4}},
            Line{ start: Point{x:6,y:4}, end: Point{x:2,y:0}},
            Line{ start: Point{x:0,y:9}, end: Point{x:2,y:9}},
            Line{ start: Point{x:3,y:4}, end: Point{x:1,y:4}},
            Line{ start: Point{x:0,y:0}, end: Point{x:8,y:8}},
            Line{ start: Point{x:5,y:5}, end: Point{x:8,y:2}},
        ];
        assert_eq!(parse_input(&data), list)
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(&vec![
            Line{ start: Point{x:0,y:9}, end: Point{x:5,y:9}},
            Line{ start: Point{x:8,y:0}, end: Point{x:0,y:8}},
            Line{ start: Point{x:9,y:4}, end: Point{x:3,y:4}},
            Line{ start: Point{x:2,y:2}, end: Point{x:2,y:1}},
            Line{ start: Point{x:7,y:0}, end: Point{x:7,y:4}},
            Line{ start: Point{x:6,y:4}, end: Point{x:2,y:0}},
            Line{ start: Point{x:0,y:9}, end: Point{x:2,y:9}},
            Line{ start: Point{x:3,y:4}, end: Point{x:1,y:4}},
            Line{ start: Point{x:0,y:0}, end: Point{x:8,y:8}},
            Line{ start: Point{x:5,y:5}, end: Point{x:8,y:2}},
        ]), 5)
    }

    #[test]
    fn test_covered_points() {
        assert_eq!(
            Line{ start: Point{x:0,y:9}, end: Point{x:5,y:9}}.covered_points(),
            HashSet::from([Point{x:0,y:9}, Point{x:1,y:9}, Point{x:2,y:9}, Point{x:3,y:9}, Point{x:4,y:9}, Point{x:5,y:9}])
        );
        assert_eq!(
            Line{ start: Point{x:2,y:2}, end: Point{x:2,y:1}}.covered_points(),
            HashSet::from([Point{x:2,y:2}, Point{x:2,y:1}])
        );
        assert_eq!(
            Line{ start: Point{x:1,y:1}, end: Point{x:3,y:3}}.covered_points(),
            HashSet::from([Point{x:1,y:1}, Point{x:2,y:2}, Point{x:3,y:3}])
        );
        assert_eq!(
            Line{ start: Point{x:9,y:7}, end: Point{x:7,y:9}}.covered_points(),
            HashSet::from([Point{x:9,y:7}, Point{x:8,y:8}, Point{x:7,y:9}])
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(solve_part_2(&vec![
            Line{ start: Point{x:0,y:9}, end: Point{x:5,y:9}},
            Line{ start: Point{x:8,y:0}, end: Point{x:0,y:8}},
            Line{ start: Point{x:9,y:4}, end: Point{x:3,y:4}},
            Line{ start: Point{x:2,y:2}, end: Point{x:2,y:1}},
            Line{ start: Point{x:7,y:0}, end: Point{x:7,y:4}},
            Line{ start: Point{x:6,y:4}, end: Point{x:2,y:0}},
            Line{ start: Point{x:0,y:9}, end: Point{x:2,y:9}},
            Line{ start: Point{x:3,y:4}, end: Point{x:1,y:4}},
            Line{ start: Point{x:0,y:0}, end: Point{x:8,y:8}},
            Line{ start: Point{x:5,y:5}, end: Point{x:8,y:2}},
        ]), 12)
    }

    #[test]
    fn test_golden_master() {
        let data = input::read("data/05.txt");
        let lines = parse_input(&data);

        assert_eq!(solve_part_1(&lines), 6311);
        assert_eq!(solve_part_2(&lines), 19929);
    }
}
