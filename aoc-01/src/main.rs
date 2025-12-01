fn main() {
    let rotations = parse_input();

    println!("{}", part_one(&rotations));
    println!("{}", part_two(&rotations));
}

fn parse_input() -> Vec<Rotation> {
    include_str!("../input")
        .lines()
        .map(|line| {
            let val = line[1..].parse::<i32>().unwrap();
            if &line[..1] == "R" { val } else { -val }
        })
        .map(Rotation)
        .collect()
}

fn part_one(rotations: &[Rotation]) -> i32 {
    let mut lock = Rotation(50);
    let mut password = 0;

    for rotation in rotations {
        lock.rotate(*rotation);

        if lock.is_zero() {
            password += 1;
        }
    }

    password
}

fn part_two(rotations: &[Rotation]) -> i32 {
    let mut lock = Rotation(50);
    let mut password = 0;

    for rotation in rotations {
        let full_turns = (rotation.0 / 100).abs();
        password += full_turns;

        let prev = lock;
        lock.rotate(*rotation);

        if !prev.is_zero()
            && (lock.is_zero()
                || (rotation.0 > 0 && lock.lock_val() < prev.lock_val())
                || (rotation.0 < 0 && lock.lock_val() > prev.lock_val()))
        {
            password += 1;
        }
    }

    password
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Rotation(i32);

impl Rotation {
    fn rotate(&mut self, val: Rotation) {
        self.0 += val.0;
    }

    fn is_zero(&self) -> bool {
        self.lock_val() == 0
    }

    fn lock_val(&self) -> i32 {
        self.0.rem_euclid(100)
    }
}
