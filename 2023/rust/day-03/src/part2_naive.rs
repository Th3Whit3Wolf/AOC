use aoc::AocError;
use hashbrown::HashMap;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Number {
    value: u32,
    start: u32,
    end: u32,
}

impl Number {
    fn has_position(&self, position: &u32) -> bool {
        position >= &self.start && position <= &self.end
    }
}

#[derive(Debug, Copy, Clone)]
struct Symbol {
    line: u32,
    position: u32,
}

struct Numbers(HashMap<u32, Vec<Number>>);

impl Numbers {
    fn new() -> Self {
        Numbers(HashMap::new())
    }

    fn add_line(&mut self, y: usize) {
        self.0.insert(y as u32, Vec::with_capacity(6));
    }

    fn add(&mut self, y: usize, x: usize, num: u32) {
        let x_offset: u32 = if num > 1000 {
            4
        } else if num > 100 {
            3
        } else if num > 10 {
            2
        } else {
            1
        };
        let start = x as u32 - x_offset;
        let number = Number {
            value: num,
            start,
            end: x as u32 - 1_u32,
        };
        let line_numbers = self.0.get_mut(&(y as u32)).unwrap();
        line_numbers.push(number);
    }

    fn get_gear_ratio(&mut self, symbols: Vec<Symbol>) -> Vec<u64> {
        symbols
            .iter()
            .map(|symbol| {
                let mut prev_numbers =
                    if let Some(nums) = self.0.get(&(symbol.line as u32 - 1)) {
                        nums.into_iter()
                            .filter(|&num| {
                                num.has_position(&(symbol.position - 1))
                                    || num.has_position(&symbol.position)
                                    || num.has_position(&(symbol.position + 1))
                            })
                            .copied()
                            .collect::<Vec<Number>>()
                    } else {
                        Vec::new()
                    };

                let mut curr_numbers =
                    if let Some(nums) = self.0.get(&symbol.line) {
                        nums.into_iter()
                            .filter(|&num| {
                                num.has_position(&(symbol.position - 1))
                                    || num.has_position(&(symbol.position + 1))
                            })
                            .copied()
                            .collect::<Vec<Number>>()
                    } else {
                        Vec::new()
                    };

                let mut next_numbers =
                    if let Some(nums) = self.0.get(&(symbol.line as u32 + 1)) {
                        nums.into_iter()
                            .filter(|&num| {
                                num.has_position(&(symbol.position - 1))
                                    || num.has_position(&symbol.position)
                                    || num.has_position(&(symbol.position + 1))
                            })
                            .copied()
                            .collect::<Vec<Number>>()
                    } else {
                        Vec::new()
                    };

                if (prev_numbers.len()
                    + curr_numbers.len()
                    + next_numbers.len())
                    == 2
                {
                    let mut gears = Vec::with_capacity(2);
                    gears.append(&mut prev_numbers);
                    gears.append(&mut curr_numbers);
                    gears.append(&mut next_numbers);

                    gears.iter().map(|n| n.value as u64).product::<u64>()
                } else {
                    0
                }
            })
            .collect::<Vec<u64>>()
    }
}

struct Symbols(Vec<Symbol>);

impl Symbols {
    fn new() -> Self {
        Self(Vec::with_capacity(723))
    }

    fn add(&mut self, y: usize, x: usize) {
        self.0.push(Symbol {
            line: y as u32,
            position: x as u32,
        })
    }

    fn get(self) -> Vec<Symbol> {
        self.0
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut numbers = Numbers::new();
    let mut symbols = Symbols::new();
    let mut number_buffer = None;

    for (y, line) in input.lines().enumerate() {
        numbers.add_line(y);
        if let Some(num) = number_buffer {
            numbers.add(y - 1, line.len() - 1, num);
            number_buffer = None;
        }
        for (x, character) in line.chars().enumerate() {
            match character {
                c if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).expect("should be a number");
                    if let Some(num) = number_buffer {
                        number_buffer = Some(num * 10 + digit)
                    } else {
                        number_buffer = Some(digit)
                    }
                }
                _ => {
                    if let Some(num) = number_buffer {
                        numbers.add(y, x, num);
                        number_buffer = None;
                    }
                    if character == '.' {
                        continue;
                    } else {
                        symbols.add(y, x);
                    }
                }
            }
        }
    }

    let answer = numbers
        .get_gear_ratio(symbols.get())
        .par_iter()
        .sum::<u64>();

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
.664.598.";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
