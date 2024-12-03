use aoc::AocError;
use rayon::prelude::*;
use std::collections::VecDeque;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut input = input.split("\n\n");
    let seeds = input.next().unwrap()[7..]
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut new_seeds = Vec::new();
    for range in seeds.chunks_exact(2).clone() {
        let start = range[0];
        let length = range[1];
        new_seeds.push((start, start + length));
    }
    let mut seeds = new_seeds;

    for map in input {
        let ranges = map
            .lines()
            .skip(1)
            .map(|l| {
                let mut range = l.split_whitespace();
                let target = range.next().unwrap().parse::<u64>().unwrap();
                let lower = range.next().unwrap().parse::<u64>().unwrap();
                let lenght = range.next().unwrap().parse::<u64>().unwrap();
                (target, lower, lower + lenght)
            })
            .collect::<Vec<_>>();

        let mut seed_queue = VecDeque::new();
        seed_queue.extend(seeds.into_iter());

        let mut new_seeds = Vec::new();

        while let Some((start, end)) = seed_queue.pop_front() {
            let mut found = false;

            for &(target, lower, upper) in &ranges {
                if start >= lower && start < upper && end < upper {
                    // Complete range is in bounds
                    let s = start + target - lower;
                    let e = end + target - lower;
                    new_seeds.push((s, e));
                    found = true;
                    break;
                } else if start >= lower && start < upper {
                    // Start is in bounds, end is not
                    let s = start + target - lower;
                    let e = upper - 1 + target - lower;
                    new_seeds.push((s, e));
                    seed_queue.push_back((upper, end));
                    found = true;
                    break;
                } else if start < lower && end >= lower && end < upper {
                    // End is in bounds, start is not
                    let s = lower + target - lower;
                    let e = end + target - lower;
                    new_seeds.push((s, e));
                    seed_queue.push_back((start, lower - 1));
                    found = true;
                    break;
                } else if start < lower && end >= upper {
                    // Neither start nor end are in bounds
                    new_seeds.push((
                        lower + target - lower,
                        upper - 1 + target - lower,
                    ));
                    seed_queue.push_back((upper, end));
                    seed_queue.push_back((start, lower - 1));
                    found = true;
                    break;
                }
            }

            if !found {
                // No overlap with any range
                new_seeds.push((start, end));
            }
        }

        seeds = new_seeds;
    }

    let answer = seeds.par_iter().map(|s| s.0).min().unwrap();

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
