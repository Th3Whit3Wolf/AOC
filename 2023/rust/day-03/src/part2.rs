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

    fn iter(&self) -> GridIter {
        GridIter::new(self)
    }

    fn sum(&self) -> u32 {
        let mut sum = 0;
        for (y, x) in self.iter() {
            if self.data[y][x] != '*' {
                continue;
            }

            let mut number_positions = vec![];

            if y > 0 {
                let above_left = (x > 0
                    && self.data[y - 1][x - 1].is_ascii_digit())
                    as usize;
                let above = self.data[y - 1][x].is_ascii_digit() as usize;
                let above_right = (x < self.width - 1
                    && self.data[y - 1][x + 1].is_ascii_digit())
                    as usize;

                let sum = above_left + above + above_right;
                if sum == 0 {
                    // Do nothing.
                } else if sum == 2 {
                    if above == 0 {
                        number_positions.push((y - 1, x - 1));
                        number_positions.push((y - 1, x + 1));
                    } else {
                        number_positions.push((y - 1, x));
                    }
                } else if sum == 3 || above == 1 {
                    number_positions.push((y - 1, x));
                } else if above_left == 1 {
                    number_positions.push((y - 1, x - 1));
                } else {
                    assert_eq!(above_right, 1);
                    number_positions.push((y - 1, x + 1));
                }
            }

            if x > 0 && self.data[y][x - 1].is_ascii_digit() {
                number_positions.push((y, x - 1));
            }
            if x < self.width - 1 && self.data[y][x + 1].is_ascii_digit() {
                number_positions.push((y, x + 1));
            }

            if y < self.height - 1 {
                let below_left = (x > 0
                    && self.data[y + 1][x - 1].is_ascii_digit())
                    as usize;
                let below = self.data[y + 1][x].is_ascii_digit() as usize;
                let below_right = (x < self.width - 1
                    && self.data[y + 1][x + 1].is_ascii_digit())
                    as usize;

                let sum = below_left + below + below_right;
                if sum == 0 {
                    // Do nothing.
                } else if sum == 2 {
                    if below == 0 {
                        number_positions.push((y + 1, x - 1));
                        number_positions.push((y + 1, x + 1));
                    } else {
                        number_positions.push((y + 1, x));
                    }
                } else if sum == 3 || below == 1 {
                    number_positions.push((y + 1, x));
                } else if below_left == 1 {
                    number_positions.push((y + 1, x - 1));
                } else {
                    assert_eq!(below_right, 1);
                    number_positions.push((y + 1, x + 1));
                }
            }

            if number_positions.len() == 2 {
                sum += number_positions
                    .iter()
                    .fold(1, |acc, (y, x)| acc * number_at(self, *y, *x));
            }
        }

        sum
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    next_index: usize,
}

impl<'a> GridIter<'a> {
    fn new(grid: &'a Grid) -> GridIter {
        GridIter {
            grid,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let idx = self.next_index;
            if idx >= self.grid.width * self.grid.height {
                return None;
            }

            let (x, y) = (idx % self.grid.width, idx / self.grid.width);
            self.next_index += 1;
            match self.grid.data[y][x] {
                '0'..='9' | '.' => {}
                _ => {
                    return Some((y, x));
                }
            }
        }
    }
}

fn number_at(grid: &Grid, y: usize, x: usize) -> u32 {
    let mut start = x;
    assert!(grid.data[y][x].is_ascii_digit());

    while start > 0 && grid.data[y][start - 1].is_ascii_digit() {
        start -= 1;
    }
    let mut end = x;
    while end < grid.width - 1 && grid.data[y][end + 1].is_ascii_digit() {
        end += 1;
    }

    let mut n = 0;
    for x in start..=end {
        n *= 10;
        n += grid.data[y][x].to_digit(10).expect("digit");
    }

    n
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
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}

// 78016152
