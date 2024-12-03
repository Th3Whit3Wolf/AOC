use aoc::AocError;

#[derive(Debug)]
struct Seeds(Vec<usize>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    src: usize,
    dst: usize,
    len: usize,
}

impl Range {
    fn contains(&self, key: &usize) -> bool {
        let lb = self.src;
        let ub = self.src + self.len;
        (lb..ub).contains(key)
    }

    fn map(&self, key: &usize) -> usize {
        assert!(self.contains(key), "key must be in range");
        self.dst + (key - self.src)
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }

    fn map(&self, key: usize) -> usize {
        // Ranges are all sorted by src; hence we can binar search over them
        // to find the range that contains the key.
        let mut lb = 0;
        let mut ub = self.ranges.len();
        while lb < ub {
            let mid = (lb + ub) / 2;
            let range = self.ranges[mid];
            if range.contains(&key) {
                return range.map(&key);
            } else if key < range.src {
                ub = mid;
            } else {
                lb = mid + 1;
            }
        }
        // At this point, we haven't found the input key in the range.
        // return the key itself.
        key
    }
}

#[derive(Debug)]
struct Maps(Vec<Map>);

impl Maps {
    fn map(&self, key: usize) -> usize {
        // map through all maps in order
        self.0.iter().fold(key, |acc, map| map.map(acc))
    }

    fn min(&self, lb: usize, ub: usize) -> usize {
        assert!(lb < ub, "range must be non-empty");

        // binary search over the map to find the minimum value
        if lb + 1 == ub {
            return self.map(lb);
        }

        let len = ub - lb;
        let value_lb = self.map(lb);
        let value_ub = self.map(ub);
        if value_ub > value_lb && value_ub - value_lb == len {
            // the map range (lb, ub) is monotonic and linear
            // hence we can return the minimum value directly
            value_lb
        } else {
            let mid = (lb + ub) / 2;
            usize::min(self.min(lb, mid), self.min(mid, ub))
        }
    }
}

struct Almanac(Seeds, Maps);

impl Almanac {
    fn new(input: &str) -> Self {
        let inp = input
            .replace("seeds: ", "")
            .replace("\n\nsoil-to-fertilizer map", "")
            .replace("\n\nfertilizer-to-water map", "")
            .replace("\n\nwater-to-light map", "")
            .replace("\n\nlight-to-temperature map", "")
            .replace("\n\ntemperature-to-humidity map", "")
            .replace("\n\nhumidity-to-location map", "");

        let (seeds_str, rest) = inp.split_once("\n").unwrap();

        let seeds = seeds_str
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut maps: Vec<Map> = Vec::with_capacity(7);

        let mut split_input = rest.split(":").into_iter();
        split_input.next();

        // maps.push(Map::new(
        let mut map_1 = split_input
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                let mut nums =
                    line.split(" ").map(|num| num.parse::<usize>().unwrap());
                let dst = nums.next().unwrap();
                let src = nums.next().unwrap();
                let len = nums.next().unwrap();
                Range { src, dst, len }
            })
            .collect::<Vec<Range>>();
        map_1.sort();
        maps.push(Map::new(map_1));

        let mut map_2 = split_input
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                let mut nums =
                    line.split(" ").map(|num| num.parse::<usize>().unwrap());
                let dst = nums.next().unwrap();
                let src = nums.next().unwrap();
                let len = nums.next().unwrap();
                Range { src, dst, len }
            })
            .collect::<Vec<Range>>();
        map_2.sort();
        maps.push(Map::new(map_2));

        let mut map_3 = split_input
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                let mut nums =
                    line.split(" ").map(|num| num.parse::<usize>().unwrap());
                let dst = nums.next().unwrap();
                let src = nums.next().unwrap();
                let len = nums.next().unwrap();
                Range { src, dst, len }
            })
            .collect::<Vec<Range>>();
        map_3.sort();
        maps.push(Map::new(map_3));

        let mut map_4 = split_input
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                let mut nums =
                    line.split(" ").map(|num| num.parse::<usize>().unwrap());
                let dst = nums.next().unwrap();
                let src = nums.next().unwrap();
                let len = nums.next().unwrap();
                Range { src, dst, len }
            })
            .collect::<Vec<Range>>();
        map_4.sort();
        maps.push(Map::new(map_4));

        let mut map_5 = split_input
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                let mut nums =
                    line.split(" ").map(|num| num.parse::<usize>().unwrap());
                let dst = nums.next().unwrap();
                let src = nums.next().unwrap();
                let len = nums.next().unwrap();
                Range { src, dst, len }
            })
            .collect::<Vec<Range>>();
        map_5.sort();
        maps.push(Map::new(map_5));

        let mut map_6 = split_input
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                let mut nums =
                    line.split(" ").map(|num| num.parse::<usize>().unwrap());
                let dst = nums.next().unwrap();
                let src = nums.next().unwrap();
                let len = nums.next().unwrap();
                Range { src, dst, len }
            })
            .collect::<Vec<Range>>();
        map_6.sort();
        maps.push(Map::new(map_6));

        let mut map_7 = split_input
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                let mut nums =
                    line.split(" ").map(|num| num.parse::<usize>().unwrap());
                let dst = nums.next().unwrap();
                let src = nums.next().unwrap();
                let len = nums.next().unwrap();
                Range { src, dst, len }
            })
            .collect::<Vec<Range>>();
        map_7.sort();
        maps.push(Map::new(map_7));

        Almanac(Seeds(seeds), Maps(maps))
    }

    fn lowest_location_of_seed_ranges(&self) -> usize {
        let Almanac(seeds, maps) = self;
        seeds
            .0
            .chunks_exact(2)
            .enumerate()
            .map(|(_i, chunk)| {
                let seed = chunk[0];
                let len = chunk[1];

                let lb = seed;
                let ub = seed + len;
                maps.min(lb, ub)
            })
            .fold(usize::MAX, usize::min)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let almanac = Almanac::new(input);
    let answer = almanac.lowest_location_of_seed_ranges();

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

// 47909639
