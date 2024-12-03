use aoc::AocError;
use core::fmt;
use itertools::Itertools;

const LABEL_TO_STRENGTH: [u32; 36] =
    generate_label_to_strength_lookup(b"J23456789TQKA");

pub fn str_to_u32(bid: &str) -> u32 {
    let mut val = 0u32;

    for c in bid.bytes() {
        val = val * 10 + (c - b'0') as u32;
    }

    val
}

const fn generate_label_to_strength_lookup(
    label_order: &[u8; 13],
) -> [u32; 36] {
    let mut map = [u32::MAX; 36];

    let mut i = 0;
    while i < 13 {
        map[(label_order[i] - b'2') as usize] = i as u32;
        i += 1;
    }

    map
}

fn labels_to_hand_strength(labels: &str) -> u32 {
    let kind = HandKind::from_labels(labels);

    labels
        .bytes()
        .rev()
        .map(|l| LABEL_TO_STRENGTH[l as usize - b'2' as usize])
        .enumerate()
        .map(|(i, s)| (s as u32) << (i << 2)) // i * 4
        .sum::<u32>()
        | kind as u32
}

#[derive(PartialEq, Eq, Ord)]
struct Hand {
    kind: HandKind,
    strength: u64,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.kind
                .cmp(&other.kind)
                .then(self.strength.cmp(&other.strength)),
        )
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hand {{ kind: {:?}, strength: 0x{:010x} }}",
            self.kind, self.strength
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u32)]
pub enum HandKind {
    HighCard = 0 << 5 * 4,
    OnePair = 1 << 5 * 4,
    TwoPairs = 2 << 5 * 4,
    ThreeOfAKind = 3 << 5 * 4,
    FullHouse = 4 << 5 * 4,
    FourOfAKind = 5 << 5 * 4,
    FiveOfAKind = 6 << 5 * 4,
}

impl HandKind {
    fn from_labels(labels: &str) -> Self {
        let mut counts = [0; 13];
        let mut max = 0;
        let mut max_idx = 0usize;

        for label in labels.chars() {
            let strength =
                LABEL_TO_STRENGTH[label as usize - b'2' as usize] as usize;
            let count = counts[strength] + 1;

            if count >= max && label != 'J' {
                max = count;
                max_idx = strength;
            }

            counts[strength] = count;
        }

        let joker_strength =
            LABEL_TO_STRENGTH[b'J' as usize - (b'2' as usize)] as usize;
        let joker_count = counts[joker_strength];
        counts[joker_strength] = 0;
        counts[max_idx] += joker_count;

        let mut pairs = 0;
        let mut tripple = false;
        for c in counts {
            match c {
                5 => return HandKind::FiveOfAKind,
                4 => return HandKind::FourOfAKind,
                3 => tripple = true,
                2 => pairs += 1,
                _ => (),
            }
        }

        match pairs {
            0 => {
                if tripple {
                    HandKind::ThreeOfAKind
                } else {
                    HandKind::HighCard
                }
            }
            1 => {
                if tripple {
                    HandKind::FullHouse
                } else {
                    HandKind::OnePair
                }
            }
            2 => HandKind::TwoPairs,
            _ => panic!("Invalid number of pairs"),
        }
    }
}

fn process_line(line: &str) -> (u32, u32) {
    let hand_strength = labels_to_hand_strength(&line[..5]);
    let bet = str_to_u32(&line[6..]);
    (hand_strength, bet)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(input
        .lines()
        .map(|line| process_line(line))
        .sorted_by_cached_key(|(hand_strength, _)| *hand_strength)
        .zip(1..)
        .map(|((_, bet), i)| i * bet)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(5905, process(input)?);
        Ok(())
    }
}
