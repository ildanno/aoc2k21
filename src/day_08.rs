use std::collections::{HashMap, HashSet};
use crate::input;

type InputSignal = Vec<char>;
type InputNumber = Vec<char>;
#[derive(Debug, PartialEq, Clone)]
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

fn signal_with_lenght(signals: &Vec<InputSignal>, lenght: usize) -> Result<InputSignal, &str> {
    for i in 0..signals.len() {
        let signal = &signals[i];
        if signal.len() == lenght {
            return Ok(signal.iter().cloned().collect::<InputSignal>().to_owned())
        }
    }

    Err("No lenght match found")
}

fn signals_with_lenght(signals: &Vec<InputSignal>, lenght: usize) -> Vec<InputSignal> {
    signals
        .iter()
        .filter(|signal| signal.len() == lenght)
        .map(|x| x.clone())
        .collect::<Vec<InputSignal>>()
        .clone()
}

fn segments_with_occurrencies(signals: &Vec<InputSignal>, occurrencies: usize) -> Vec<char> {
    let letters = vec!['a', 'b', 'c', 'd', 'e', 'f'];

    letters
        .iter()
        .map(|letter| signals.iter().fold((letter, 0), |carry, signal| {
            (letter, carry.1 + if signal.contains(letter) { 1 } else { 0 })
        }))
        .filter(|c| c.1 == occurrencies)
        // i take the only item which is not present in just 1 digit
        .map(|tuple| *tuple.0)
        .collect::<Vec<char>>()
}

fn decode_signal(signals: &Vec<InputSignal>) -> HashMap<InputSignal, i64> {
    let mut signals = signals.clone();
    let mut hashmap = HashMap::new();

    let digit_1 = signals_with_lenght(&signals, 2).get(0).cloned().expect("");
    let digit_4 = signals_with_lenght(&signals, 4).get(0).cloned().expect("");
    let digit_7 = signals_with_lenght(&signals, 3).get(0).cloned().expect("");
    let digit_8 = signals_with_lenght(&signals, 7).get(0).cloned().expect("");

    hashmap.insert(digit_1.clone(), 1);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");
    hashmap.insert(digit_4.clone(), 4);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");
    hashmap.insert(digit_7.clone(), 7);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");
    hashmap.insert(digit_8.clone(), 8);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");

    let segment_f: char = *segments_with_occurrencies(&signals, 9).get(0).expect("");
    let digit_2 = signals_with_no_segment(&signals, segment_f).get(0).expect("").clone();
    hashmap.insert(digit_2.clone(), 2);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");

    let digit_1_segments: HashSet<char> = HashSet::from_iter(digit_1.clone());
    let digit_3 = signals_with_lenght(&signals, 5)
        .iter()
        .filter(|s| HashSet::from_iter(s.iter().cloned()).intersection(&digit_1_segments).eq(&digit_1_segments))
        .map(|x| x.clone())
        .collect::<Vec<InputSignal>>()
        .get(0)
        .cloned()
        .expect("");
    hashmap.insert(digit_3.clone(), 3);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");

    let digit_5 = signals
        .iter()
        .filter(|x| x.len() == 5)
        .filter(|x| !x.iter().cloned().eq(digit_2.clone()) && !x.iter().cloned().eq(digit_3.clone()))
        .map(|x| x.clone())
        .collect::<Vec<InputSignal>>()
        .get(0)
        .cloned()
        .expect("");
    hashmap.insert(digit_5, 5);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");

    let digit_4_segments: HashSet<char> = HashSet::from_iter(digit_4.clone());
    let digit_9 = signals_with_lenght(&signals, 6)
        .iter()
        .filter(|s| HashSet::from_iter(s.iter().cloned()).intersection(&digit_4_segments).eq(&digit_4_segments))
        .map(|x| x.clone())
        .collect::<Vec<InputSignal>>()
        .get(0)
        .cloned()
        .expect("");
    hashmap.insert(digit_9.clone(), 9);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");

    let digit_7_segments: HashSet<char> = HashSet::from_iter(digit_7.clone());
    let digit_0 = signals_with_lenght(&signals, 6)
        .iter()
        .filter(|&s| match hashmap.get(s) { None => true, Some(_) => false })
        .filter(|s| HashSet::from_iter(s.iter().cloned()).intersection(&digit_7_segments).eq(&digit_7_segments))
        .map(|x| x.clone())
        .collect::<Vec<InputSignal>>()
        .get(0)
        .cloned()
        .expect("");
    hashmap.insert(digit_0.clone(), 0);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");

    let digit_6 = signals
        .iter()
        .filter(|&s| match hashmap.get(s) { None => true, Some(_) => false })
        .map(|x| x.clone())
        .collect::<Vec<InputSignal>>()
        .get(0)
        .cloned()
        .expect("");
    hashmap.insert(digit_6.clone(), 6);
    for h in hashmap.clone() { println!("{:?}", h); } println!("----");

    // digit 1: L2 (unico)
    // digit 7: L3 (unico)
    // digit 4: L4 (unico)
    // digit 8: L7 (unico)
    // digit 2: L5 (1/3, unico a non avere segnale F)
    // digit 3: L5 (1/3, unico con L5 ad avere c+f, cioè quelli di 1)
    // digit 5: L5 (1/3, il restante con L5)
    // digit 9: L6 (1/3, contiene interamennte 4)
    // digit 0: L6 (1/3, contiene interamennte 7)
    // digit 6: L6 (1/3, ultimo restante)

    // a: in tutti eccetto 1 e 4
    // b: in tutti eccetto 1, 2, 3, 7
    // c: in tutti eccetto 5 e 6
    // d: in tutti eccetto 0, 1 e 7
    // e: SOLO IN 0, 2, 6, 8
    // f: in tutti eccetto 2
    // g: in tutti eccetto 1, 4, 7
    let hashmap = hashmap;
    return hashmap;
}

