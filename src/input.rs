use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read(filename: &str) -> Vec<String> {
    let mut strings = Vec::<String>::new();
    let file = File::open(filename).expect("cannot open file");
    let lines = BufReader::new(file).lines();
    for line in lines {
        match line {
            Ok(contents) => strings.push(contents),
            Err(_) => continue,
        }
    }

    strings
}

mod tests {
    use crate::input::read;

    #[test]
    fn test_read() {
        assert_eq!(
            read("data/00.txt"),
            vec!["acbde", "12345", "  .  ",]
        )
    }
}
