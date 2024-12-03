use aoc::AocError;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const NUM_POSSIBLE_NODES: usize = 26426;
const Z: u16 = 25;

pub static mut NETWORK: [(u16, u16); NUM_POSSIBLE_NODES] =
    [(0, 0); NUM_POSSIBLE_NODES];

fn binary_gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    let shift = (u | v).trailing_zeros();
    u >>= u.trailing_zeros();
    loop {
        v >>= v.trailing_zeros();
        if u > v {
            (u, v) = (v, u);
        }
        v -= u;
        if v == 0 {
            break;
        }
    }
    u << shift
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / binary_gcd(a, b)
}

fn calc_cycle_length(
    start: u16,
    mut instructions: impl Iterator<Item = char>,
) -> usize {
    let mut current = start;
    let mut step = 0;
    while current & 0b11111 != Z {
        let (left, right) = unsafe { NETWORK[current as usize] };

        current = match instructions.next().unwrap() {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        };

        step += 1;
    }

    step
}

const fn encode_base_26(name: &str) -> u16 {
    let name = name.as_bytes();
    let mut result = 0;

    result |= (name[2] - b'A') as u16;
    result |= ((name[1] - b'A') as u16) << 5;
    result |= ((name[0] - b'A') as u16) << 10;

    result
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let instructions = instructions.chars().cycle();

    let mut current_nodes = vec![];
    nodes.lines().for_each(|line| {
        let name = &line[0..=2];
        let key = encode_base_26(&line[0..=2]);

        if name.as_bytes()[2] == b'A' {
            current_nodes.push(key);
        }

        let left = encode_base_26(&line[7..=9]);

        let right = encode_base_26(&line[12..=14]);

        unsafe {
            NETWORK[key as usize] = (left, right);
        }
    });

    Ok(current_nodes
        .par_iter()
        .map(|&node| calc_cycle_length(node, instructions.clone()))
        .reduce(|| 1, |a, b| lcm(a, b)))
}