fn signals_with_no_segment(signals: &Vec<InputSignal>, segment: char) -> Vec<InputSignal> {
    signals
        .iter()
        .filter(|signal| ! signal.contains(&segment))
        .map(|x: &Vec<char>| -> InputSignal { x.into_iter().map(|a| *a).collect::<Vec<char>>()})
        .collect::<Vec<InputSignal>>()
}

fn solve_part_2(data: &Vec<InputLine>) -> i64 {
    data
        .iter()
        .map(|x| {
            x.numbers.iter().map(|n| {
                match n.iter().cloned().collect::<String>().trim() {
                    "acedgfb" => 8,
                    "cdfbe" => 5,
                    "gcdfa" => 2,
                    "fbcad" => 3,
                    "dab" => 7,
                    "cefabd" => 9,
                    "cdfgeb" => 6,
                    "eafb" => 4,
                    "cagedb" => 0,
                    "ab" => 1,
                    x => { println!("{}", x); -1 },
                }
            })
                .fold(0, |a, g| a * 10 + g)
        })
        .sum()
    // 0
}

mod tests {
    use std::array::IntoIter;
    use std::collections::HashMap;
    use crate::day_08::{decode_signal, InputLine, InputSignal, parse_input, signals_with_no_segment, solve_part_1, solve_part_2};
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
    fn test_decode_signal() {
        let expected= HashMap::<_, _>::from_iter(
            IntoIter::new([
                (vec!['a','c','e','d','g','f','b'], 8),
                (vec!['c','d','f','b','e'], 5),
                (vec!['g','c','d','f','a'], 2),
                (vec!['f','b','c','a','d'], 3),
                (vec!['d','a','b'], 7),
                (vec!['c','e','f','a','b','d'], 9),
                (vec!['c','d','f','g','e','b'], 6),
                (vec!['e','a','f','b'], 4),
                (vec!['c','a','g','e','d','b'], 0),
                (vec!['a','b'], 1),
            ])
        );

        let input = vec![String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")];
        assert_eq!(decode_signal(&(parse_input(&input).get(0).expect("Empty").clone().signals)), expected)
    }

    #[test]
    fn test_signals_with_no_segment() {
        let input = vec![String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")];
        let signals: Vec<InputSignal> = parse_input(&input)
            .get(0)
            .expect("Empty")
            .clone()
            .signals;
        let signals_with_no_b = signals_with_no_segment(&signals, 'b');
        assert_eq!(signals_with_no_b, vec![vec!['g', 'c', 'd', 'f', 'a']])
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input();
        assert_eq!(solve_part_2(&input), 61229);
    }

    #[test]
    fn test_golden_master() {
        let input = input::read("data/08.txt");
        let data = parse_input(&input);
        assert_eq!(solve_part_1(&data), 456);
        // assert_eq!(solve_part_2(&data), 1);
    }
}
