use aoc::AocError;

struct Grid {
    pub data: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    fn new(s: &str) -> Grid {
        let mut data = vec![];
        #[cfg(test)]
        let height = 10;
        #[cfg(not(test))]
        let height = 139;
        #[cfg(test)]
        let width = 10;
        #[cfg(not(test))]
        let width = 140;
        for line in s.lines() {
            data.push(line.chars().collect());
        }

        for line in s.lines() {
            data.push(line.chars().collect());
        }

        Grid {
            data,
            height,
            width,
        }
    }

    fn sum(&self) -> u32 {
        let mut sum = 0;

        for y in 0..self.height {
            let mut x = 0;
            while x < self.width {
                match self.data[y][x] {
                    '.' => {}
                    '0'..='9' => {
                        let begin = x;
                        while x < self.width && self.data[y][x].is_ascii_digit()
                        {
                            x += 1;
                        }

                        let end = x - 1;

                        let before_x = begin.saturating_sub(1);
                        let after_x = (end + 1).min(self.width - 1);

                        if (y > 0
                            && (before_x..=after_x)
                                .any(|x| is_symbol(self.data[y - 1][x])))
                            || (y < self.height - 1
                                && (before_x..=after_x)
                                    .any(|x| is_symbol(self.data[y + 1][x])))
                            || (begin > 0 && is_symbol(self.data[y][begin - 1]))
                            || (end < self.width - 1
                                && is_symbol(self.data[y][end + 1]))
                        {
                            let mut n = 0;
                            for x in begin..=end {
                                n *= 10;
                                n += self.data[y][x]
                                    .to_digit(10)
                                    .expect("digit");
                            }

                            sum += n;
                            continue;
                        }
                    }
                    _ => {}
                }

                x += 1;
            }
        }

        sum
    }
}

fn is_symbol(c: char) -> bool {
    match c {
        '0'..='9' | '.' => false,
        _ => true,
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = Grid::new(input);
    let answer = grid.sum();
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
