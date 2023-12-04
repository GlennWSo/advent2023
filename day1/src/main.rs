use day1::{Find, Node};
use once_cell::sync::Lazy;

fn simple_row2value(line: &str) -> u32 {
    let mut numbers = line.chars().filter(|c| c.is_ascii_digit());
    let first = numbers.next().unwrap().to_digit(10).unwrap();
    let last = match numbers.next_back() {
        Some(c) => c.to_digit(10).unwrap(),
        None => first,
    };
    first * 10 + last
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let values = lines.map(simple_row2value);

    values.sum()
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
    let tree = Node::new_tree(reverse.zip(1..10));
    tree
});

fn find_digit(chars: &mut impl Iterator<Item = char>, tree: &Node) -> Option<u32> {
    let mut node: &Node = tree;
    for c in chars {
        if let Some(v) = c.to_digit(10) {
            return Some(v);
        };
        match decend(node, c, tree) {
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
    let mut rev = chars.rev();
    let find = find_digit(&mut rev, &REV_TREE);
    let last = match find {
        Some(v) => v,
        None => first,
    };

    first * 10 + last
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let values = lines.map(row2value);

    values.sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1 sum: {}", part1(input));
    println!("part2 sum: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{find_digit, part1, row2value, REV_TREE, TREE};

    #[test]
    fn test1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected: u32 = 142;
        let res = part1(input);
        assert_eq!(expected, res);
    }

    #[test]
    fn test2() {
        let input = "two1ninez\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n";
        let expected = vec![29, 83, 13, 24, 42, 14, 76];
        let res = input.lines().map(row2value);
        for (exp, res) in res.zip(expected) {
            assert_eq!(exp, res);
        }
    }
    #[test]
    fn test3() {
        let input = "1321";
        let expected: u32 = 11;
        let res = row2value(input);
        assert_eq!(expected, res);
    }
    const NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn parse_digit_end(str: &str) -> Option<u32> {
        for (i, n) in NUMBERS.iter().enumerate() {
            if str.ends_with(n) {
                return Some(i as u32 + 1);
            }
        }
        None
    }

    fn parse_digit_start(str: &str) -> Option<u32> {
        for (i, n) in NUMBERS.iter().enumerate() {
            if str.starts_with(n) {
                return Some(i as u32 + 1);
            }
        }
        None
    }
    fn test5() {
        let input = include_str!("input.txt");
        for row in input.lines() {
            let exp_front = parse_digit_start(row);
            let res_front = find_digit(&mut row.chars(), &TREE);
            assert_eq!(exp_front, res_front);

            let exp_back = parse_digit_end(row);
            let res_back = find_digit(&mut row.chars().rev(), &REV_TREE);
            assert_eq!(exp_back, res_back);
        }
    }
}
