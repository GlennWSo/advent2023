use day1::Tree;

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

fn find_digit(chars: &mut impl Iterator<Item = char>, tree: &mut Tree) -> Option<u32> {
    chars.find_map(|c| match c.to_digit(10) {
        Some(v) => Some(v),
        None => tree.decend(c).map(|v| v as u32),
    })
}

fn row2value(line: &str, tree: &mut Tree, rev_tree: &mut Tree) -> u32 {
    let mut chars = line.chars();
    let first = find_digit(&mut chars, tree).expect("each line should have atleast one value");
    let mut rev = chars.rev();
    let find = find_digit(&mut rev, rev_tree);
    let last = match find {
        Some(v) => v,
        None => first,
    };

    first * 10 + last
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut tree = Tree::digits();
    let mut rev_tree = Tree::rev_digits();
    let values = lines.map(|row| row2value(row, &mut tree, &mut rev_tree));

    values.sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1 sum: {}", part1(input));
    println!("part2 sum: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use day1::Tree;

    use crate::{find_digit, part1, row2value};

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
        let mut tree = Tree::digits();
        let mut rev_tree = Tree::rev_digits();
        let res = input
            .lines()
            .map(|row| row2value(row, &mut tree, &mut rev_tree));
        for (exp, res) in res.zip(expected) {
            assert_eq!(exp, res);
        }
    }
    #[test]
    fn test3() {
        let input = "3onen";
        let expected: u32 = 31;
        let mut tree = Tree::digits();
        let mut rev_tree = Tree::rev_digits();
        let res = row2value(input, &mut tree, &mut rev_tree);
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
    #[test]
    fn test5() {
        let input = include_str!("input.txt");
        let mut tree = Tree::digits();
        let mut rev_tree = Tree::rev_digits();
        for row in input.lines() {
            let exp_front = find_first(&mut row.chars());
            let res_front = find_digit(&mut row.chars(), &mut tree).unwrap();
            assert_eq!(exp_front, res_front, "row is {row}");

            let exp_back = find_last(row.chars());
            let res_back = find_digit(&mut row.chars().rev(), &mut rev_tree).unwrap();
            assert_eq!(exp_back, res_back, "row is {row}");
        }
    }

    fn find_last(mut chars: std::str::Chars<'_>) -> u32 {
        let mut last = 0;
        loop {
            if let Some(o) = parse_digit_end(chars.as_str()) {
                last = o;
                break;
            }

            if let Some(ch) = chars.next_back() {
                if ch.is_ascii_digit() {
                    last = ch.to_digit(10).unwrap();
                    break;
                }
            } else {
                break;
            }
        }
        last
    }

    fn find_first(chars: &mut std::str::Chars<'_>) -> u32 {
        let mut first = 0;
        loop {
            if let Some(o) = parse_digit_start(chars.as_str()) {
                first = o;
                break;
            }

            if let Some(ch) = chars.next() {
                if ch.is_ascii_digit() {
                    first = ch.to_digit(10).unwrap();
                    break;
                }
            } else {
                break;
            }
        }
        first
    }
}
