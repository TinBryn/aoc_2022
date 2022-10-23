use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

mod input;
use crate::input::{eval, Instruction};

fn main() {
    aoc_2022::set_dir!();
    let input = File::open("input.txt").map(BufReader::new).unwrap();

    let wires: HashMap<_, _> = input
        .lines()
        .map(|line| Instruction::from_str(&line.unwrap()).unwrap())
        .inspect(|inst| println!("{inst:?}"))
        .map(|Instruction { dst, src }| (dst, src))
        .collect();

    let result = eval("a", &wires);

    println!("a = {}", result);

    let input = input::Source::Value {
        value: input::Value::Imm(result),
    };

    let mut wires = wires;

    wires.insert("b".to_string(), input);

    let result = eval("a", &wires);

    println!("a = {}", result);
}
