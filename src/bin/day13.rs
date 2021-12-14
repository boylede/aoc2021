use std::collections::{HashMap, HashSet};

mod input;

const ALPHABET: &str = include_str!("input/alphabet.txt");

const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

const Y_AXIS: &str = "fold along y";
const X_AXIS: &str = "fold along x";

fn main() {
    println!("day 13");
    let input = input::day13::INPUT;
    let (dots, instructions) = input.split_once("\n\n").unwrap();
    let dots: Vec<(i32, i32)> = dots
        .lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .collect();
    let instructions: Vec<(&str, i32)> = instructions
        .lines()
        .map(|s| s.split_once('=').unwrap())
        .map(|(a, v)| (a, v.parse::<i32>().unwrap()))
        .collect();
    println!(
        "found {} dots and {} instructions",
        dots.len(),
        instructions.len()
    );

    {
        let dot_set: HashSet<(i32, i32)> = dots.iter().copied().collect();
        let first_fold = instructions
            .iter()
            .scan(
                dot_set,
                |dots, (axis, value): &(&str, i32)| -> Option<HashSet<(i32, i32)>> {
                    let new_dots: HashSet<_> = dots
                        .iter()
                        .map(|(x, y)| match axis {
                            &X_AXIS => {
                                if x > value {
                                    (value - (x - value), *y)
                                } else {
                                    (*x, *y)
                                }
                            }
                            &Y_AXIS => {
                                if y > value {
                                    (*x, value - (y - value))
                                } else {
                                    (*x, *y)
                                }
                            }
                            _ => panic!("invalid input"),
                        })
                        .collect();
                    *dots = new_dots;

                    Some(dots.clone())
                },
            )
            .next()
            .unwrap();
        let dot_count = first_fold.iter().count();
        println!("parta: {}", dot_count);
    }
    {
        let dot_set: HashSet<(i32, i32)> = dots.iter().copied().collect();
        let code_image = instructions.iter().fold(
            dot_set,
            |dots: HashSet<(i32, i32)>, (axis, value): &(&str, i32)| -> HashSet<(i32, i32)> {
                let new_dots: HashSet<(i32, i32)> = dots
                    .into_iter()
                    .map(|(x, y)| {
                        let value = *value;
                        match axis {
                            &X_AXIS => {
                                if x > value {
                                    (value - (x - value), y)
                                } else {
                                    (x, y)
                                }
                            }
                            &Y_AXIS => {
                                if y > value {
                                    (x, value - (y - value))
                                } else {
                                    (x, y)
                                }
                            }
                            _ => panic!("invalid input"),
                        }
                    })
                    .collect();
                new_dots
            },
        );

        print_image(&code_image);
        let output = faux_ocr(&code_image);
        println!("partb: {}", output);
    }
}

fn print_image(dots: &HashSet<(i32, i32)>) {
    let min_x = dots.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = dots.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = dots.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = dots.iter().map(|(_, y)| *y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn faux_ocr(dots: &HashSet<(i32, i32)>) -> String {
    let digits: Vec<Vec<(i32, i32)>> = {
        // let mut buffer = [[[false; 4]; 6]; 8];
        let mut digits = HashMap::new();
        for (x, y) in dots.iter() {
            let digit = x / 5; //todo: digits not 4 wide will cause issues
            let row = *y;
            let col = x % 5;
            digits.entry(digit).or_insert(Vec::new()).push((col, row));
            // buffer[digit][col][row] = true;
        }
        let places = digits.keys().max().cloned().unwrap();
        let mut buffer = Vec::new();
        for place in 0..=places {
            let mut digit = if let Some(digit) = digits.get(&place) {
                digit.clone()
            } else {
                Vec::new()
            };
            digit.sort();
            buffer.push(digit);
        }
        buffer
    };
    let alphabet: HashMap<Vec<(i32, i32)>, char> = ALPHABET
        .split("\r\n\r\n")
        .map(|lettermap| lettermap.split_once('\n').unwrap())
        .map(|(key, bitmap)| {
            if let Some(key) = key.chars().next() {
                let mut map: Vec<(i32, i32)> = bitmap
                    .lines()
                    .enumerate()
                    .flat_map(|(y, line)| {
                        line.chars()
                            .enumerate()
                            .filter(|(_, c)| *c == '#')
                            .map(move |(x, _)| (x as i32, y as i32))
                    })
                    .collect();
                map.sort();
                (map, key)
            } else {
                panic!("invalid input");
            }
        })
        .collect();

    // let mut s = String::with_capacity(digits.len());
    let s: String = digits.iter().map(|d| alphabet.get(d).unwrap()).collect();
    // for letter in digits.iter() {
    //     unimplemented!()
    // }
    // s
    s
}
