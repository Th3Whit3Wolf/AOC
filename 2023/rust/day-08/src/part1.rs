use aoc::AocError;
use rayon::prelude::*;

const NUM_POSSIBLE_NODES: usize = 26426;
const ZZZ: u16 = 26425;
const AAA: u16 = 0;

pub static mut NETWORK: [(u16, u16); NUM_POSSIBLE_NODES] =
    [(0, 0); NUM_POSSIBLE_NODES];

const fn encode_base_26(name: &str) -> u16 {
    let name = name.as_bytes();
    let mut result = 0;
    result |= (name[2] - b'A') as u16;
    result |= ((name[1] - b'A') as u16) << 5;
    result |= ((name[0] - b'A') as u16) << 10;
    result
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let mut instructions = instructions.chars().cycle();

    nodes.par_lines().for_each(|line| {
        let name = encode_base_26(&line[0..=2]);
        let left = encode_base_26(&line[7..=9]);
        let right = encode_base_26(&line[12..=14]);
        unsafe {
            NETWORK[name as usize] = (left, right);
        }
    });

    let mut current = AAA;
    let mut step = 0;
    while current != ZZZ {
        let (left, right) = unsafe { NETWORK[current as usize] };

        let instruction = instructions.next().unwrap();
        current = match instruction {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        };

        step += 1;
    }

    Ok(step)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        2
    )]
    fn test_process(
        #[case] input: &str,
        #[case] expected: i32,
    ) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
