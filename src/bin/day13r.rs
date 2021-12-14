// generate inputs for day 13

use rand::{prelude::*, Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

const CODE: &str = "HELLO WORLD";
const FOLDS: usize = 12;
const ALPHABET: &str = include_str!("input/alphabet.txt");

const Y_AXIS: &str = "fold along y";
const X_AXIS: &str = "fold along x";

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(4); // chosen by fair dice roll
    let letters: HashMap<char, HashSet<(i32, i32)>> = ALPHABET
        .split("\r\n\r\n")
        .map(|lettermap| lettermap.split_once('\n').unwrap())
        .map(|(key, bitmap)| {
            if let Some(key) = key.chars().next() {
                let map = bitmap
                    .lines()
                    .enumerate()
                    .flat_map(|(y, line)| {
                        line.chars()
                            .enumerate()
                            .filter(|(_, c)| *c == '#')
                            .map(move |(x, _)| (x as i32, y as i32))
                    })
                    .collect();
                (key, map)
            } else {
                panic!("invalid input");
            }
        })
        .collect();

    let map: Vec<(i32, i32)> = CODE
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let x_offset = i * 5; //todo: chars not 4 wide
            letters
                .get(&c)
                .unwrap()
                .iter()
                .map(move |(x, y)| (x + x_offset as i32, *y))
        })
        .collect();

    let (map, folds): (Vec<(i32, i32)>, Vec<String>) =
        (0..FOLDS).fold((map, Vec::new()), |(map, mut folds), _fold| {
            let (fold, (dx, dy)) = if rng.gen_bool(0.5) {
                let dx = map.iter().map(|(x, _)| *x).max().unwrap() + 1;
                let fold = format!("\n{}={}", X_AXIS, dx);
                (fold, (Some(dx), None))
            } else {
                let dy = map.iter().map(|(_, y)| *y).max().unwrap() + 1;
                let fold = format!("\n{}={}", Y_AXIS, dy);
                (fold, (None, Some(dy)))
            };
            let map = map
                .into_iter()
                .flat_map(|(x, y)| {
                    let nx = if let Some(dx) = dx { dx + (dx - x) } else { x };
                    let ny = if let Some(dy) = dy { dy + (dy - y) } else { y };
                    if rng.gen_bool(0.5) {
                        vec![(x, y), (nx, ny)]
                    } else {
                        // move the value
                        if rng.gen_bool(0.5) {
                            // stay here
                            vec![(x, y)]
                        } else {
                            // move there
                            vec![(nx, ny)]
                        }
                    }
                })
                .collect();
            folds.push(fold);
            (map, folds)
        });
    let output: String = map
        .iter()
        .map(|(x, y)| format!("{},{}\n", x, y))
        .chain(folds.iter().rev().cloned())
        .collect();
    println!("{}", output);
}
