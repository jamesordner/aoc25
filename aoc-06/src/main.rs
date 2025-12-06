use std::array::from_fn;

fn main() {
    let problems = parse_input();

    println!("{}", part_one(&problems));
    println!("{}", part_two(&problems));
}

#[derive(Clone, Copy)]
struct Problem {
    vals: [&'static [u8]; 4],
    op: Op,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

fn parse_input() -> Vec<Problem> {
    let mut problems = Vec::new();
    let input = include_str!("../input");

    let mut lines: [_; 4] = from_fn(|i| input.lines().nth(i).unwrap().as_bytes());
    let mut ops_line = input.lines().nth(4).unwrap().as_bytes();

    loop {
        let len = ops_line[1..]
            .iter()
            .position(|val| *val != b' ')
            .unwrap_or(ops_line.len());

        problems.push(Problem {
            vals: from_fn(|i| &lines[i][..len]),
            op: if ops_line[0] == b'+' {
                Op::Add
            } else {
                Op::Mul
            },
        });

        if len >= ops_line.len() {
            break;
        }

        for line in &mut lines {
            *line = &line[len + 1..];
        }
        ops_line = &ops_line[len + 1..];
    }

    problems
}

fn part_one(problems: &[Problem]) -> u64 {
    sum_problems(problems, |problem| {
        let vals = problem.vals.map(|val| {
            str::from_utf8(val.trim_ascii())
                .unwrap()
                .parse::<u64>()
                .unwrap()
        });

        vals.into_iter()
            .reduce(|acc, val| reduce_op(acc, val, problem.op))
            .unwrap()
    })
}

fn part_two(problems: &[Problem]) -> u64 {
    sum_problems(problems, |problem| {
        (0..problem.vals[0].len())
            .map(|col| {
                let val: [_; 4] = from_fn(|i| problem.vals[i][col]);
                str::from_utf8(val.trim_ascii())
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
            })
            .reduce(|acc, val| reduce_op(acc, val, problem.op))
            .unwrap()
    })
}

fn sum_problems<F>(problems: &[Problem], solver: F) -> u64
where
    F: Fn(&Problem) -> u64,
{
    problems.iter().map(solver).sum()
}

fn reduce_op(acc: u64, val: u64, op: Op) -> u64 {
    if op == Op::Add { acc + val } else { acc * val }
}
