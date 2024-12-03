fn main() {
    divan::main();
}

#[divan::bench_group(name = "Day 6: Wait For It")]
mod day {
    use aoc_2023_06::*;

    #[divan::bench(name = "Part 1")]
    fn part1() {
        part1::process(divan::black_box(INPUT)).unwrap();
    }

    #[divan::bench(name = "Part 2")]
    fn part2() {
        part2::process(divan::black_box(INPUT)).unwrap();
    }
}
