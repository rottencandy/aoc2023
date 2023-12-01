fn solve_part1(input: &str) -> u32 {
    input
        .split_whitespace()
        .map(|line| {
            let nums = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            match nums.len() {
                1 => nums[0] * 10 + nums[0],
                _ => nums[0] * 10 + nums.last().unwrap(),
            }
        })
        .sum::<u32>()
}

fn solve_part2(input: &str) -> u32 {
    input
        .split_whitespace()
        .map(|line| {
            let mut digits: Vec<u32> = vec![];

            for i in 0..line.len() {
                if line.chars().nth(i).is_some_and(|c| c.is_ascii_digit()) {
                    digits.push(line.chars().nth(i).unwrap_or('0').to_digit(10).unwrap());
                }
                for (key, value) in DIGITS.iter().enumerate() {
                    if line[i..].starts_with(value) {
                        digits.push(key as u32);
                    }
                }
            }

            let first_digit = digits.iter().nth(0).unwrap_or(&0);
            let last_digit = digits.iter().nth_back(0).unwrap_or(&0);

            first_digit * 10 + last_digit
        })
        .sum::<u32>()
}

pub fn run() {
    let input = include_str!("../inputs/day1a");
    let result_p1 = solve_part1(input);
    let result_p2 = solve_part2(input);
    println!("part 1 result: {}", result_p1);
    println!("part 2 result: {}", result_p2);
}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}
