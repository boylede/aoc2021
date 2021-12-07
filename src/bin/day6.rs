mod input;

const TEST_INPUT: &str = "3,4,3,1,2";

fn main() {
    println!("day 6");
    let input = input::day6::INPUT;

    let starter_fish: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    {
        let count = std::iter::repeat(())
            .scan(starter_fish.clone(), |state, _| {
                let mut born_count = 0;
                state.iter_mut().for_each(|i| {
                    if *i == 0 {
                        *i = 6;
                        born_count += 1;
                    } else {
                        *i -= 1
                    }
                });
                for _ in 0..born_count {
                    state.push(8);
                }
                Some(state.len())
            })
            .skip(79)
            .next()
            .unwrap();

        println!("parta: {}", count);
    }

    {
        let mut buckets = (0u64,0,0,0,0,0,0,0,0);
        for fish in starter_fish.iter() {
            match fish {
                0 => buckets.0 += 1,
                1 => buckets.1 += 1,
                2 => buckets.2 += 1,
                3 => buckets.3 += 1,
                4 => buckets.4 += 1,
                5 => buckets.5 += 1,
                6 => buckets.6 += 1,
                7 => buckets.7 += 1,
                8 => buckets.8 += 1,
                _ => panic!("unexpected input"),
            }
        }
        (0..256).for_each(|_|{
            let zeros = buckets.0;
            buckets.0 = buckets.1;
            buckets.1 = buckets.2;
            buckets.2 = buckets.3;
            buckets.3 = buckets.4;
            buckets.4 = buckets.5;
            buckets.5 = buckets.6;
            buckets.6 = buckets.7;
            buckets.7 = buckets.8;
            buckets.8 = zeros;
            buckets.6 += zeros;
        });
        let sum = buckets.0 + buckets.1 + buckets.2 +buckets.3 + buckets.4 + buckets.5 + buckets.6 + buckets.7 + buckets.8;
        println!("partb: {}", sum);
    }
}
