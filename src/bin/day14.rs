use std::collections::HashMap;
const INPUT: &str = include_str!("../../input/day14.txt");
const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

const FIVE_MEGABYTES: usize = 5 * 1024 * 1024;
const FOUR_MEGABYTES: usize = 4 * 1024 * 1024;

fn main() {
    println!("day 14");
    let (template, rules) = INPUT.split_once("\n\n").unwrap();
    let template: Vec<char> = template.chars().collect();
    let rules: HashMap<(char, char), char> = rules
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(pair, inserts)| {
            (
                (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap()),
                inserts.chars().nth(0).unwrap(),
            )
        })
        .collect();

    {
        let mut expanded = template.clone();

        // println!("{:?}", expanded);
        for _ in 0..10 {
            expanded.push(' '); // add dummy value to end
            expanded = expanded
                .windows(2)
                .map(|a| (a[0], a[1]))
                .flat_map(|(a, b)| {
                    if let Some(insert) = rules.get(&(a, b)) {
                        vec![a, *insert]
                    } else {
                        vec![a]
                    }
                })
                .collect();
            // println!("{:?}", expanded);
        }
        // expanded.pop(); // remove the dummy value
        let counts = expanded.iter().fold(HashMap::new(), |mut a, n| {
            *a.entry(n).or_insert(0) += 1;
            a
        });
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();

        println!("parta: {}", max - min);
    }
    {
        // let template: Vec<u8> = template.iter().map(|c| *c as u8).collect();*c as u8
        // let rules: HashMap<(u8, u8), u8> = rules
        //     .iter()
        //     .map(|((a, b), c)| ((*a as u8, *b as u8), *c as u8))
        //     .collect();
        let mut expanded = template.clone();
        expanded.push(' '); // pad with an extra char to preserve last value
        let pair_count: HashMap<(char, char), u64> = expanded
            .windows(2)
            .map(|slice| ((slice[0], slice[1]), 1))
            .collect();
        // println!("template is {:?}", pair_count);
        let pair_count = (0..40).fold(pair_count, |pair_count, step| {
            let new_pairs: HashMap<(char, char), u64> = pair_count
                .into_iter()
                .flat_map(|(pair, count)| {
                    if let Some(&c) = rules.get(&pair) {
                        let (a, b) = pair;
                        vec![((a, c), count), ((c, b), count)]
                    } else {
                        vec![(pair, count)]
                    }
                })
                // .flat_map(|((a, b), (c, d), count)| [((a, b), count), ((c, d), count)])
                .fold(HashMap::new(), |mut new_pairs, (pair, count)|{
                    *new_pairs.entry(pair).or_insert(0) += count;
                    new_pairs
                });
            // for (pair, count) in new_pairs.into_iter() {
            //     *pair_count.entry(pair).or_insert(0) += count;
            // }
            // println!("step {} is {:?}",step,  new_pairs);
            // pair_count
            new_pairs
        });

        let letter_counts: HashMap<char, u64> = pair_count
            .into_iter()
            // .flat_map(|((a, b), count)| vec![(a, count), (b, count)])
            .fold(HashMap::new(), |mut counts, ((letter, _), count)| {
                // we ignore the second character in the pair
                // thats why we had to insert the dummy value above
                *counts.entry(letter).or_insert(0) += count;
                counts
            });

        // let mut state: ((u8, u8), Vec<u8>) = ((a, b), expanded);
        // let mut expansion_queue: Vec<(usize, Vec<u8>, usize)> = vec![(0, expanded, 1)];
        // let mut letter_counts: HashMap<u8, i32> = HashMap::new();

        // while expansion_queue.len() > 0 {
        //     let (start, mut expansion_set, split_generation) = expansion_queue.pop().unwrap();

        //     for i in start..40 {
        //         let generations: Vec<usize> = expansion_queue.iter().map(|(i, _, _)| *i).collect();
        //         println!(
        //             "generation {}, step: {} = {}, with queue: {:?}",
        //             split_generation,
        //             i,
        //             expansion_set.len(),
        //             generations
        //         );
        //         expansion_set.push(b' '); // add dummy value to end
        //         expansion_set = expansion_set
        //             .windows(2)
        //             .map(|a| (a[0], a[1]))
        //             .flat_map(|(a, b)| {
        //                 if let Some(insert) = rules.get(&(a, b)) {
        //                     vec![a, *insert]
        //                 } else {
        //                     vec![a]
        //                 }
        //             })
        //             .collect();
        //         if expansion_set.len() > FIVE_MEGABYTES && i != 39 {
        //             println!("splitting off at step {}", i);
        //             let mid = expansion_set[FOUR_MEGABYTES];
        //             let (a, b) = expansion_set.split_at(FOUR_MEGABYTES);
        //             expansion_queue.push((i, b.to_vec(), split_generation + 1));
        //             expansion_set = a.to_vec();
        //             expansion_set.push(mid);
        //         }
        //     }
        //     println!("coalescing {} split back into count", split_generation);
        //     for letter in expansion_set.iter() {
        //         *letter_counts.entry(*letter).or_insert(0) += 1;
        //     }
        // }

        // // println!("{:?}", expanded);
        // for i in 0..40 {
        //     println!("step: {} = {}", i, expanded.len());
        //     expanded.push(b' '); // add dummy value to end
        //     expanded = expanded
        //         .windows(2)
        //         .map(|a| (a[0], a[1]))
        //         .flat_map(|(a, b)| {
        //             if let Some(insert) = rules.get(&(a, b)) {
        //                 vec![a, *insert]
        //             } else {
        //                 vec![a]
        //             }
        //         })
        //         .collect();
        //     // println!("{:?}", expanded);
        // }
        // // expanded.pop(); // remove the dummy value
        // let counts = expanded.iter().fold(HashMap::new(), |mut a, n| {
        //     *a.entry(n).or_insert(0) += 1;
        //     a
        // });
        let max = letter_counts.values().max().unwrap();
        let min = letter_counts.values().min().unwrap();
        println!("max {} vs min {}", max, min);
        println!("partb: {}", max - min);
    }
}
