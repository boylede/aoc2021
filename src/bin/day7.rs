use std::collections::HashMap;
mod input;

const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

fn main() {
    println!("day 7");
    let input = input::day7::INPUT;
    let crabs: Vec<i32> = input
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
