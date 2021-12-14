const INPUT: &str = include_str!("../../input/day10.txt");

const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

fn main() {
    println!("day 10");
    
    let tokens: Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    {
        let currupt_lines: u32 = tokens
            .iter()
            .map(|tokens| {
                tokens.iter().fold(
                    (Vec::new(), None),
                    |(mut a, e): (Vec<char>, Option<char>), n| {
                        if e.is_some() {
                            // short circuit the fold and stop as soon as we have an error
                            (a, e)
                        } else {
                            if let Some(last_open) = a.last() {
                                // we are in a nested chunk
                                if is_open(*n) {
                                    // starting a new chunk always succeeds
                                    a.push(*n);
                                    (a, None)
                                } else {
                                    // if we are closing a chunk, check that we are closing the right one
                                    let next_expected_close = matching_close(*last_open);
                                    if *n == next_expected_close {
                                        // bracket matches, remove the chunk from the ast
                                        a.pop();
                                        (a, None)
                                    } else {
                                        // bracket didn't match, return it
                                        (a, Some(*n))
                                    }
                                }
                            } else {
                                // we are starting a new chunk
                                if is_open(*n) {
                                    // starting a new chunk always succeeds
                                    a.push(*n);
                                    (a, None)
                                } else {
                                    // cant close a chunk we didn't open
                                    (a, Some(*n))
                                }
                            }
                        }
                    },
                )
            })
            .filter(|(_, e)| e.is_some())
            .map(|(_, e)| score_currupt(e.unwrap()))
            .sum();
        println!("parta: {}", currupt_lines);
    }
    {
        let mut incomplete_lines_scores: Vec<u64> = tokens
            .iter()
            .map(|tokens| {
                tokens.iter().fold(
                    (Vec::new(), None),
                    |(mut a, e): (Vec<char>, Option<char>), n| {
                        if e.is_some() {
                            (a, e)
                        } else {
                            if let Some(last_open) = a.last() {
                                // we are in a nested chunk

                                if is_open(*n) {
                                    a.push(*n);
                                    (a, None)
                                } else {
                                    let next_expected_close = matching_close(*last_open);
                                    if *n == next_expected_close {
                                        a.pop();
                                        (a, None)
                                    } else {
                                        (a, Some(*n))
                                    }
                                }
                            } else {
                                // we are starting a new chunk
                                if is_open(*n) {
                                    a.push(*n);
                                    (a, None)
                                } else {
                                    // cant close a chunk we didn't open
                                    (a, Some(*n))
                                }
                            }
                        }
                    },
                )
            })
            .filter(|(_, e)| e.is_none())
            .map(|(incomplete, _)| {
                incomplete
                    .iter()
                    .rev()
                    .map(|b| matching_close(*b))
                    .fold(0, |score, next| (score * 5) + score_incomplete(next))
            })
            .collect();
        incomplete_lines_scores.sort();
        let middle = incomplete_lines_scores.len() / 2;
        let middling_score = incomplete_lines_scores[middle];
        println!("partb: {}", middling_score);
    }
}

fn is_open(b: char) -> bool {
    match b {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

fn is_close(b: char) -> bool {
    match b {
        ')' => true,
        ']' => true,
        '}' => true,
        '>' => true,
        _ => false,
    }
}

fn matching_close(b: char) -> char {
    match b {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unexpected value in input"),
    }
}

fn score_currupt(b: char) -> u32 {
    match b {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected value in input: {}", b),
    }
}

fn score_incomplete(b: char) -> u64 {
    match b {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unexpected value in input: {}", b),
    }
}
