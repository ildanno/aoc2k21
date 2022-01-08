use crate::input;

pub fn print_solution() {
    let input = input::read("data/09.txt");
    let data = decode_input(&parse_input(&input));

    println!("# Day 09");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    y: i64, // line
    x: i64, // col
    z: i8,  // height
}

#[derive(Debug, PartialEq, Clone)]
struct Heightmap {
    points: Vec<Vec<Point>>
}

impl Heightmap {
    fn up(&self, p: &Point) -> Option<Point> {
        if p.y == 0 { None } else { Some(self.points.get(p.y as usize - 1).expect("").get(p.x as usize).expect("").clone()) }
    }

    fn down(&self, p: &Point) -> Option<Point> {
        if p.y == (self.points.len() - 1) as i64 { None } else { Some(self.points.get(p.y as usize + 1).expect("").get(p.x as usize).expect("").clone()) }
    }

    fn left(&self, p: &Point) -> Option<Point> {
        if p.x == 0 { None } else { Some(self.points.get(p.y as usize).expect("").get(p.x as usize - 1).expect("").clone()) }
    }

    fn right(&self, p: &Point) -> Option<Point> {
        if p.x == (self.points[0].len() - 1) as i64 { None } else { Some(self.points.get(p.y as usize).expect("").get(p.x as usize + 1).expect("").clone()) }
    }
}


fn parse_input(input: &Vec<String>) -> Vec<Vec<i8>> {
    input
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| y.to_string().parse().expect("Cannot parse integer"))
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>()
}

