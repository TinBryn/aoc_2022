use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    let input = aoc_2022::input!().map(BufReader::new).unwrap();

    let graph: Graph = input
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    for (start, dests) in graph.map {
        println!("{start} -> {dests:?}")
    }
}

struct Graph {
    map: HashMap<String, HashMap<String, usize>>,
}

impl FromIterator<Link> for Graph {
    fn from_iter<T: IntoIterator<Item = Link>>(iter: T) -> Self {
        let mut map: HashMap<String, HashMap<String, usize>> = HashMap::new();
        for Link {
            start,
            end,
            distance,
        } in iter
        {
            map.entry(start.clone())
                .or_default()
                .insert(end.clone(), distance);
            map.entry(end).or_default().insert(start, distance);
        }
        Self { map }
    }
}

struct Link {
    start: String,
    end: String,
    distance: usize,
}

impl FromStr for Link {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end, distance) = s
            .trim()
            .split_once('=')
            .and_then(|(a, distance)| {
                a.trim().split_once("to").map(|(start, end)| {
                    (start.trim(), end.trim(), distance.trim())
                })
            })
            .ok_or(())?;
        let distance = distance.parse().or(Err(()))?;
        let start = start.into();
        let end = end.into();

        Ok(Self {
            start,
            end,
            distance,
        })
    }
}
