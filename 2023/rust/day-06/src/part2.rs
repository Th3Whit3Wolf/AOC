use aoc::AocError;

#[tracing::instrument]
fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let x1 = (-b + root) / 2.0 * a;
    let x2 = (-b - root) / 2.0 * a;

    (x1, x2)
}

#[tracing::instrument]
fn solve(b: f64, c: f64) -> usize {
    let a: f64 = 1.0;
    let root = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let x1 = (-b + root) / 2.0 * a;
    let x2 = (-b - root) / 2.0 * a;

    ((x1 - 1.0).ceil() as usize) - ((x2 + 1.0).floor() as usize) + 1
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
    //let (times_str, distances_str) = input.split_once("\n").unwrap();

    let mut t = String::with_capacity(9);
    let mut d = String::with_capacity(15);
    t.push('-');

    let mut chars = input.split_at(10).1.trim().chars();

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            t.push(c);
            continue;
        }
        if c == '\n' {
            break;
        }
    }

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            d.push(c);
            continue;
        }
    }

    Ok(solve(
        fast_float::parse::<f64, _>(t).unwrap(),
        fast_float::parse::<f64, _>(d).unwrap(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
