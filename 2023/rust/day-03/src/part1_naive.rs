use aoc::AocError;
use hashbrown::HashMap;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Number {
    value: u32,
    start: u32,
    end: u32,
    part_number: bool,
}

impl Number {
    fn has_position(&self, position: &u32) -> bool {
        position >= &self.start && position <= &self.end
    }

    fn is_part_number(&mut self) {
        self.part_number = true
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
            part_number: false,
        };
        let line_numbers = self.0.get_mut(&(y as u32)).unwrap();
        line_numbers.push(number);
    }

    fn check_part_number(&mut self, symbols: Vec<Symbol>) {
        symbols.iter().for_each(|symbol| {
            let prev_numbers = self.0.get_mut(&(symbol.line as u32 - 1));
            if prev_numbers.is_some() {
                let nums = prev_numbers.unwrap();
                *nums = nums
                    .iter_mut()
                    .map(|num| {
                        if num.has_position(&(symbol.position - 1))
                            || num.has_position(&symbol.position)
                            || num.has_position(&(symbol.position + 1))
                        {
                            num.is_part_number();
                        }
                        num.to_owned()
                    })
                    .collect::<Vec<Number>>();
            }

            let curr_numbers = self.0.get_mut(&symbol.line);

            if curr_numbers.is_some() {
                let nums = curr_numbers.unwrap();
                *nums = nums
                    .iter_mut()
                    .map(|num| {
                        if num.has_position(&(symbol.position - 1))
                            || num.has_position(&(symbol.position + 1))
                        {
                            num.is_part_number();
                        }
                        num.to_owned()
                    })
                    .collect::<Vec<Number>>();
            }

            let next_numbers = self.0.get_mut(&(symbol.line as u32 + 1));
            if next_numbers.is_some() {
                let nums = next_numbers.unwrap();
                *nums = nums
                    .iter_mut()
                    .map(|num| {
                        if num.has_position(&(symbol.position - 1))
                            || num.has_position(&symbol.position)
                            || num.has_position(&(symbol.position + 1))
                        {
                            num.is_part_number();
                        }
                        num.to_owned()
                    })
                    .collect::<Vec<Number>>();
            }
        })
    }

    fn get_part_numbers(&self) -> Vec<u32> {
        self.0
            .values()
            .flatten()
            .filter(|&&number| number.part_number)
            .map(|number| number.value)
            .collect::<Vec<u32>>()
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

    numbers.check_part_number(symbols.get());
    let answer = numbers.get_part_numbers().par_iter().sum::<u32>();

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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
