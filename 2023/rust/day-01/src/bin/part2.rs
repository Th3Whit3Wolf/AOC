use aoc_2023_01::{part2::process, INPUT};
use miette::Context;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let result = process(INPUT).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
