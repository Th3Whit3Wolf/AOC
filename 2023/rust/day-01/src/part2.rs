use aoc::AocError;
use rayon::prelude::*;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let subline = &line[index..];
        let from_letters =
            DIGITS.into_iter().zip(1_u32..).find_map(|(digit, num)| {
                if subline.starts_with(digit) {
                    Some(num)
                } else {
                    None
                }
            });

        let result = if from_letters.is_some() {
            from_letters
        } else {
            subline.chars().next().unwrap().to_digit(10)
        };

        result
    });

    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let answer = input.par_lines().map(process_line).sum::<u32>();
    //let answer = input.lines().map(process_line).sum::<u32>();
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    /// this test case is from the real input
    /// it tests two overlapping numbers
    /// where the second number should succeed
    #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!("281", crate::part2::process(input)?);
        Ok(())
    }
}
