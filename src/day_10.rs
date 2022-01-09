use crate::input;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Line {
    Valid,
    Incomplete,
    Corrupted,
}

pub fn print_solution() {
    let input = input::read("data/10.txt");

    println!("# Day 10");
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn reduce(input: &String) -> String {
    let re = Regex::new(r"\(\)|\[]|\{}|<>").unwrap();

    let mut string = String::from(input);
    loop {
        let replaced = re.replace_all(&string, "").to_string();
        if string.eq(&replaced) {
            return String::from(replaced);
        }

        string = replaced;
    }
}

fn detect(input: &String) -> Line {
    let reduced = reduce(input);
    if reduced == "" { return Line::Valid }

    let re = Regex::new(r"[)\]}>]").unwrap();
    if re.is_match(&reduced) { Line::Corrupted } else { Line::Incomplete }
}

fn missing_sequence(input: &String) -> String {
    let reduced = reduce(&input);
    reduced
        .chars()
        .rev()
        .map(|x| match x {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => '\0',
        })
        .collect::<String>()
}

fn solve_part_1(data: &Vec<String>) -> i64 {
    data
        .iter()
        .filter(|x| match detect(x) {
            Line::Corrupted => true,
            _ => false,
        })
        .map(|x| {
            let re = Regex::new(r"[)\]}>]").unwrap();
            let reduced = reduce(&x);
            let first_illegal = &re.captures(&reduced).expect("No matches")[0];
            match first_illegal {
                ")" => 3,
                "]" => 57,
                "}" => 1197,
                ">" => 25137,
                _ => 0,
            }
        })
        .sum()
}

fn solve_part_2(data: &Vec<String>) -> i64 {
    let mut scores = data
        .iter()
        .filter(|x| match detect(x) {
            Line::Incomplete => true,
            _ => false,
        })
        .map(|x| {
            missing_sequence(&x)
                .chars()
                .fold(0 as i64, |total, symbol| total * 5 + match symbol {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                })
        })
        .collect::<Vec<i64>>();
    scores.sort();

    scores[(scores.len()-1)/2]
}

mod tests {
    use crate::day_10::{detect, Line, missing_sequence, reduce, solve_part_1, solve_part_2};
    use crate::input;

    fn get_input() -> Vec<String> {
        vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ]
    }

    #[test]
    fn test_solve_part_1() {
        let input = get_input();
        assert_eq!(solve_part_1(&input), 26397);
    }

    #[test]
    fn test_solve_part_2() {
        let input = get_input();
        assert_eq!(solve_part_2(&input), 288957);
    }

    #[test]
    fn test_golden_master() {
        let input = input::read("data/10.txt");
        assert_eq!(solve_part_1(&input), 288291);
        assert_eq!(solve_part_2(&input), 820045242);
    }

    #[test]
    fn test_reduce() {
        assert_eq!(reduce(&"()".to_string()), "".to_string());
        assert_eq!(reduce(&"[]".to_string()), "".to_string());
        assert_eq!(reduce(&"{()()()}".to_string()), "".to_string());
        assert_eq!(reduce(&"<([{}])>".to_string()), "".to_string());
        assert_eq!(reduce(&"(]".to_string()), "(]".to_string());
        assert_eq!(reduce(&"{()()()>".to_string()), "{>".to_string());
        assert_eq!(reduce(&"(((()))}".to_string()), "(}".to_string());
        assert_eq!(reduce(&"<([]){()}[{}])".to_string()), "<)".to_string());
    }

    #[test]
    fn test_detect() {
        assert_eq!(detect(&"{()()()}".to_string()), Line::Valid);
        assert_eq!(detect(&"[({(<(())[]>[[{[]{<()<>>".to_string()), Line::Incomplete);
        assert_eq!(detect(&"{([(<{}[<>[]}>{[]{[(<()>".to_string()), Line::Corrupted);
    }

    #[test]
    fn test_missing_sequence() {
        assert_eq!(missing_sequence(&"[({(<(())[]>[[{[]{<()<>>".to_string()), "}}]])})]".to_string());
        assert_eq!(missing_sequence(&"[(()[<>])]({[<{<<[]>>(".to_string()), ")}>]})".to_string());
        assert_eq!(missing_sequence(&"(((({<>}<{<{<>}{[]{[]{}".to_string()), "}}>}>))))".to_string());
        assert_eq!(missing_sequence(&"{<[[]]>}<{[{[{[]{()[[[]".to_string()), "]]}}]}]}>".to_string());
        assert_eq!(missing_sequence(&"<{([{{}}[<[[[<>{}]]]>[]]".to_string()), "])}>".to_string());
    }
}
