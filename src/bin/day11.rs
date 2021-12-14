const INPUT: &str = include_str!("../../input/day11.txt");

const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

const SMOL_TEST: &str = "11111
19991
19191
19991
11111";

fn main() {
    println!("day 11");
    
    let octopi: Vec<Vec<_>> = INPUT
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, energy)| (col, row, energy.to_digit(10).unwrap()))
                // .map(|d| (d, false, false))
                .collect::<Vec<_>>()
        })
        .collect();
    let width = octopi[0].len() as isize;
    let height = octopi.len() as isize;

    {
        // print_octopi(&octopi);
        let mut octopi = octopi.clone();
        let mut flashes = 0;

        for step in 1..=100 {
            // increment the energy of all octopi
            octopi
                .iter_mut()
                .for_each(|row| row.iter_mut().for_each(|(_, _, o)| *o += 1));
            // let o = octopi.iter_mut().cycle();

            // check which ones are flashing to prime the pump
            let mut is_flashing: Vec<Vec<(usize, usize, bool)>> = octopi
                .iter()
                .map(|row| row.iter().map(|(x, y, o)| (*x, *y, *o > 9)).collect())
                .collect();

            // keep track of which ones we need to propogate the energy flashes
            let mut just_flashed: Vec<(usize, usize)> = is_flashing
                .iter()
                .flat_map(|row| {
                    row.iter()
                        .filter(|(_x, _y, f)| *f)
                        .map(|(x, y, _)| (*x, *y))
                })
                .collect();
            // println!("at the beginning of step {}, there were {} natural flashes", step, just_flashed.len());
            let mut recent_flashes = just_flashed.len();
            while just_flashed.len() > 0
            // .iter()
            // .any(|row| row.iter().any(|(_x, _y, visited)| !visited))
            {
                let (x, y) = just_flashed.pop().unwrap();
                let x = x as isize;
                let y = y as isize;
                let neighbors = [
                    (x - 1, y - 1),
                    (x - 1, y),
                    (x - 1, y + 1),
                    (x, y - 1),
                    (x, y + 1),
                    (x + 1, y - 1),
                    (x + 1, y),
                    (x + 1, y + 1),
                ];
                for (nx, ny) in neighbors.into_iter() {
                    if nx >= 0 && ny >= 0 && nx < width && ny < height {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        let energy = &mut octopi[ny][nx].2;
                        *energy += 1;
                        if *energy > 9 {
                            let (ox, oy, _energy) = is_flashing[ny][nx];
                            // println!("looked for {}x{}, but was given {}x{}", nx,ny,ox,oy);
                            assert!(ox == nx);
                            assert!(oy == ny);

                            if !is_flashing[ny][nx].2 {
                                recent_flashes += 1;
                                just_flashed.push((nx, ny));
                                is_flashing[ny][nx].2 = true;
                            }
                        }
                    }
                }
            }
            // reset excited octopi back to 0
            octopi.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|(_, _, o)| {
                    if *o > 9 {
                        *o = 0;
                    }
                })
            });
            // println!("during step {}, there were {} flashes", step, recent_flashes);
            // print_octopi(&octopi);
            flashes += recent_flashes;
        }

        println!("parta: {}", flashes);
    }
    {
        let mut octopi = octopi.clone();

        let sync = (1..)
            .scan(octopi, |octopi, step| -> Option<(usize, Vec<bool>)> {
                octopi
                    .iter_mut()
                    .for_each(|row| row.iter_mut().for_each(|(_, _, o)| *o += 1));

                let mut is_flashing: Vec<Vec<(usize, usize, bool)>> = octopi
                    .iter()
                    .map(|row| row.iter().map(|(x, y, o)| (*x, *y, *o > 9)).collect())
                    .collect();

                let mut just_flashed: Vec<(usize, usize)> = is_flashing
                    .iter()
                    .flat_map(|row| {
                        row.iter()
                            .filter(|(_x, _y, f)| *f)
                            .map(|(x, y, _)| (*x, *y))
                    })
                    .collect();

                while just_flashed.len() > 0 {
                    let (x, y) = just_flashed.pop().unwrap();
                    let x = x as isize;
                    let y = y as isize;
                    let neighbors = [
                        (x - 1, y - 1),
                        (x - 1, y),
                        (x - 1, y + 1),
                        (x, y - 1),
                        (x, y + 1),
                        (x + 1, y - 1),
                        (x + 1, y),
                        (x + 1, y + 1),
                    ];
                    for (nx, ny) in neighbors.into_iter() {
                        if nx >= 0 && ny >= 0 && nx < width && ny < height {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            let energy = &mut octopi[ny][nx].2;
                            *energy += 1;
                            if *energy > 9 {
                                let (ox, oy, _energy) = is_flashing[ny][nx];

                                assert!(ox == nx);
                                assert!(oy == ny);

                                if !is_flashing[ny][nx].2 {
                                    just_flashed.push((nx, ny));
                                    is_flashing[ny][nx].2 = true;
                                }
                            }
                        }
                    }
                }
                let flashed: Vec<bool> = octopi
                    .iter()
                    .flat_map(|row| {
                        row.iter()
                            .map(|(_, _, energy)| *energy > 9)
                            .collect::<Vec<_>>()
                    })
                    .collect();
                octopi.iter_mut().for_each(|row| {
                    row.iter_mut().for_each(|(_, _, o)| {
                        if *o > 9 {
                            *o = 0;
                        }
                    })
                });

                Some((step, flashed))
            })
            .filter(|(_step, octopi)| octopi.iter().all(|e| *e))
            .next()
            .map(|(step, _)| step)
            .unwrap();

        // for step in 1..=100 {
        //     octopi
        //         .iter_mut()
        //         .for_each(|row| row.iter_mut().for_each(|(_, _, o)| *o += 1));

        //     let mut is_flashing: Vec<Vec<(usize, usize, bool)>> = octopi
        //         .iter()
        //         .map(|row| row.iter().map(|(x, y, o)| (*x, *y, *o > 9)).collect())
        //         .collect();

        //     let mut just_flashed: Vec<(usize, usize)> = is_flashing
        //         .iter()
        //         .flat_map(|row| {
        //             row.iter()
        //                 .filter(|(_x, _y, f)| *f)
        //                 .map(|(x, y, _)| (*x, *y))
        //         })
        //         .collect();

        //     while just_flashed.len() > 0 {
        //         let (x, y) = just_flashed.pop().unwrap();
        //         let x = x as isize;
        //         let y = y as isize;
        //         let neighbors = [
        //             (x - 1, y - 1),
        //             (x - 1, y),
        //             (x - 1, y + 1),
        //             (x, y - 1),
        //             (x, y + 1),
        //             (x + 1, y - 1),
        //             (x + 1, y),
        //             (x + 1, y + 1),
        //         ];
        //         for (nx, ny) in neighbors.into_iter() {
        //             if nx >= 0 && ny >= 0 && nx < width && ny < height {
        //                 let nx = nx as usize;
        //                 let ny = ny as usize;
        //                 let energy = &mut octopi[ny][nx].2;
        //                 *energy += 1;
        //                 if *energy > 9 {
        //                     let (ox, oy, _energy) = is_flashing[ny][nx];

        //                     assert!(ox == nx);
        //                     assert!(oy == ny);

        //                     if !is_flashing[ny][nx].2 {
        //                         just_flashed.push((nx, ny));
        //                         is_flashing[ny][nx].2 = true;
        //                     }
        //                 }
        //             }
        //         }
        //     }

        //     octopi.iter_mut().for_each(|row| {
        //         row.iter_mut().for_each(|(_, _, o)| {
        //             if *o > 9 {
        //                 *o = 0;
        //             }
        //         })
        //     });
        // }
        println!("partb: {}", sync);
    }
}

fn print_octopi(octopi: &Vec<Vec<(usize, usize, u32)>>) {
    for line in octopi.iter() {
        for (_, _, o) in line.iter() {
            print!("{}", o)
        }
        println!("");
    }
}
