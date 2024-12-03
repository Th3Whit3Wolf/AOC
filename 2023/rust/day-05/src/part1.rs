use aoc::AocError;
use rayon::prelude::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut input = input.split("\n\n");
    let mut seeds = input.next().unwrap()[7..]
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

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

        seeds.iter_mut().for_each(|seed| {
            for &(target, lower, upper) in &ranges {
                if *seed >= lower && *seed < upper {
                    *seed = *seed + target - lower;
                    break;
                }
            }
        });
    }

    let answer = *seeds.par_iter().min().unwrap();

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
// 226172555
