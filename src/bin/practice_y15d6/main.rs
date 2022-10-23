use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    aoc_2022::set_dir!();
    let input = BufReader::new(File::open("input.txt").unwrap());

    let commands: Vec<_> = input
        .lines()
        .map(|line| Instruction::from_string(&line.unwrap()).unwrap())
        // .inspect(|inst| println!("{:?}", inst))
        .collect();

    let mut grid = vec![0; 1_000_000];

    for inst in commands {
        match inst {
            Instruction::TurnOn { range } => {
                for x in range.from.0..=range.to.0 {
                    for y in range.from.1..=range.to.1 {
                        let idx = x + y * 1000;
                        grid[idx] += 1;
                    }
                }
            }
            Instruction::TurnOff { range } => {
                for x in range.from.0..=range.to.0 {
                    for y in range.from.1..=range.to.1 {
                        let idx = x + y * 1000;
                        grid[idx] = usize::saturating_sub(grid[idx], 1);
                    }
                }
            }
            Instruction::Toggle { range } => {
                for x in range.from.0..=range.to.0 {
                    for y in range.from.1..=range.to.1 {
                        let idx = x + y * 1000;
                        grid[idx] += 2;
                    }
                }
            }
        }
    }
    let count: usize = grid.into_iter().sum();

    println!("The light brightness is {}", count);
}

#[derive(Debug)]
enum Instruction {
    TurnOn { range: LightRange },
    TurnOff { range: LightRange },
    Toggle { range: LightRange },
}

impl Instruction {
    fn from_string(line: &str) -> Option<Self> {
        if let Some(range) = line.strip_prefix("turn on") {
            let range = LightRange::from_string(range.trim())?;
            Some(Self::TurnOn { range })
        } else if let Some(range) = line.strip_prefix("turn off") {
            let range = LightRange::from_string(range.trim())?;
            Some(Self::TurnOff { range })
        } else if let Some(range) = line.strip_prefix("toggle") {
            let range = LightRange::from_string(range.trim())?;
            Some(Self::Toggle { range })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct LightRange {
    from: (usize, usize),
    to: (usize, usize),
}

impl LightRange {
    fn from_string(line: &str) -> Option<Self> {
        let (from, to) = line.split_once("through")?;

        let from = Self::point_from_string(from.trim())?;
        let to = Self::point_from_string(to.trim())?;

        Some(Self { from, to })
    }

    fn point_from_string(line: &str) -> Option<(usize, usize)> {
        let (x, y) = line.split_once(',')?;
        let x = x.trim().parse().ok()?;
        let y = y.trim().parse().ok()?;
        Some((x, y))
    }
}
