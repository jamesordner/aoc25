use rayon::prelude::*;
use z3::{Solver, ast::Int};

fn main() {
    let machines = parse_input();

    println!("{}", part_one(&machines));
    println!("{}", part_two(&machines));
}

fn parse_input() -> Vec<Machine> {
    include_str!("../input")
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();

            let light_pattern = iter.next().unwrap();
            let light_pattern = light_pattern[1..light_pattern.len() - 1]
                .chars()
                .map(|c| c == '#')
                .collect();

            let mut toggles = Vec::new();

            for val in iter {
                if &val[..1] == "(" {
                    let indices = val[1..val.len() - 1]
                        .split(',')
                        .map(|c| c.parse().unwrap())
                        .collect();
                    toggles.push(indices);
                } else {
                    let joltage = val[1..val.len() - 1]
                        .split(',')
                        .map(|c| c.parse().unwrap())
                        .collect();
                    return Machine {
                        light_pattern,
                        toggles,
                        joltage,
                    };
                }
            }

            unreachable!()
        })
        .collect()
}

fn part_one(machines: &[Machine]) -> usize {
    machines.iter().map(lights_min_presses).sum()
}

fn part_two(machines: &[Machine]) -> u64 {
    machines.par_iter().map(joltage_min_presses).sum()
}

struct Machine {
    light_pattern: Vec<bool>,
    toggles: Vec<Vec<usize>>,
    joltage: Vec<i32>,
}

fn lights_min_presses(machine: &Machine) -> usize {
    let mut lights = machine.light_pattern.clone();
    let mut presses = 1;

    loop {
        let mut press_pattern = vec![0; presses];

        'a: loop {
            // Reset lights.
            lights.fill(false);

            // Try press pattern.
            for press in &press_pattern {
                for i in &machine.toggles[*press] {
                    lights[*i] = !lights[*i];
                }
            }

            // Check result.
            if lights == machine.light_pattern {
                return presses;
            }

            // Update press pattern.
            for press in &mut press_pattern {
                let val = *press + 1;
                if val < machine.toggles.len() {
                    *press = val;
                    continue 'a;
                } else {
                    *press = 0;
                }
            }

            break;
        }

        presses += 1;
    }
}

fn joltage_min_presses(machine: &Machine) -> u64 {
    let button_presses = (0..machine.toggles.len())
        .map(|i| Int::fresh_const(&format!("b {i}")))
        .collect::<Vec<_>>();

    let solver = Solver::new();

    for button in &button_presses {
        solver.assert(button.ge(0));
    }

    for (i, joltage) in machine.joltage.iter().enumerate() {
        solver.assert(
            machine
                .toggles
                .iter()
                .enumerate()
                .filter(|(_, button)| button.contains(&i))
                .map(|(i, _)| button_presses[i].clone())
                .reduce(|acc, val| acc + val)
                .unwrap()
                .eq(*joltage),
        );
    }

    solver
        .solutions(&button_presses, false)
        .map(|val| val.into_iter().map(|val| val.as_u64().unwrap()).sum())
        .min()
        .unwrap()
}
