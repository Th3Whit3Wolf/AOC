use aoc::AocError;
use rayon::prelude::*;

fn process_line(line: &str) -> u32 {
    let mut it = line.chars().filter_map(|character| character.to_digit(10));
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        // If there is only one number then reuse it
        None => first * 10 + first,
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let answer = input.par_lines().map(process_line).sum::<u32>();

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
