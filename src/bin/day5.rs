use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input/day5.txt");

const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

const DIAGONAL_INPUT: &str = "1,1 -> 3,3";
const DIAGONAL_INPUTB: &str = "9,7 -> 7,9";

fn main() {
    println!("day 5");
    let clouds: Vec<(i32, i32, i32, i32)> = INPUT
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();
            let (axs, ays) = a.split_once(',').unwrap();
            let (bxs, bys) = b.split_once(',').unwrap();
            let ax = axs.parse::<i32>().unwrap();
            let ay = ays.parse::<i32>().unwrap();
            let bx = bxs.parse::<i32>().unwrap();
            let by = bys.parse::<i32>().unwrap();
            (ax, ay, bx, by)
        })
        .collect();

    {
        let flat_clouds: Vec<(i32, i32, i32, i32)> = clouds
            .clone()
            .into_iter()
            .filter(|(ax, ay, bx, by)| ax == bx || ay == by)
            .collect();
        let map: HashMap<(i32, i32), i32> = flat_clouds
            .iter()
            .flat_map(|point| collect_points(*point))
            .fold(HashMap::new(), |mut map, point| {
                *map.entry(point).or_insert(0) += 1;
                map
            });
        let overlapping_points = map.iter().filter(|(_k, v)| **v > 1).count();
        println!("parta: {}", overlapping_points);
    }

    {
        let map: HashMap<(i32, i32), i32> = clouds
            .iter()
            .flat_map(|line| collect_points(*line))
            .fold(HashMap::new(), |mut map, point| {
                *map.entry(point).or_insert(0) += 1;
                map
            });
        let overlapping_points = map.iter().filter(|(_k, v)| **v > 1).count();
        // print_map(&map);
        println!("partb: {}", overlapping_points);
    }
}

fn collect_points(line: (i32, i32, i32, i32)) -> HashSet<(i32, i32)> {
    let (ax, ay, bx, by) = line;
    let width = bx - ax;
    let height = by - ay;
    let point_count = width.abs().max(height.abs());
    let dx = width.signum();
    let dy = height.signum();

    (0..=point_count)
        .map(|i| (i * dx + ax, i * dy + ay))
        .collect()
}

fn print_map(map: &HashMap<(i32, i32), i32>) {
    let min_x = map.iter().map(|((x, _), _)| x).min().unwrap();
    let max_x = map.iter().map(|((x, _), _)| x).max().unwrap();
    let min_y = map.iter().map(|((_, y), _)| y).min().unwrap();
    let max_y = map.iter().map(|((_, y), _)| y).max().unwrap();
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            let symbol = map
                .get(&(x, y))
                .map(|i| format!("{}", i))
                .unwrap_or(".".to_string());
            print!("{}", symbol);
        }
        println!("");
    }
}
