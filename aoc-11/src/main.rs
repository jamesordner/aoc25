use std::collections::HashMap;

fn main() {
    let device_map = parse_input();
    let reverse_map = reverse_map(&device_map);

    println!("{}", part_one(&device_map, &reverse_map));
    println!("{}", part_two(&device_map, &reverse_map));
}

fn parse_input() -> DeviceMap {
    include_str!("../input")
        .lines()
        .map(|line| {
            let (key, values) = line.split_once(':').unwrap();
            let values = values.split_whitespace().collect();
            (key, values)
        })
        .collect()
}

fn part_one(device_map: &DeviceMap, reverse_map: &DeviceMap) -> u64 {
    count_paths("you", "out", device_map, reverse_map)
}

fn part_two(device_map: &DeviceMap, reverse_map: &DeviceMap) -> u64 {
    let svr_to_fft = count_paths("svr", "fft", device_map, reverse_map);
    let fft_to_dac = count_paths("fft", "dac", device_map, reverse_map);
    let dac_to_out = count_paths("dac", "out", device_map, reverse_map);

    svr_to_fft * fft_to_dac * dac_to_out
}

type DeviceMap = HashMap<&'static str, Vec<&'static str>>;

fn count_paths(from: &str, to: &str, device_map: &DeviceMap, reverse_map: &DeviceMap) -> u64 {
    let mut paths_to_dst = HashMap::from([(to, 1)]);
    let mut current_nodes = vec![to];
    let mut next_nodes = vec![];

    loop {
        for current_node in &current_nodes {
            for &next_node in &reverse_map[current_node] {
                let total = device_map[next_node]
                    .iter()
                    .map(|node| paths_to_dst.get(node).unwrap_or(&0))
                    .sum();

                if next_node == from {
                    return total;
                }

                if total > *paths_to_dst.get(next_node).unwrap_or(&0) {
                    paths_to_dst.insert(next_node, total);
                    next_nodes.push(next_node);
                }
            }
        }

        // The order of this affects the result but I'm tired.
        current_nodes.append(&mut next_nodes);
    }
}

fn reverse_map(device_map: &DeviceMap) -> DeviceMap {
    let mut reverse_map = DeviceMap::new();

    for (k, v) in device_map {
        for v in v {
            reverse_map.entry(v).or_default().push(k);
        }
    }

    reverse_map
}
