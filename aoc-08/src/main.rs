fn main() {
    let input = parse_input();

    let (part_one, part_two) = part_one_and_two(&input);
    println!("{part_one}");
    println!("{part_two}");
}

fn parse_input() -> Input {
    let jboxes = include_str!("../input")
        .lines()
        .map(|line| {
            let (x, line) = line.split_once(',').unwrap();
            let (y, z) = line.split_once(',').unwrap();

            [x, y, z].map(|val| val.parse().unwrap())
        })
        .collect::<Vec<Vec3>>();

    let mut sorted_connections = Vec::new();

    for (i, jbox) in jboxes.iter().enumerate() {
        for (j, other) in jboxes.iter().enumerate().skip(i + 1) {
            sorted_connections.push(Connection {
                jboxes: (i, j),
                distance_squared: distance_squared(jbox, other),
            });
        }
    }

    sorted_connections
        .sort_unstable_by(|a, b| a.distance_squared.partial_cmp(&b.distance_squared).unwrap());

    Input {
        jboxes,
        sorted_connections,
    }
}

fn part_one_and_two(input: &Input) -> (u64, u64) {
    // Part one.

    let mut circuits = Vec::from_iter((0..input.jboxes.len()).map(|i| vec![i]));

    for connection in &input.sorted_connections[..1000] {
        append_connection(&mut circuits, connection);
    }

    circuits.sort_unstable_by_key(|circuit| circuit.len());

    let part_one = circuits[circuits.len() - 3..]
        .iter()
        .map(|circuit| circuit.len().try_into().unwrap())
        .reduce(|acc, val| acc * val)
        .unwrap();

    // Part two.

    for connection in &input.sorted_connections[1000..] {
        append_connection(&mut circuits, connection);

        if circuits.len() == 1 {
            let part_two = input.jboxes[connection.jboxes.0][0] as u64
                * input.jboxes[connection.jboxes.1][0] as u64;

            return (part_one, part_two);
        }
    }

    unreachable!();
}

struct Input {
    jboxes: Vec<Vec3>,
    sorted_connections: Vec<Connection>,
}

type Vec3 = [f32; 3];

struct Connection {
    jboxes: (usize, usize),
    distance_squared: f32,
}

fn distance_squared(a: &Vec3, b: &Vec3) -> f32 {
    let x = a[0] - b[0];
    let y = a[1] - b[1];
    let z = a[2] - b[2];
    x * x + y * y + z * z
}

fn append_connection(circuits: &mut Vec<Vec<usize>>, connection: &Connection) {
    let c0 = circuits
        .iter()
        .position(|circuit| circuit.contains(&connection.jboxes.0));
    let c1 = circuits
        .iter()
        .position(|circuit| circuit.contains(&connection.jboxes.1));

    if c0.is_none() && c1.is_none() {
        // Insert new circuit.
        circuits.push(vec![connection.jboxes.0, connection.jboxes.1]);
    } else if let Some(c0) = c0
        && let Some(c1) = c1
        && c0 != c1
    {
        // Merge circuits.
        let r = c0.max(c1);
        let i = c0.min(c1);
        let mut circuit = circuits.swap_remove(r);
        circuits[i].append(&mut circuit);
    } else if let Some(c0) = c0
        && c1.is_none()
    {
        circuits[c0].push(connection.jboxes.1);
    } else if let Some(c1) = c1
        && c0.is_none()
    {
        circuits[c1].push(connection.jboxes.0);
    }
}