fn decode_input(input: &Vec<Vec<i8>>) -> Heightmap {
    let points = input
        .into_iter()
        .enumerate()
        .map(|x| {
            x.1
                .into_iter()
                .enumerate()
                .map(|y| {
                    Point{ y: x.0.clone() as i64, x: y.0.clone() as i64, z: y.1.clone()}
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    return Heightmap {points: points.clone()};
}

fn solve_part_1(map: &Heightmap) -> i64 {
    lowers(map)
        .iter()
        .map(|p| p.z as i64 + 1)
        .sum()
}

fn lowers(map: &Heightmap) -> Vec<Point> {
    map.points
        .iter()
        .enumerate()
        .fold(vec![], |lowers, line| {
            let mut new = line.1
                .iter()
                .enumerate()
                .filter(|p| {
                    let top = match map.up(p.1) {
                        None => true,
                        Some(t) => p.1.z < t.z
                    };
                    let bottom = match map.down(p.1) {
                        None => true,
                        Some(t) => p.1.z < t.z
                    };
                    let left = match map.left(p.1) {
                        None => true,
                        Some(t) => p.1.z < t.z
                    };
                    let right = match map.right(p.1) {
                        None => true,
                        Some(t) => p.1.z < t.z
                    };
                    return top && bottom && left && right
                })
                .map(|p| p.1.clone())
                .collect::<Vec<Point>>();
            let mut lowers = lowers;
            lowers.append(&mut new);

            return lowers.clone();
        })
}

fn solve_part_2(map: &Heightmap) -> i64 {
    let low = lowers(map);
    let basins = low
        .iter()
        .map(|p| basin(map, p, &vec![]))
        .collect::<Vec<Vec<Point>>>()
        ;

    let mut b_lenghts = basins
        .iter()
        .map(|b| b.len())
        .collect::<Vec<usize>>();

    (0..3).into_iter()
        .fold(1, |c, _| {
            let max = *b_lenghts.iter().max().unwrap();
            let index = b_lenghts.iter().position(|x| *x == max).unwrap();
            b_lenghts.remove(index);

            c * max as i64
        })
}

fn basin(map: &Heightmap, point: &Point, starting: &Vec<Point>) -> Vec<Point> {
    // println!("Inspecting point {:?}", point);
    if starting.contains(point) {
        // println!("Inspected point {:?} already in basin", point);
        return starting.clone();
    }
    let mut points = starting.clone();
    points.append(&mut vec![point.clone()]);
    let mut new_points = vec![point.clone()];
    for i in ["up","right","down","left"] {
        let next = match i {
            "up" => map.up(point),
            "right" => map.right(point),
            "down" => map.down(point),
            "left" => map.left(point),
            _ => None
        };
        match next {
            Some(p) if points.contains(&p) => {
                // println!("{:?} already there", p)
            },
            Some(p) if p.z == 9 => {
                // println!("{:?} is high, not basin", p)
            },
            Some(p) => {
                // println!("Next point ({}): {:?}", i, p);
                let mut n = basin(&map, &p, &points);
                points.append(&mut n.clone());
                new_points.append(&mut n);
            },
            None => {
                // println!("No options for {}", i);
            }
        }
    }

    return new_points.clone()
}

mod tests {
    use crate::day_09::{basin, decode_input, Heightmap, parse_input, Point, solve_part_1, solve_part_2};
    use crate::input;

    fn get_input() -> Vec<Vec<i8>> {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
    }

    fn get_map() -> Heightmap {
        Heightmap {
            points: vec![
                vec![
                    Point { y: 0, x: 0, z: 2 },
                    Point { y: 0, x: 1, z: 1 },
                    Point { y: 0, x: 2, z: 9 },
                    Point { y: 0, x: 3, z: 9 },
                    Point { y: 0, x: 4, z: 9 },
                    Point { y: 0, x: 5, z: 4 },
                    Point { y: 0, x: 6, z: 3 },
                    Point { y: 0, x: 7, z: 2 },
                    Point { y: 0, x: 8, z: 1 },
                    Point { y: 0, x: 9, z: 0 },
                ],
                vec![
                    Point { y: 1, x: 0, z: 3 },
                    Point { y: 1, x: 1, z: 9 },
                    Point { y: 1, x: 2, z: 8 },
                    Point { y: 1, x: 3, z: 7 },
                    Point { y: 1, x: 4, z: 8 },
                    Point { y: 1, x: 5, z: 9 },
                    Point { y: 1, x: 6, z: 4 },
                    Point { y: 1, x: 7, z: 9 },
                    Point { y: 1, x: 8, z: 2 },
                    Point { y: 1, x: 9, z: 1 },
                ],
                vec![
                    Point { y: 2, x: 0, z: 9 },
                    Point { y: 2, x: 1, z: 8 },
                    Point { y: 2, x: 2, z: 5 },
                    Point { y: 2, x: 3, z: 6 },
                    Point { y: 2, x: 4, z: 7 },
                    Point { y: 2, x: 5, z: 8 },
                    Point { y: 2, x: 6, z: 9 },
                    Point { y: 2, x: 7, z: 8 },
                    Point { y: 2, x: 8, z: 9 },
                    Point { y: 2, x: 9, z: 2 },
                ],
                vec![
                    Point { y: 3, x: 0, z: 8 },
                    Point { y: 3, x: 1, z: 7 },
                    Point { y: 3, x: 2, z: 6 },
                    Point { y: 3, x: 3, z: 7 },
                    Point { y: 3, x: 4, z: 8 },
                    Point { y: 3, x: 5, z: 9 },
                    Point { y: 3, x: 6, z: 6 },
                    Point { y: 3, x: 7, z: 7 },
                    Point { y: 3, x: 8, z: 8 },
                    Point { y: 3, x: 9, z: 9 },
                ],
                vec![
                    Point { y: 4, x: 0, z: 9 },
                    Point { y: 4, x: 1, z: 8 },
                    Point { y: 4, x: 2, z: 9 },
                    Point { y: 4, x: 3, z: 9 },
                    Point { y: 4, x: 4, z: 9 },
                    Point { y: 4, x: 5, z: 6 },
                    Point { y: 4, x: 6, z: 5 },
                    Point { y: 4, x: 7, z: 6 },
                    Point { y: 4, x: 8, z: 7 },
                    Point { y: 4, x: 9, z: 8 },
                ],
            ]
        }
    }

    #[test]
    fn test_parse_input() {
        let input = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];

        assert_eq!(parse_input(&input), get_input())
    }

    #[test]
    fn test_decode_input() {
        let expected = get_map();

        assert_eq!(decode_input(&get_input()), expected)
    }

    #[test]
    fn test_basin() {
        let map = get_map();
        let lower_point = Point { y: 0, x: 0, z: 2 };
        let expected1 = vec![
            lower_point,
            Point{y:0, x:1, z:1},
            Point{y:1, x:0, z:3},
        ];
        assert_eq!(basin(&map, &lower_point, &vec![]), expected1)
    }
    #[test]
    fn test_solve_part_1() {
        let input = get_input();
        assert_eq!(solve_part_1(&decode_input(&input)), 15);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input();
        assert_eq!(solve_part_2(&decode_input(&input)), 1134);
    }

    #[test]
    fn test_golden_master() {
        let input = input::read("data/09.txt");
        let data = parse_input(&input);
        assert_eq!(solve_part_1(&decode_input(&data)), 564);
        assert_eq!(solve_part_2(&decode_input(&data)), 1038240);
    }
}
