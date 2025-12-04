fn main() {
    let mut grid = parse_input();

    println!("{}", part_one(&grid));
    println!("{}", part_two(&mut grid));
}

fn parse_input() -> Grid {
    include_str!("../input")
        .lines()
        .map(|line| line.as_bytes().iter().map(|val| *val == b'@').collect())
        .collect()
}

fn part_one(grid: &Grid) -> usize {
    let mut count = 0;

    accessible_rolls(grid, |_| {
        count += 1;
    });

    count
}

fn part_two(grid: &mut Grid) -> usize {
    let mut removed = 0;

    loop {
        let mut accessible_coords = Vec::new();

        accessible_rolls(grid, |coord| {
            accessible_coords.push(*coord);
        });

        if accessible_coords.is_empty() {
            break;
        }

        for coord in &accessible_coords {
            grid[coord[1] as usize][coord[0] as usize] = false;
        }

        removed += accessible_coords.len();
    }

    removed
}

type Grid = Vec<Vec<bool>>;

type Coord = [isize; 2];

fn accessible_rolls<F>(grid: &Grid, mut f: F)
where
    F: FnMut(&Coord),
{
    let offsets = [
        [-1isize, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
        [1, 0],
        [1, -1],
        [0, -1],
        [-1, -1],
    ];

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    for x in 0..width {
        for y in 0..height {
            if is_roll(grid, &[x, y])
                && offsets
                    .iter()
                    .filter(|offset| is_not_roll(grid, &[x + offset[0], y + offset[1]]))
                    .count()
                    >= 5
            {
                f(&[x, y]);
            }
        }
    }
}

fn is_roll(grid: &Grid, coord: &Coord) -> bool {
    grid.get(coord[1] as usize)
        .and_then(|row| row.get(coord[0] as usize))
        .is_some_and(|pos| *pos)
}

fn is_not_roll(grid: &Grid, coord: &Coord) -> bool {
    grid.get(coord[1] as usize)
        .and_then(|row| row.get(coord[0] as usize))
        .is_none_or(|pos| !*pos)
}
