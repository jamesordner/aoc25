fn main() {
    let mut tiles = parse_input();

    println!("{}", part_one(&tiles));

    // Avoid wraparound indexing for part two.
    tiles.push(tiles[0]);

    println!("{}", part_two(&tiles));
}

fn parse_input() -> Vec<Coord> {
    include_str!("../input")
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x, y].map(|val| val.parse().unwrap())
        })
        .collect()
}

fn part_one(tiles: &[Coord]) -> isize {
    max_area(tiles, |_, _| true)
}

fn part_two(tiles: &[Coord]) -> isize {
    max_area(tiles, |tile, other| {
        let x0 = tile[0].min(other[0]);
        let x1 = tile[0].max(other[0]);
        let y0 = tile[1].min(other[1]);
        let y1 = tile[1].max(other[1]);
        !border_inside_rect(tiles, &[x0, y0], &[x1, y1])
    })
}

type Coord = [isize; 2];

fn max_area<F>(tiles: &[Coord], mut filter: F) -> isize
where
    F: FnMut(&Coord, &Coord) -> bool,
{
    tiles
        .iter()
        .enumerate()
        .map(|(i, tile)| {
            tiles[i..]
                .iter()
                .filter(|coord| filter(tile, coord))
                .map(|other| ((tile[0] - other[0]).abs() + 1) * ((tile[1] - other[1]).abs() + 1))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

/// This is so inefficient lol.
fn border_inside_rect(tiles: &[Coord], a: &Coord, b: &Coord) -> bool {
    tiles.windows(2).any(|line| {
        if line[0][0] == line[1][0] {
            // Vertical.
            let y0 = line[0][1].min(line[1][1]);
            let y1 = line[0][1].max(line[1][1]);
            (y0..=y1)
                .map(|y| [line[0][0], y])
                .any(|p| point_inside_rect(&p, a, b))
        } else {
            // Vertical.
            let x0 = line[0][0].min(line[1][0]);
            let x1 = line[0][0].max(line[1][0]);
            (x0..=x1)
                .map(|x| [x, line[0][1]])
                .any(|p| point_inside_rect(&p, a, b))
        }
    })
}

fn point_inside_rect(point: &Coord, a: &Coord, b: &Coord) -> bool {
    a[0] < point[0] && point[0] < b[0] && a[1] < point[1] && point[1] < b[1]
}
