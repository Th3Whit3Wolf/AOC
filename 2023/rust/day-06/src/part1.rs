use aoc::AocError;

#[tracing::instrument]
fn quadratic_formula(b: f64, c: f64) -> (f64, f64) {
    let a = 1.0;
    let root = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let x1 = (-b + root) / 2.0 * a;
    let x2 = (-b - root) / 2.0 * a;

    (x1, x2)
}

#[tracing::instrument]
fn solve(b: f64, c: f64) -> (usize, usize) {
    let a = 1.0;
    let root = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let x1 = (-b + root) / 2.0 * a;
    let x2 = (-b - root) / 2.0 * a;

    ((x1 - 1.0).ceil() as usize, (x2 + 1.0).floor() as usize)
}

// distance = hold_time * (time - hold_time)
// -> distance_to_beat < hold_time * (time - hold_time)

// d = h * (t - h) (h=1.7)
// d/h = t-h
// d/h + h = t
// d + x^2 = th
// h^2 - th + d = 0

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (times_str, distances_str) = input.split_once("\n").unwrap();

    Ok(times_str
        .split_at(10)
        .1
        .split_ascii_whitespace()
        .map(|n| fast_float::parse::<f64, _>(n).unwrap())
        .zip(
            distances_str
                .split_at(10)
                .1
                .split_ascii_whitespace()
                .map(|n| fast_float::parse::<f64, _>(n).unwrap()),
        )
        .map(|(time, distance_to_beat)| {
            let (max_hold_time, min_hold_time) =
                quadratic_formula(-(time as f64), distance_to_beat as f64);
            (
                (max_hold_time - 1.0).ceil() as usize,
                (min_hold_time + 1.0).floor() as usize,
            )
        })
        .map(|(max_hold_time, min_hold_time)| max_hold_time - min_hold_time + 1)
        .reduce(|acc, x| acc * x)
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
