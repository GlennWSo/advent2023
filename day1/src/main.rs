fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let values = lines.map(|line| {
        let mut numbers = line.chars().filter(|c| c.is_digit(10));
        let first = numbers.next().unwrap().to_digit(10).unwrap();
        let last = match numbers.last() {
            Some(c) => c.to_digit(10).unwrap(),
            None => first,
        };
        first * 10 + last
    });
    let res = values.sum();
    res
}
fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected: u32 = 142;
        let res = part1(input);
        assert_eq!(expected, res);
    }
}
