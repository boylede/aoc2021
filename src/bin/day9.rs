mod input;

const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

fn main() {
    println!("day 9");
    let input = input::day9::INPUT;
    let heightmap: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let width = heightmap[0].len() as isize;
    let height = heightmap.len() as isize;
    {
        // println!("found heightmap {}x{}", width, height);
        let points = heightmap
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| ((x, y), *cell))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let low_points = points
            .iter()
            .filter(|((x, y), v)| {
                let x = *x as isize;
                let y = *y as isize;
                let neighbors = [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];
                // println!("is {}@{}x{} a low point?", v, x,y);
                for (nx, ny) in neighbors.iter() {
                    if *nx >= 0 && *ny >= 0 && *nx < width && *ny < height {
                        let nv = heightmap[*ny as usize][*nx as usize];
                        // println!("\tchecking {} <= {}", nv, *v);
                        if nv <= *v {
                            // println!("\tdetermined was not a low point");
                            return false;
                        }
                        // else {
                        //     println!("{} > {}", v, heightmap[ny as usize][nx as usize]);
                        // }
                    } else {
                        // println!("\tskipped checking {}x{} because it is out of bounds", nx, ny);
                    }
                }

                // println!("\tfound low point {} at {}x{}", v, x, y);
                return true;
            })
            .collect::<Vec<_>>();
        let low_point_sum: u32 = low_points.iter().map(|(_p, v)| *v + 1).sum();
        // println!("found {} low points", low_points.len());
        println!("parta: {}", low_point_sum);
    }
    {
        let low_points: Vec<(usize, usize, u32)> = heightmap
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| (x, y, *cell)))
            .filter(|(x, y, v)| {
                let x = *x as isize;
                let y = *y as isize;
                let neighbors = [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];

                for (nx, ny) in neighbors.iter() {
                    if *nx >= 0 && *ny >= 0 && *nx < width && *ny < height {
                        let nv = heightmap[*ny as usize][*nx as usize];

                        if nv <= *v {
                            return false;
                        }
                    }
                }

                // println!("\tfound low point {} at {}x{}", v, x, y);
                return true;
            })
            .collect::<Vec<_>>();
        let mut largest_basins: Vec<usize> = low_points
            .into_iter()
            .map(|(x, y, v)| -> Vec<(usize, usize, u32)> {
                // println!("expanding low point ({},{},{}) into a basin", x, y, v);
                let mut basin = vec![(x, y, v)];
                let x = x as isize;
                let y = y as isize;
                let mut visit_queue = vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];
                let mut visited = vec![(x,y)];

                while visit_queue.len() > 0 {
                    if let Some((nx, ny)) = visit_queue.pop() {
                        // println!("visiting {}x{} to see if it is in this basin", nx, ny);
                        if nx >= 0 && ny >= 0 && nx < width && ny < height && !visited.contains(&(nx,ny)) {
                            let nv = heightmap[ny as usize][nx as usize];

                            if nv != 9 {
                                // println!("it wasnt 9, so it must be: {}", nv);
                                let new_neighbors =
                                    [(nx - 1, ny), (nx, ny - 1), (nx + 1, ny), (nx, ny + 1)];
                                visit_queue.extend_from_slice(&new_neighbors);
                                basin.push((nx as usize, ny as usize, nv));
                            } else {
                                // println!(" it was 9, so no");
                            }
                        } else {
                            // println!("either it is out of bounds or we've been there before");
                        }
                        visited.push((nx,ny));
                    }
                }
                // println!("found a basin with {} points", basin.len());
                basin
            })
            
            .map(|b| b.len())
            .collect();
        largest_basins.sort();
        let basin_product: usize = largest_basins.into_iter().rev().take(3).product();
        println!("partb: {}", basin_product);
    }
}
