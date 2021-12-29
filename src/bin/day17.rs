use std::{collections::HashMap, ops::Neg};

const INPUT: &str = include_str!("../../input/day17.txt");
const INPUT_TRICKY: &str = "target area: x=352..377, y=-49..-30"; // correct answer is 66, not 1176
const TEST_INPUT_B: &str = "target area: x=819..820, y=-11..-5";
const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

const TEST_INPUT_C: &str = "target area: x=119..176, y=-141..-84";
fn main() {
    println!("day 17");
    let (a, b) = INPUT.split_once(", ").unwrap();
    let (a, max_x) = a.split_once("..").unwrap();
    let max_x = max_x.parse::<i64>().unwrap();
    let (_, min_x) = a.split_once("x=").unwrap();
    let min_x = min_x.parse::<i64>().unwrap();
    let (b, max_y) = b.split_once("..").unwrap();
    let max_y = max_y.parse::<i64>().unwrap();
    let (_, min_y) = b.split_once("y=").unwrap();
    let min_y = min_y.parse::<i64>().unwrap();

    // println!("target min x: {}", min_x);
    // println!("target max x: {}", max_x);
    // println!("target min y: {}", min_y);
    // println!("target max y: {}", max_y);

    // test(min_x, max_x, min_y, max_y, 7, 2);
    // test(min_x, max_x, min_y, max_y, 6, 3);
    // test(min_x, max_x, min_y, max_y, 9, 0);
    // test(min_x, max_x, min_y, max_y, 17, -4);
    // test(min_x, max_x, min_y, max_y, 6, 9);
    let minimum_x_velocity = (0..=min_x)
            .filter(|x| (1..=(*x)).sum::<i64>() > min_x)
            .min()
            .unwrap();
    let max_y_velocity = min_y * -1;
    // println!("found mimimum x velocity of {}", minimum_x_velocity);
    // println!("using max x velocity of {}", max_x);
    // println!("using min y velocity {}", min_y);
    // println!("using max y velocity {}", max_y_velocity);
    let mut tests = Vec::new();
    for vx in minimum_x_velocity..=max_x {
        for vy in min_y..=max_y_velocity {
            tests.push((vx, vy));
        }
    }
    // println!("testing: {:?}", tests);
    let solutions: Vec<_> = tests
        .iter()
        .rev()
        .filter_map(|(vx, vy)| {
            let s = test(min_x, max_x, min_y, max_y, *vx, *vy);
            if let Some((y, map)) = s {
                // println!("({},{}) yields {}", vx, vy, y);
                Some((y, vx, vy, map))
            } else {
                None
            }
        })
        .collect();

        
        
    // println!("reached {}, with ({},{})", highest_y, ivx, ivy);
    // print_map(&map);

    // let n = (min_y * -1) -1;
    // let _best_height_hack = n*(n + 1)/2;
    // println!("shortcut answer was {}", best_height_hack);

    {
        let (highest_y, _ivx,_ivy,_map) = solutions.iter().max_by_key(|(y, _vx, _vy, _map)| *y)
        .unwrap();

        println!("parta: {}", highest_y);
    }
    {
        let count = solutions.len();
        println!("partb: {}", count);
    }
}

fn test(min_x: i64, max_x: i64, min_y: i64, max_y: i64, mut vx: i64, mut vy: i64) -> Option<(i64,HashMap<(i64,i64),char>)> {
    let ovx = vx;
    let ovy = vy;
    // println!("testing ({},{})", vx, vy);
    let mut map: HashMap<(i64, i64), char> = HashMap::new();
    map.insert((0, 0), 'S');

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            map.insert((x, y), 'T');
        }
    }
    let mut x = 0;
    let mut y = 0;
    // let mut vx = 17;
    // let mut vy = -4;
    // let mut hit = false;
    while map.get(&(x, y)) != Some(&'T') {
        // println!("marking {},{}",x,y);
        map.entry((x, y)).or_insert('#');
        let (nx, ny, nvx, nvy) = step(x, y, vx, vy);
        x = nx;
        y = ny;
        vx = nvx;
        vy = nvy;
        if x > max_x || y < min_y {
            break;
        }
        // if y > min_y && vy
        // else if *hit_char == 'T' {
        //     hit = true;
        //     break;
        // }
    }
    let hit = map.get(&(x, y)) == Some(&'T');
    map.insert((x, y), '#');
    // print_map(&map);
    let highest_y = map.keys().map(|(_, y)| *y).max().unwrap();
    // println!("hit: {}, with highest y: {}", hit, highest_y);
    if hit {
        // println!("test ({},{}) yielded {}", ovx, ovy, highest_y);
        Some((highest_y, map))
    } else {
        None
    }
}

fn step(mut x: i64, mut y: i64, mut vx: i64, mut vy: i64) -> (i64, i64, i64, i64) {
    x += vx;
    y += vy;
    match vx.signum() {
        0 => (),
        1 => vx -= 1,
        -1 =>vx += 1,
        _ => unreachable!()//unsafe { std::hint::unreachable_unchecked()}
    }
    // if vx.is_positive() {
    //     vx -= 1;
    // } else {
    //     if vx != 0 {
    //         vx += 1;
    //     }
        
    // }

    vy -= 1;
    (x, y, vx, vy)
}

fn print_map(map: &HashMap<(i64, i64), char>) {
    let min_x = map.keys().map(|(x, _)| *x).min().unwrap();
    let max_x = map.keys().map(|(x, _)| *x).max().unwrap();
    let min_y = map.keys().map(|(_, y)| *y).min().unwrap();
    let max_y = map.keys().map(|(_, y)| *y).max().unwrap();
    for y in (min_y..=max_y).rev() {
        print!("{:04}: ",y);
        for x in min_x..=max_x {
            if let Some(c) = map.get(&(x, y)) {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
