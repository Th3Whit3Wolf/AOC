#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left: Vec<i32> = Vec::with_capacity(1000);
    let mut right: Vec<i32> = Vec::with_capacity(1000);

    for l in input.lines() {
        let s = l.split_once("   ").unwrap();
        left.push(s.0.parse::<i32>().unwrap());
        right.push(s.1.parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());

    let mut total = 0;
    for i in 0..left.len() {
        let value = (left[i] - right[i]).abs();
        total += value
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
