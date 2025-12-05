fn main() {
    let input = parse_input();

    println!("{}", part_one(&input));
    println!("{}", part_two(input));
}

struct Input {
    ranges: Vec<(u64, u64)>,
    ingredient_ids: Vec<u64>,
}

fn parse_input() -> Input {
    let (ranges, ingredient_ids) = include_str!("../input").split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|range| {
            let (a, b) = range.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let ingredient_ids = ingredient_ids
        .lines()
        .map(|id| id.parse().unwrap())
        .collect();

    Input {
        ranges,
        ingredient_ids,
    }
}

fn part_one(input: &Input) -> usize {
    input
        .ingredient_ids
        .iter()
        .filter(|id| {
            input
                .ranges
                .iter()
                .any(|range| (range.0..=range.1).contains(id))
        })
        .count()
}

fn part_two(input: Input) -> u64 {
    let mut non_overlapping_ranges = input.ranges;

    non_overlapping_ranges.sort_by_key(|range| range.0);

    let mut i = 1;
    while i < non_overlapping_ranges.len() {
        let prev_end = non_overlapping_ranges[i - 1].1;

        if non_overlapping_ranges[i].0 <= prev_end {
            non_overlapping_ranges[i].0 = prev_end + 1;
        }

        if non_overlapping_ranges[i].0 > non_overlapping_ranges[i].1 {
            non_overlapping_ranges.remove(i);
        } else {
            i += 1;
        }
    }

    non_overlapping_ranges
        .iter()
        .map(|range| range.1 - range.0 + 1)
        .sum()
}
