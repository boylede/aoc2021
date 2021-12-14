// day 3
// gamma: 3903
// epsilon: 192
// parta: 749376
// oxygen: 3871
// scrubber: 613
// partb: 2372923

const INPUT: &str = include_str!("../../input/day3.txt");

const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

fn main() {
    println!("day 3");
    let bit_width = INPUT.lines().next().unwrap().len();
    {
        let gamma = INPUT
            .lines()
            .map(|s| -> Vec<(u32, u32)> {
                s.chars()
                    .rev()
                    .map(|c| if c == '1' { (1, 0) } else { (0, 1) })
                    .collect()
            })
            .reduce(|mut totals: Vec<(u32, u32)>, next: Vec<(u32, u32)>| {
                totals.iter_mut().zip(next.iter()).for_each(
                    |((trues, falses), (next_true, next_false))| {
                        *trues += next_true;
                        *falses += next_false;
                    },
                );
                totals
            })
            .unwrap()
            .iter()
            .enumerate()
            .fold(0, |accumulator, (position, (trues, falses))| {
                let bit = if trues > falses { 1u32 } else { 0u32 };
                accumulator | bit << position
            });
        let bit_mask = (1 << bit_width) - 1;
        let epsilon = (!gamma) & bit_mask;
        let parta = gamma * epsilon;
        println!("parta: {}", parta);
    }

    {
        let diagnostics: Vec<Vec<u32>> = INPUT
            .lines()
            .map(|s| -> Vec<u32> { s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect() })
            .collect();
        let oxygen_diagnostics = (0..bit_width).fold(diagnostics.clone(), |diagnostics, bit| {
            let bit_counts: u32 = diagnostics.iter().map(|v| v.get(bit).unwrap()).sum();
            let most_common_bit = if bit_counts >= diagnostics.len() as u32 - bit_counts {
                1
            } else {
                0
            };
            diagnostics
                .into_iter()
                .filter(|p| *p.get(bit).unwrap() == most_common_bit)
                .collect()
        });
        let oxygen: u32 = oxygen_diagnostics
            .first()
            .unwrap()
            .iter()
            .rev()
            .enumerate()
            .map(|(position, set)| set << position)
            .sum();
        let scrubber_diagnostics = (0..bit_width).fold(diagnostics, |diagnostics, bit| {
            if diagnostics.len() == 1 {
                diagnostics
            } else {
                let popular_bit_count: u32 = diagnostics.iter().map(|v| v.get(bit).unwrap()).sum();
                let unpopular_bit_count = diagnostics.len() as u32 - popular_bit_count;
                let most_common_bit = if popular_bit_count >= unpopular_bit_count {
                    1
                } else {
                    0
                };
                diagnostics
                    .into_iter()
                    .filter(|p| *p.get(bit).unwrap() != most_common_bit)
                    .collect()
            }
        });
        let scrubber: u32 = scrubber_diagnostics
            .first()
            .unwrap()
            .iter()
            .rev()
            .enumerate()
            .map(|(position, set)| set << position)
            .sum();
        // .fold(0, |a, (position, set)| a | set << position);
        let partb = oxygen * scrubber;
        println!("partb: {}", partb);
    }
}
