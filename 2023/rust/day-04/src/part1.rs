use hashbrown::HashSet;
use rayon::prelude::*;

use aoc::AocError;

#[tracing::instrument]
pub fn process_line(line: &str) -> u32 {
    let (_id_str, rest) = line.split_once(":").unwrap();
    let (winning, ours) = rest.trim().split_once(" | ").unwrap();

    let winning_numbers: HashSet<u32> = winning
        .split_ascii_whitespace()
        .map(|num| num.trim().parse::<u32>().unwrap())
        .collect();

    let our_numbers: HashSet<u32> = ours
        .split_ascii_whitespace()
        .map(|num| num.trim().parse::<u32>().unwrap())
        .collect();

    let winning: Vec<u32> = winning_numbers
        .intersection(&our_numbers)
        .copied()
        .collect();

    if winning.len() > 0 {
        return u32::pow(2, (winning.len() as u32) - 1);
    }

    return 0;
}

#[tracing::instrument]
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
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 1";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
