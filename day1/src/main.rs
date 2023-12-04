use std::iter;

use day1::{Find, Node};
use once_cell::sync::Lazy;

fn simple_row2value(line: &str) -> u32 {
    let mut numbers = line.chars().filter(|c| c.is_digit(10));
    let first = numbers.next().unwrap().to_digit(10).unwrap();
    let last = match numbers.rev().next() {
        Some(c) => c.to_digit(10).unwrap(),
        None => first,
    };
    first * 10 + last
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let values = lines.map(|line| simple_row2value(line));
    let res = values.sum();
    res
}

static TREE: Lazy<Node> = Lazy::new(|| {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    Node::new_tree(words.into_iter().zip(1..10))
});

static REV_TREE: Lazy<Node> = Lazy::new(|| {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let reverse: [String; 9] = words.map(|word| word.chars().rev().collect());
    let reverse = reverse.iter().map(|word| (word.as_str()));
    Node::new_tree(reverse.zip(1..10))
});

fn find_digit(chars: &mut impl Iterator<Item = char>, tree: &Node) -> Option<u32> {
    let mut node: &Node = tree;
    for c in chars {
        if let Some(v) = c.to_digit(10) {
            return Some(v);
        };
        match dbg!(decend(node, dbg!(c), tree)) {
            Find::Complete(v) => return Some(v as u32),
            Find::Partial(inner_node) => node = inner_node,
            Find::NoMatch => node = tree,
        }
    }
    None
}

fn decend<'a>(node: &'a Node, c: char, root: &'a Node) -> Find<'a> {
    match node.decend(c) {
        a @ Find::Complete(_) | a @ Find::Partial(_) => a,
        Find::NoMatch => {
            // println! {"debugging 58 {c} {:#?}", node};
            if node.is_root() {
                return Find::NoMatch;
            }
            decend(root, c, root)
        }
    }
}
fn row2value(line: &str) -> u32 {
    let mut chars = line.chars();
    let first = find_digit(&mut chars, &TREE).expect("each line should have atleast one value");
    let mut rev = dbg!(chars.rev());
    let find = find_digit(&mut rev, &REV_TREE);
    let last = match find {
        Some(v) => v,
        None => {
            dbg!("nothing found from rev, falling back to first");
            first
        }
    };

    first * 10 + last
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let values = lines.map(|line| row2value(line));
    let res = values.sum();
    res
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1 sum: {}", part1(input));
    println!("part2 sum: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, row2value};

    #[test]
    fn test1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected: u32 = 142;
        let res = part1(input);
        assert_eq!(expected, res);
    }

    #[test]
    fn test2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let expected: u32 = 281;
        let res = part2(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test3() {
        let input = "seven5pqrstsixteen";
        let expected: u32 = 76;
        let res = row2value(input);
        assert_eq!(expected, res);
    }
}
