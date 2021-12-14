use std::collections::HashMap;
const INPUT: &str = include_str!("../../input/day7.txt");

const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

fn main() {
    println!("day 7");

    let crabs: Vec<i32> = INPUT
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    {
        let min = *crabs.iter().min().unwrap();
        let max = *crabs.iter().max().unwrap();
        let fuel: i32 = (min..=max)
            .map(|candidate| crabs.iter().map(|crab| (candidate - crab).abs()).sum())
            .min()
            .unwrap();

        println!("parta: {}", fuel);
    }
    {
        let min = *crabs.iter().min().unwrap();
        let max = *crabs.iter().max().unwrap();

        let fuel: i32 = (min..=max)
            .map(|candidate| {
                crabs
                    .iter()
                    .map(|crab| (0..=(candidate - crab).abs()).sum::<i32>())
                    .sum()
            })
            .min()
            .unwrap();
        println!("partb: {}", fuel);
    }
}
