use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day17.txt");
const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";
const TEST_INPUT_C: &str = "target area: x=119..176, y=-141..-84";
const TEST_INPUT_B: &str = "target area: x=819..820, y=-11..-5";
const INPUT_TRICKY: &str = "target area: x=352..377, y=-49..-30"; // correct answer is 66, not 1176

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

    let minimum_x_velocity = (0..=min_x)
        .filter(|x| (1..=(*x)).sum::<i64>() > min_x)
        .min()
        .unwrap();
    let max_y_velocity = min_y * -1;

    let solutions: Vec<_> = (minimum_x_velocity..=max_x)
        .cartesian_product(min_y..=max_y_velocity)
        .filter_map(|(vx, vy)| {
            let s = test(min_x, max_x, min_y, max_y, vx, vy);
            if let Some(y) = s {
                Some((y, vx, vy))
            } else {
                None
            }
        })
        .collect();

    {
        let (highest_y, _ivx, _ivy) = solutions.iter().max_by_key(|(y, _vx, _vy)| *y).unwrap();
        println!("parta: {}", highest_y);
    }
    {
        let count = solutions.len();
        println!("partb: {}", count);
    }
}

fn is_inside(x: i64, y: i64, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> bool {
    x >= min_x && x <= max_x && y >= min_y && y <= max_y
}

fn test(min_x: i64, max_x: i64, min_y: i64, max_y: i64, mut vx: i64, mut vy: i64) -> Option<i64> {
    let mut highest_y: i64 = 0;
    let mut x = 0;
    let mut y = 0;
    while !is_inside(x, y, min_x, max_x, min_y, max_y) {
        let (nx, ny, nvx, nvy) = step(x, y, vx, vy);
        x = nx;
        y = ny;
        vx = nvx;
        vy = nvy;
        if y > highest_y {
            highest_y = y;
        }
        if x > max_x || y < min_y {
            break;
        }
    }
    let hit = is_inside(x, y, min_x, max_x, min_y, max_y);
    if hit {
        Some(highest_y)
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
        -1 => vx += 1,
        _ => unreachable!(),
    }

    vy -= 1;
    (x, y, vx, vy)
}
