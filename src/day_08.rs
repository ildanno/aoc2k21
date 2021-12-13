use crate::input;

type InputSignal = Vec<char>;
type InputNumber = Vec<char>;
#[derive(Debug, PartialEq)]
struct InputLine {signals: Vec<InputSignal>, numbers: Vec<InputNumber>}

pub fn print_solution() {
    let input = input::read("data/08.txt");
    let data = parse_input(&input);

    println!("# Day 08");
    println!("Part 1: {}", solve_part_1(&data));
    // println!("Part 2: {}", solve_part_2(&data));
}

fn parse_input(input: &Vec<String>) -> Vec<InputLine> {
    input
        .iter()
        .map(|x| {
            let a = x.split("|").map(|s| String::from(s)).collect::<Vec<String>>();
            let signals: Vec<InputSignal> = parse_list(a.get(0));
            let numbers: Vec<InputNumber> = parse_list(a.get(1));
            return InputLine{ signals, numbers }
        })
        .collect::<Vec<InputLine>>()
}

fn parse_list(option_1: Option<&String>) -> Vec<Vec<char>> {
    option_1
        .expect("Empty list")
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|b| b.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn solve_part_1(data: &Vec<InputLine>) -> usize {
    let map = data
        .iter()
        .map(|l| l.numbers.clone())
        .map(|l| {
            l.iter()
                .filter(|n| match n.len() {
                    2 | 4 | 3 | 7 => true,
                    _ => false
                })
                .count()
        }).collect::<Vec<usize>>();

    map.iter().sum()
}

fn solve_part_2(data: &Vec<InputLine>) -> i64 {
    0
}

mod tests {
    use crate::day_08::{InputLine, parse_input, solve_part_1, solve_part_2};
    use crate::input;

    fn get_input() -> Vec<InputLine> {
        parse_input(&vec![
            String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"),
        ])
    }

    #[test]
    fn test_parse_input() {
        let input = vec![String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")];
        let input_line = InputLine {
            signals: vec![
                vec!['a','c','e','d','g','f','b'],
                vec!['c','d','f','b','e'],
                vec!['g','c','d','f','a'],
                vec!['f','b','c','a','d'],
                vec!['d','a','b'],
                vec!['c','e','f','a','b','d'],
                vec!['c','d','f','g','e','b'],
                vec!['e','a','f','b'],
                vec!['c','a','g','e','d','b'],
                vec!['a','b'],
            ],
            numbers: vec![
                vec!['c','d','f','e','b'],
                vec!['f','c','a','d','b'],
                vec!['c','d','f','e','b'],
                vec!['c','d','b','a','f'],
            ],
        };
        assert_eq!(parse_input(&input), vec![input_line]);
    }

    #[test]
    fn test_solve_part_1() {
        let input = get_input();
        assert_eq!(solve_part_1(&input), 26);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input();
        assert_eq!(solve_part_2(&input), 1);
    }

    #[test]
    fn test_golden_master() {
        let input = input::read("data/08.txt");
        let data = parse_input(&input);
        assert_eq!(solve_part_1(&data), 456);
        // assert_eq!(solve_part_2(&data), 1);
    }
}
