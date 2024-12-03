fn main() {
    divan::main();
}

#[divan::bench_group(name = "Day 7: Camel Cards")]
mod day {
    use aoc_2023_07::*;

    #[divan::bench(name = "Part 1")]
    fn part1() {
        part1::process(divan::black_box(INPUT)).unwrap();
    }

    #[divan::bench(name = "Part 2")]
    fn part2() {
        part2::process(divan::black_box(INPUT)).unwrap();
    }
}
