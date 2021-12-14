use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input/day12.txt");

const START: &str = "start";
const END: &str = "end";

const TEST_INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

const TEST_INPUTB: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

const TEST_INPUTC: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

fn main() {
    println!("day 12");
    
    let edges: Vec<(&str, &str)> = INPUT
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    {
        let bi_edges: Vec<(&&str, &&str)> =
            edges.iter().flat_map(|(a, b)| [(a, b), (b, a)]).collect();
        let mut complete_paths: Vec<String> = Vec::new();
        let mut paths_to_explore: Vec<String> = Vec::new();
        let first_path = START.to_string();
        paths_to_explore.push(first_path);

        while paths_to_explore.len() > 0 {
            let this_path = paths_to_explore.pop().unwrap();
            let last_node = this_path.split(',').last().unwrap();
            if last_node != "end" {
                for next_node in bi_edges
                    .iter()
                    .filter(|(a, _)| **a == last_node)
                    .map(|(_, b)| b)
                {
                    if next_node.chars().all(|c: char| c.is_ascii_uppercase())
                        || !this_path.contains(*next_node)
                    {
                        let mut next_path = this_path.clone();
                        next_path.push_str(",");
                        next_path.push_str(next_node);
                        paths_to_explore.push(next_path);
                    }
                }
            } else {
                complete_paths.push(this_path);
            }
        }

        println!("parta: {}", complete_paths.len());
    }
    {
        let bi_edges: Vec<(&&str, &&str)> =
            edges.iter().flat_map(|(a, b)| [(a, b), (b, a)]).collect();
        let mut complete_paths: Vec<String> = Vec::new();
        let mut paths_to_explore: Vec<String> = Vec::new();
        let first_path = START.to_string();
        paths_to_explore.push(first_path);

        while paths_to_explore.len() > 0 {
            let this_path = paths_to_explore.pop().unwrap();
            let nodes = this_path.split(',');
            let last_node = nodes.clone().last().unwrap();

            let small_cave_visited_twice = {
                nodes
                    .filter(|node| node.chars().all(|c: char| c.is_ascii_lowercase()))
                    .fold(HashMap::new(), |mut a, n| {
                        *a.entry(n).or_insert(0) += 1;
                        a
                    })
                    .iter()
                    .filter(|(_, c)| **c > 1)
                    .map(|(c, _)| c)
                    .next()
                    .map(|c| c.to_string())
            };

            if last_node != END {
                for next_node in bi_edges
                    .iter()
                    .filter(|(a, _)| **a == last_node)
                    .map(|(_, b)| b)
                {
                    if next_node.chars().all(|c: char| c.is_ascii_uppercase())
                        || !this_path.contains(*next_node)
                        || (small_cave_visited_twice.is_none() && **next_node != START)
                    {
                        let mut next_path = this_path.clone();
                        next_path.push_str(",");
                        next_path.push_str(next_node);
                        paths_to_explore.push(next_path);
                    }
                }
            } else {
                complete_paths.push(this_path);
            }
        }
        println!("partb: {}", complete_paths.len());
    }
}
