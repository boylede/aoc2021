mod input;

fn main() {
    println!("day 3");
    let input = input::day3::INPUT;
    //     let input = "00100
    // 11110
    // 10110
    // 10111
    // 10101
    // 01111
    // 00111
    // 11100
    // 10000
    // 11001
    // 00010
    // 01010";
    // 749376

    {
        let gamma = input
            .lines()
            .map(|s| -> Vec<(u32, u32)> {
                s.chars()
                    .rev()
                    .map(|c| if c == '1' { (1, 0) } else { (0, 1) })
                    .collect()
            })
            .reduce(|mut bits: Vec<(u32, u32)>, current: Vec<(u32, u32)>| {
                bits.iter_mut()
                    .zip(current.iter())
                    .for_each(|((t, f), (nt, nf))| {
                        *t += nt;
                        *f += nf;
                    });
                bits
            })
            .unwrap()
            .iter()
            .enumerate()
            .fold(0, |a, (position, (t, f))| {
                let set = if t > f { 1u32 } else { 0u32 };
                a | set << position
            });
        let bit_mask = (1 << 12) - 1;
        let epsilon = (!gamma) & bit_mask;
        let parta = gamma * epsilon;
        println!("parta: {}", parta);
        // assert!(749376 == parta);
    }

    {
        // let mut bit_accumulator: Vec<Vec<(u32, u32)>>  = input
        //     .lines()

        //     }).collect();
        let mut diagnostics: Vec<Vec<u32>> = input
            .lines()
            .map(|s| -> Vec<u32> {
                s.chars()
                    // .rev()
                    .map(|c| if c == '1' { 1 } else { 0 })
                    .collect()
            })
            .collect();
        for bit in 0..12 {
            println!("considering bit {}", bit);
            let mut remove_list = Vec::new();
            let bit_counts: (u32, u32) =
                diagnostics
                    .iter()
                    .filter_map(|v| v.get(bit))
                    .fold(
                        (0, 0),
                        |(t, f), i| {
                            if *i == 1 {
                                (t + 1, f)
                            } else {
                                (t, f + 1)
                            }
                        },
                    );
            println!("there are {} set and {} unset", bit_counts.0, bit_counts.1);
            let most_common_bit = if bit_counts.0 >= bit_counts.1 { 1 } else { 0 };
            println!("therefor the most common is {}", most_common_bit);
            for (i, diagnostic) in diagnostics.iter().enumerate() {
                if diagnostic[bit] != most_common_bit {
                    remove_list.push(i);
                    println!("will remove {}", i);
                }
            }

            diagnostics = diagnostics
                .into_iter()
                .enumerate()
                .filter_map(|(index, value)| {
                    if remove_list.contains(&index) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .collect();
            println!("kept {:?}", diagnostics);

            // while i < diagnostics.len() {
            //     if i in  {
            //         let val = vec.remove(i);
            //         // your code here
            //     } else {
            //         i += 1;
            //     }
        }
        // for i in remove_list.iter() {
        //     diagnostics.swap_remove(*i);
        // }
        println!("there are {} options left.", diagnostics.len());
        let oxygen = diagnostics
            .iter()
            .next()
            .unwrap()
            // .chars()
            .iter()
            .rev()
            // .rev()
            // .map(|c| if c == '1' { 1 } else { 0 })
            .enumerate()
            // .enumerate()
            .fold(0, |a, (position, set)| a | set << position);
        println!("oxygen: {}", oxygen);
        let mut diagnostics: Vec<Vec<u32>> = input
            .lines()
            .map(|s| -> Vec<u32> {
                s.chars()
                    // .rev()
                    .map(|c| if c == '1' { 1 } else { 0 })
                    .collect()
            })
            .collect();
        for bit in 0..12 {
            if diagnostics.len() == 1 {
                break;
            }
            println!("considering bit {}", bit);
            let mut remove_list = Vec::new();
            let bit_counts: (u32, u32) =
                diagnostics
                    .iter()
                    .filter_map(|v| v.get(bit))
                    .fold(
                        (0, 0),
                        |(t, f), i| {
                            if *i == 1 {
                                (t + 1, f)
                            } else {
                                (t, f + 1)
                            }
                        },
                    );
            println!("there are {} set and {} unset", bit_counts.0, bit_counts.1);
            let least_common_bit = if bit_counts.0 >= bit_counts.1 { 0 } else { 1 };
            println!("therefor the least common is {}", least_common_bit);
            for (i, diagnostic) in diagnostics.iter().enumerate() {
                if diagnostic[bit] != least_common_bit {
                    remove_list.push(i);
                    println!("will remove {}", i);
                }
            }

            diagnostics = diagnostics
                .into_iter()
                .enumerate()
                .filter_map(|(index, value)| {
                    if remove_list.contains(&index) {
                        None
                    } else {
                        Some(value)
                    }
                })
                .collect();
            println!("kept {:?}", diagnostics);

            // while i < diagnostics.len() {
            //     if i in  {
            //         let val = vec.remove(i);
            //         // your code here
            //     } else {
            //         i += 1;
            //     }
        }
        // for i in remove_list.iter() {
        //     diagnostics.swap_remove(*i);
        // }
        println!("there are {} options left.", diagnostics.len());
        let scrubber = diagnostics
            .iter()
            .next()
            .unwrap()
            // .chars()
            .iter()
            .rev()
            // .rev()
            // .map(|c| if c == '1' { 1 } else { 0 })
            .enumerate()
            // .enumerate()
            .fold(0, |a, (position, set)| a | set << position);

        println!("scrubber: {}", scrubber);
        let partb = oxygen * scrubber;
        println!("partb: {}", partb);
    }
}
