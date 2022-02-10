// use std::collections::HashSet;
use crate::input;
use petgraph::graph::{NodeIndex, UnGraph};

pub fn print_solution() {
    let input = input::read("data/12.txt");
    let data = parse_input(&input);

    println!("# Day 12");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

type CaveNameSize = i32;
type Caves<'a> = UnGraph<&'a String, ()>;

fn encode_name(name: &String) -> CaveNameSize {
    name
        .chars()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, v)| sum + v as CaveNameSize * 10i32.pow(3*i as u32))
}

// #[derive(Debug, PartialEq, Clone)]
// enum CaveSize {
//     Small,
//     Big,
//     Start,
//     End,
// }
//
// #[derive(Debug, PartialEq, Clone)]
// struct Cave<'a> {
//     name: String,
//     size: CaveSize,
//     connections: Vec<&'a mut Cave<'a>>,
// }

// impl Cave<'_> {
//     fn connect(&mut self, other: &mut Cave<'_>) {
//         if !self.connections.contains(&other) {
//             self.connections.push(other)
//         }
//
//         if !other.connections.contains(&self) {
//             other.connections.push(self)
//         }
//     }
// }
//
// struct Path {
//     // caves: Vec<Cave>
// }

fn parse_input<'a>(input: &Vec<String>) -> Caves<'a> {
    return Caves::from_edges(&vec![
        (&input[0], &input[1]),
    ]);
    // let caves = vec![
    //     Cave {size: Start, name: "start".to_string(), connections: Vec::new()}
    // ];
    //
    // for line in input.iter() {
    //
    // }
    // return caves.get(0).expect("Empty vector");
    // input
    //     .iter()
    //     .map(|x| x.trim().parse().expect("Not an integer"))
    //     .collect::<Vec<i64>>()
}

fn solve_part_1(data: &Caves) -> i64 {
    0
}

fn solve_part_2(data: &Caves) -> i64 {
    0
}

mod tests {
    use std::collections::{HashMap, HashSet};
    use petgraph::adj::NodeIndex;
    use petgraph::graph::UnGraph;
    use crate::day_12::{CaveNameSize, Caves, encode_name, solve_part_1, solve_part_2};
    use crate::input;

    fn get_input() -> Caves<'static> {
        let mut graph = Caves::new_undirected();
        let nodes = vec![
            ("start".to_string(), graph.add_node(&"start".to_string())),
            ("A".to_string(), graph.add_node(&"A".to_string())),
            ("b".to_string(), graph.add_node(&"b".to_string())),
            ("c".to_string(), graph.add_node(&"c".to_string())),
            ("d".to_string(), graph.add_node(&"d".to_string())),
            ("end".to_string(), graph.add_node(&"end".to_string())),
        ].iter().collect::<HashMap<String, NodeIndex>>();

        let start = graph.node_indices()["start".to_string()];
        // graph.add_edge(nodes.get("start".to_string()), nodes.get("A".to_string()));
        // UnGraph::<CaveNameSize, ()>::from_edges(&vec![
        //     (encode_name(&"start".to_string()), encode_name(&"A".to_string())),
        //     (encode_name(&"start".to_string()), encode_name(&"b".to_string())),
        //     (encode_name(&"A".to_string()), encode_name(&"c".to_string())),
        //     (encode_name(&"A".to_string()), encode_name(&"b".to_string())),
        //     (encode_name(&"b".to_string()), encode_name(&"d".to_string())),
        //     (encode_name(&"A".to_string()), encode_name(&"end".to_string())),
        //     (encode_name(&"b".to_string()), encode_name(&"end".to_string())),
        // ])

        graph
    }

    #[test]
    fn test_encode_name() {
        let name = "start".to_string();
        let expected = 116 * 10i32.pow(3*0)
                           + 114 * 10i32.pow(3*1)
                           + 97  * 10i32.pow(3*2)
                           + 116 * 10i32.pow(3*3)
                           + 115 * 10i32.pow(3*4);

        assert_eq!(encode_name(&name), expected);
    }

    #[test]
    fn test_parse_input() {
        let input = vec![
            "start-A".to_string(),
            "start-b".to_string(),
            "A-c".to_string(),
            "A-b".to_string(),
            "b-d".to_string(),
            "A-end".to_string(),
            "b-end".to_string(),
        ];

        // assert_eq!(parse_input(&input), get_input());
    }

    #[test]
    fn test_solve_part_1() {
        let input = get_input();
        assert_eq!(solve_part_1(&input), 1);
    }

    // #[test]
    // fn test_solve_part_2() {
    //     let input = get_input();
    //     assert_eq!(solve_part_2(&input), 1);
    // }

    // #[test]
    // fn test_golden_master() {
    //     let input = input::read("data/12.txt");
    //     let data = parse_input(&input);
    //     assert_eq!(solve_part_1(&data), 1);
    //     assert_eq!(solve_part_2(&data), 1);
    // }
}
