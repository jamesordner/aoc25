use std::collections::{HashMap, HashSet};

fn main() {
    let input = parse_input();

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input() -> Input {
    let input = include_str!("../input");

    let start = input
        .as_bytes()
        .iter()
        .position(|val| *val == b'S')
        .unwrap();

    let grid = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|val| {
                    if *val == b'^' {
                        Spot::Splitter
                    } else {
                        Spot::Empty
                    }
                })
                .collect()
        })
        .collect();

    Input {
        start: [start as isize, 0],
        grid,
    }
}

fn part_one(input: &Input) -> usize {
    let mut beams_to_trace = vec![input.start];
    let mut splitters_hit = HashSet::new();

    while let Some(beam) = beams_to_trace.pop() {
        let beam = trace_beam(&input.grid, beam);

        // We've finished tracing. Check what happened.
        if in_bounds(&input.grid, beam) {
            // Must have hit a splitter.
            if !splitters_hit.contains(&beam) {
                beams_to_trace.push([beam[0] - 1, beam[1]]);
                beams_to_trace.push([beam[0] + 1, beam[1]]);
                splitters_hit.insert(beam);
            }
        }
    }

    splitters_hit.len()
}

fn part_two(input: &Input) -> u64 {
    // Memoize split.
    let mut splitter_timelines = HashMap::<Coord, u64>::new();

    for (y, row) in input.grid.iter().enumerate().rev() {
        for (x, _) in row
            .iter()
            .enumerate()
            .filter(|(_, spot)| **spot == Spot::Splitter)
        {
            let coord = [x as isize, y as isize];

            let timelines = [[coord[0] - 1, coord[1]], [coord[0] + 1, coord[1]]]
                .map(|beam| {
                    let beam = trace_beam(&input.grid, beam);

                    if in_bounds(&input.grid, beam) {
                        splitter_timelines[&beam]
                    } else {
                        1
                    }
                })
                .iter()
                .sum();

            splitter_timelines.insert(coord, timelines);
        }
    }

    // Find first splitter hit.
    splitter_timelines[&trace_beam(&input.grid, input.start)]
}

struct Input {
    start: Coord,
    grid: Grid,
}

type Coord = [isize; 2];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spot {
    Empty,
    Splitter,
}

type Grid = Vec<Vec<Spot>>;

fn trace_beam(grid: &Grid, mut beam: Coord) -> Coord {
    while !grid
        .get(beam[1] as usize)
        .and_then(|row| row.get(beam[0] as usize))
        .is_none_or(|spot| *spot == Spot::Splitter)
    {
        beam[1] += 1;
    }

    beam
}

fn in_bounds(grid: &Grid, coord: Coord) -> bool {
    grid.get(coord[1] as usize)
        .is_some_and(|row| row.get(coord[0] as usize).is_some())
}
