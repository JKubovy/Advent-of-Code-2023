use rayon::prelude::*;
use std::{collections::HashMap, sync::Mutex};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum PlantStep {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl From<&str> for PlantStep {
    fn from(value: &str) -> Self {
        match value {
            "seed" => PlantStep::Seed,
            "soil" => PlantStep::Soil,
            "fertilizer" => PlantStep::Fertilizer,
            "water" => PlantStep::Water,
            "light" => PlantStep::Light,
            "temperature" => PlantStep::Temperature,
            "humidity" => PlantStep::Humidity,
            "location" => PlantStep::Location,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Interval {
    from: u64,
    to_exclusive: u64,
    shift: i64,
}

#[derive(Debug)]
struct Mapping {
    form: PlantStep,
    to: PlantStep,
    intervals: Vec<Interval>,
}

#[derive(Debug)]
struct Plan {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl Mapping {
    fn map(&self, input: u64) -> u64 {
        self.intervals
            .iter()
            .find(|interval| input >= interval.from && input < interval.to_exclusive)
            .map(|interval| input.checked_add_signed(interval.shift).unwrap())
            .unwrap_or(input)
    }
}

fn parse_plan(input: &str) -> Plan {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .map(|line| {
            let (_, numbers) = line
                .split_once(": ")
                .expect("Wrong input file format for seeds");
            numbers
                .split_ascii_whitespace()
                .flat_map(|n| n.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .expect("No seeds number given");
    let mut maps = Vec::new();
    let last = lines.skip(1).fold(Vec::new(), |mut vec, line| {
        if line.is_empty() {
            maps.push(vec);
            return Vec::new();
        }
        vec.push(line);
        vec
    });
    maps.push(last);
    let maps = maps
        .iter()
        .map(|lines| {
            let header = lines[0];
            let numbers = &lines[1..];
            let (from, to) = header[..header.len() - 5]
                .split_once("-to-")
                .expect("Wrong map name");
            let numbers = numbers
                .iter()
                .map(
                    |line| match line.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
                        [map_to, map_from, int] => Interval {
                            from: map_from.parse::<u64>().unwrap(),
                            to_exclusive: map_from.parse::<u64>().unwrap()
                                + int.parse::<u64>().unwrap(),
                            shift: (map_to.parse::<i64>().unwrap()
                                - map_from.parse::<i64>().unwrap()),
                        },
                        _ => unimplemented!(),
                    },
                )
                .collect();
            Mapping {
                form: from.into(),
                to: to.into(),
                intervals: numbers,
            }
        })
        .collect();
    Plan {
        seeds,
        mappings: maps,
    }
}

fn first_part(input: &Plan) -> u64 {
    let mut step = PlantStep::Seed;
    let mut numbers = input.seeds.clone();
    while step != PlantStep::Location {
        let map = input
            .mappings
            .iter()
            .find(|map| map.form == step)
            .unwrap_or_else(|| panic!("Can't find mapping for {:?}", step));
        numbers = numbers.into_iter().map(|n| map.map(n)).collect::<Vec<_>>();
        step = map.to.clone();
    }
    numbers.into_iter().min().unwrap()
}

fn second_part(input: &Plan) -> u64 {
    let min = Mutex::new(u64::MAX);
    let mut maps = HashMap::new();
    for map in &input.mappings {
        maps.insert(map.form.clone(), map);
    }
    let maps = maps;

    input.seeds.chunks(2).for_each(|w| {
        (0..w[1]).into_par_iter().for_each(|i| {
            let mut step = &PlantStep::Seed;
            let mut i = w[0] + i;
            while step != &PlantStep::Location {
                let mapping = maps.get(step).unwrap();
                i = mapping.map(i);
                step = &mapping.to;
            }
            let mut guard = min.lock().unwrap();
            if i < *guard {
                *guard = i;
            }
            drop(guard);
        });
    });
    let i = *min.lock().unwrap();
    i
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    let plan = parse_plan(input);

    let first_part = first_part(&plan);
    println!("First part: {}", first_part);
    let second_part = second_part(&plan);
    println!("Second part: {}", second_part);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let data = include_str!("../inputs/test.txt");
        let plan = parse_plan(data);
        let result = first_part(&plan);
        assert_eq!(result, 35);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let plan = parse_plan(data);
        let result = first_part(&plan);
        assert_eq!(result, 510109797);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let plan = parse_plan(data);
        let result = second_part(&plan);
        assert_eq!(result, 46);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let plan = parse_plan(data);
        let result = second_part(&plan);
        assert_ne!(result, 35081694);
        assert_eq!(result, 9622622);
    }
}
