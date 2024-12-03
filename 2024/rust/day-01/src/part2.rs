use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut map: HashMap<u32,u32> = HashMap::with_capacity(1000);
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: u32 = 0;

    for l in input.lines() {
        let s = l.split_once("   ").unwrap();
        right = s.1.parse::<u32>().unwrap();
        left.push(s.0.parse::<u32>().unwrap());

        if let Some(v) = map.get_mut(&right) {
            *v += 1;
        } else {
            map.insert(right, 1);
        }
    }


    let mut total = 0;

    for num in left {
        if let Some(v) = map.get(&num) {
            total += v*num
        }
    }


    println!("map: {:#?}", map);

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
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
