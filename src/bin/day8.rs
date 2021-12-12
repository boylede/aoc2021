use std::collections::{HashMap, HashSet};
mod input;

const TEST_INPUT: &str =
    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
const TEST_INPUTB: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
fn main() {
    println!("day 8");
    let input = input::day8::INPUT;
    let signals = input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        // .filter_map(|o|o)
        .map(|(a, b)| {
            let rules: Vec<HashSet<char>> = a
                .split_whitespace()
                .map(|r| r.chars().collect::<HashSet<char>>())
                .collect();
            let output: Vec<HashSet<char>> = b
                .split_whitespace()
                .map(|r| r.chars().collect::<HashSet<char>>())
                .collect();
            (rules, output)
        })
        .collect::<Vec<_>>();

    println!("found {} lines", signals.len());
    println!(
        "line 0 had {} rules, output was {} long",
        signals[0].0.len(),
        signals[0].1.len()
    );
    {
        let uniques = signals
            .iter()
            .flat_map(|(rules, output)| output)
            .filter(|n| match n.len() {
                2 => true,
                3 => true,
                4 => true,
                7 => true,
                _ => false,
            })
            .count();
        println!("parta: {}", uniques);
    }
    {
        let values: i32 = signals
            .iter()
            .map(|(rules, output)| -> i32 {
                // println!("output count: {}", output.len());

                let knowns = rules
                    .iter()
                    .filter_map(|digit| {
                        match digit.len() {
                            2 => {
                                // println!("{:?} = 1", digit);
                                Some((digit, 1))
                            }
                            3 => {
                                // println!("{:?} = 7", digit);
                                Some((digit, 7))
                            }
                            4 => {
                                // println!("{:?} = 4", digit);
                                Some((digit, 4))
                            }
                            5 => {
                                // println!("5 = 2, 3, or 5: {:?}", digit);
                                None
                            }
                            6 => {
                                // println!("6 = 0, 6, or 9: {:?}", digit);
                                None
                            }
                            7 => {
                                // println!("{:?} = 8", digit);
                                Some((digit, 8))
                            }
                            _ => {
                                println!("unexpected {}", digit.len());
                                panic!()
                            }
                        }
                    })
                    .map(|(k, v)| {
                        let mut key = k.iter().cloned().collect::<Vec<_>>();
                        key.sort();
                        let key: String = key.iter().collect();
                        (key, v)
                    })
                    .collect::<HashMap<_, _>>();
                // println!("figured out {} known digits", knowns.len());
                if let Some((ones, _)) = knowns.iter().filter(|(k, v)| **v == 1).next() {
                    // lenth 5 which have both of one's chars are 3
                    // length 6 which do are not 6, but 0 or 9
                    if let Some((all_fours, _)) = knowns.iter().filter(|(k, v)| **v == 4).next() {
                        let fours: String =
                            all_fours.chars().filter(|c| !ones.contains(*c)).collect();
                        // fours contains two signals that are in 5 and 9
                        output
                            .iter()
                            .map(|digit| {
                                // println!("wanting to find out what {:?} is", digit);
                                match digit.len() {
                                    2 => {
                                        // println!("{:?} = 1", digit);
                                        1
                                    }
                                    3 => {
                                        // println!("{:?} = 7", digit);
                                        7
                                    }
                                    4 => {
                                        // println!("{:?} = 4", digit);
                                        4
                                    }
                                    5 => {
                                        if ones.chars().all(|signal| digit.contains(&signal)) {
                                            // println!("{:?} = 3", digit);
                                            3
                                        } else {
                                            // println!("5 = 2, 3, or 5: {:?}", digit);
                                            if fours.chars().all(|signal| digit.contains(&signal)) {
                                                // println!("{:?} = 5", digit);
                                                5
                                            } else {
                                                // println!("{:?} = 2", digit);
                                                2
                                            }
                                        }
                                    }
                                    6 => {
                                        if ones.chars().all(|signal| digit.contains(&signal)) {
                                            // println!("6 = 0, 6, or 9: {:?}", digit);
                                            if fours.chars().all(|signal| digit.contains(&signal)) {
                                                // println!("{:?} = 9", digit);
                                                9
                                            } else {
                                                // println!("{:?} = 0", digit);
                                                0
                                            }
                                        } else {
                                            // println!("{:?} = 6", digit);
                                            6
                                        }
                                    }
                                    7 => {
                                        // println!("{:?} = 8", digit);
                                        8
                                    }
                                    _ => {
                                        println!("unexpected {}", digit.len());
                                        panic!()
                                    }
                                }
                            })
                            .rev()
                            .enumerate()
                            .fold(0, |a, (i, d)| {
                                let place = 10i32.pow(i as u32);
                                // println!("adding {} to the mix", d*place);
                                a + d * place
                                // unimplemented!()
                            })
                    } else {
                        panic!("didn't have enough info A");
                    }
                } else {
                    panic!("didn't have enough info B");
                }
            })
            // .inspect(|v|println!("summing: {}", v))
            .sum::<i32>();
        // println!("idk {}", values);
        println!("partb: {}", values);
    }
}
