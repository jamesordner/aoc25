fn main() {
    let ranges = parse_input();

    println!("{}", part_one(&ranges));
    println!("{}", part_two(&ranges));
}

fn parse_input() -> Vec<(u64, u64)> {
    include_str!("../input")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|range| {
            let mut range = range.split('-');
            let lower = range.next().unwrap().parse::<u64>().unwrap();
            let upper = range.next().unwrap().parse::<u64>().unwrap();
            (lower, upper)
        })
        .collect()
}

fn sum_invalid_ids<F>(ranges: &[(u64, u64)], is_invalid: F) -> u64
where
    F: FnMut(&u64) -> bool,
{
    ranges
        .iter()
        .flat_map(|range| range.0..=range.1)
        .filter(is_invalid)
        .sum()
}

fn part_one(ranges: &[(u64, u64)]) -> u64 {
    sum_invalid_ids(ranges, |id| {
        let id = id.to_string();
        let len = id.len();
        let (a, b) = id.split_at(len / 2);
        a == b
    })
}

fn part_two(ranges: &[(u64, u64)]) -> u64 {
    sum_invalid_ids(ranges, |id| {
        let id = id.to_string();
        let len = id.len();

        (2..=len).any(|subdivisions| {
            if len % subdivisions != 0 {
                return false;
            }

            let subdiv_len = len / subdivisions;
            let pattern = &id[..subdiv_len];
            (1..subdivisions).all(|i| pattern == &id[subdiv_len * i..subdiv_len * (i + 1)])
        })
    })
}
