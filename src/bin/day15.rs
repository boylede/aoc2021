use std::{collections::HashMap};

const INPUT: &str = include_str!("../../input/day15.txt");
const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

// benchmark times: debug build 
// day 15
// parta: 423
// 1,654 ms
// partb: 2778
// 120,708 ms
// release build: day 15
// parta: 423
// 83 ms
// partb: 2778
// 6,956 ms



fn main() {
    println!("day 15");
    let a_time = time::precise_time_ns();
    // let part1 = (self.part1)(&lines);
    // let b_time = time::precise_time_ns();
    // // let part2 = (self.part2)(&lines);
    // let c_time = time::precise_time_ns();
    // let p1_time = b_time - a_time;
    let ns_in_ms = 1_000_000;

    let cave: HashMap<(i32, i32), i32> = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, risk)| ((x as i32, y as i32), risk.to_digit(10).unwrap() as i32))
        })
        .collect();
    let start = (0, 0);
    let width = cave.keys().map(|(x, _)| *x as i32).max().unwrap();
    let height = cave.keys().map(|(_, y)| *y as i32).max().unwrap();
    let end = (width, height);
    // println!("found {:?} end point", end);
    {
        let path_score = wikipedia_a_star(start, end, cave.clone()).unwrap();
        println!("parta: {}", path_score);
    }
    let b_time = time::precise_time_ns();
    let p1_time = b_time - a_time;
    if p1_time > ns_in_ms {
        println!("{} ms", p1_time / ns_in_ms);
    } else {
        println!("{} ns", p1_time);
    }
    {
        
        let caves = {
            let mut caves = HashMap::new();//cave.clone();
            let width = width + 1;
            let height = height + 1;
            for row in 0..5 {
                for col in 0..5 {
                    // if row == 0 && col == 0 {
                    //     break;
                    // }
                    for ((x,y), weight) in cave.iter() {
                        
                        let new_position = (x + width * col, y + height * row);
                        let weight = weight + row + col;
                        let weight = if weight > 9 {
                            weight - 9
                        } else {
                            weight
                        };
                        // if *x == 0 && *y == 0 {
                        //     println!("{},{} => {}", col,row,weight);
                        // }
                        caves.insert(new_position, weight);
                    }
                }
            }
            caves
        };
        
        let width = caves.keys().map(|(x, _)| *x as i32).max().unwrap();
        let height = caves.keys().map(|(_, y)| *y as i32).max().unwrap();
        let end = (width, height);

        // println!("starting weights:");
        // for y in 0..=height {
        //     for x in 0..=width {
        //         if caves.contains_key(&(x,y)) {
        //             let weight = caves.get(&(x,y)).unwrap();
        //             print!("{}", weight);
        //         } else {
        //             panic!("coord out of map");
        //         }
        //     }
        //     println!("");
        // }
        // println!("--------------------------------");
        // panic!("stop");

        let path_score = wikipedia_a_star(start, end, caves.clone()).unwrap();
        
        println!("partb: {}", path_score);
    }
    let c_time = time::precise_time_ns();
    
    let p2_time = c_time - b_time;
    if p2_time > ns_in_ms {
        println!("{} ms", p2_time / ns_in_ms);
    } else {
        println!("{} ns", p2_time);
    }
}

/// searches for a path from start to end, returning the weight of the path if one is found
fn wikipedia_a_star(
    start: (i32, i32),
    end: (i32, i32),
    weights: HashMap<(i32, i32), i32>,
) -> Option<i32> {
    fn h(point: (i32, i32), end: (i32, i32)) -> i32 {
        let dx = (end.0 - point.0).abs();
        let dy = (end.1 - point.1).abs();
        dx + dy
    }
    let (max_x, max_y) = end;
    // println!("finding path from {:?} to {:?}", start, end);
    let mut open_set = vec![start];
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> =
        weights.keys().map(|(x, y)| ((*x, *y), i32::MAX)).collect();
    g_score.insert(start, 0);

    let mut f_score = g_score.clone();
    f_score.insert(start, h(start, end));
    // f_score[&start] = h(start, end);

    while open_set.len() > 0 {
        
        let (current_index, current) = open_set
            .iter()
            .cloned()
            .enumerate()
            .min_by_key(|(_i, point)| {
                f_score.get(point).unwrap()
            })
            .unwrap();
        
            

        if current == end {
            let mut score = 0;
            let mut current = current;
            while came_from[&current] != start {
                // println!("{:?} => {}", current, weights[&current]);
                score += weights[&current];
                current = came_from[&current];
            }
            // println!("start: {:?} => {}", current, weights[&current]);
            score += weights[&current];
            return Some(score);
        }
        open_set.swap_remove(current_index);

        // {
        //     // let max_x = 10;
        //     // let max_y = 10;
        //     for y in 0..max_y {
        //         for x in 0..max_x {
        //             if open_set.contains(&(x,y)) {
        //                 print!("{} ", weights.get(&(x,y)).unwrap() )
        //             } else if current == (x,y) {
        //                 print!("# ");
        //             } else if came_from.contains_key(&(x,y)) {
        //                 print!("~ ");
        //             } else if !weights.contains_key(&(x,y)) {
        //                 print!("!!");
        //             } else {
        //                 print!(". ");
        //             }
        //         }
        //         println!("");
        //     }
        // }
        // println!("-----------");

        let (cx, cy) = current;
        let neighbors = [(cx - 1, cy), (cx, cy - 1), (cx + 1, cy), (cx, cy + 1)];
        for neighbor in neighbors {
            if let Some(neighbor_weight) = weights.get(&neighbor) {
                let tentative_g_score = g_score[&current] + neighbor_weight; // i think this is correct
                // println!("will cost {} to move to {:?}", tentative_g_score, neighbor);
                if let Some(gs) = g_score.get(&neighbor) {
                    if tentative_g_score < *gs {
                        // println!("{} < {}, going with {:?}", tentative_g_score, gs, neighbor);
                        came_from.insert(neighbor, current);
                        // came_from[&neighbor] = current;
                        g_score.insert(neighbor, tentative_g_score);
                        // g_score[&neighbor] = tentative_g_score;
                        f_score.insert(neighbor, tentative_g_score + h(neighbor, end));
                        // f_score[&neighbor] = tentative_g_score + h(neighbor, end);
                        if !open_set.contains(&neighbor) {
                            open_set.push(neighbor);
                        }
                    }
                } else {
                    panic!("neighbor not in g_score {:?}", neighbor);
                }
            } else {
                let (nx, ny) = neighbor;
                if nx > 0 && nx < max_x && ny > 0 && ny < max_y {
                    // println!("neighbor {:?} not in map", neighbor);
                }
                
            }
            
            
        }
    }
    None
}
