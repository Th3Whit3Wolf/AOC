use aoc::AocError;
use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;

#[tracing::instrument]
pub fn process_line(line: &str) -> (u32, usize) {
    let (id_str, rest) = line.split_once(":").unwrap();
    let id = id_str.split_at(5).1.trim().parse::<u32>().unwrap();
    let (winning, ours) = rest.trim().split_once(" | ").unwrap();

    let winning_numbers: HashSet<u32> = winning
        .split_ascii_whitespace()
        .map(|num| num.trim().parse::<u32>().unwrap())
        .collect();

    let our_numbers: HashSet<u32> = ours
        .split_ascii_whitespace()
        .map(|num| num.trim().parse::<u32>().unwrap())
        .collect();

    return (id, winning_numbers.intersection(&our_numbers).count());
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let cards: Vec<(u32, usize)> =
        input.par_lines().map(process_line).collect();
    let store = (0..cards.len())
        .map(|index| (index, 1))
        .collect::<HashMap<usize, u32>>();

    let answer = cards
        .iter()
        .fold(store, |mut acc, (game_id, score)| {
            let index: usize = *game_id as usize - 1;
            let to_add = *acc.get(&index).unwrap();

            for i in
                (*game_id as usize)..((*game_id as usize + *score) as usize)
            {
                acc.entry(i).and_modify(|value| {
                    *value += to_add;
                });
            }
            acc
        })
        .values()
        .sum::<u32>();

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
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
