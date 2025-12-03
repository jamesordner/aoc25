use std::array::from_fn;

fn main() {
    let banks = parse_input();

    println!("{}", part_one(&banks));
    println!("{}", part_two(&banks));
}

fn parse_input() -> Vec<Bank> {
    include_str!("../input")
        .lines()
        .map(|line| line.as_bytes().iter().map(|val| val - b'0').collect())
        .collect()
}

fn part_one(banks: &[Bank]) -> u64 {
    banks.iter().map(bank_max::<2>).sum()
}

fn part_two(banks: &[Bank]) -> u64 {
    banks.iter().map(bank_max::<12>).sum()
}

fn bank_max<const DIGITS: usize>(bank: &Bank) -> u64 {
    // Start with last digits and work leftward.
    let mut indices: [usize; DIGITS] = from_fn(|i| bank.len() - DIGITS + i);

    // Find highest digit for first index.
    indices[0] = find_highest_digit(bank, indices[0], 0);

    // Find highest digits for rest of indices.
    for i in 1..DIGITS {
        indices[i] = find_highest_digit(bank, indices[i], indices[i - 1] + 1);
    }

    indices
        .into_iter()
        .rev()
        .enumerate()
        .map(|(pow, i)| bank[i] as u64 * 10u64.pow(pow as u32))
        .sum()
}

fn find_highest_digit(bank: &Bank, index: usize, leftmost_index: usize) -> usize {
    (leftmost_index..=index)
        .rev()
        .max_by_key(|i| bank[*i])
        .unwrap()
}

type Bank = Vec<u8>;
