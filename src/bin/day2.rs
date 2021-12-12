mod input;

fn main() {
    println!("day 2");
    let input = input::day2::INPUT;

    {
        let (x, y) = input
            .lines()
            .map(|line| {
                let (direction, amount) = line.split_once(' ').unwrap();
                let amount = amount.parse::<i32>().unwrap();
                match direction {
                    "forward" => (amount, 0),
                    "down" => (0, amount),
                    "up" => (0, -amount),
                    _ => panic!(""),
                }
            })
            .reduce(|(x, y), (next_x, next_y)| (x + next_x, y + next_y))
            .unwrap();

        println!("parta: {}", x * y);
    }

    {
        let (_, x, y) = input
            .lines()
            .map(|line| {
                let (direction, amount) = line.split_once(' ').unwrap();
                let amount = amount.parse::<i32>().unwrap();
                match direction {
                    "forward" => (amount, 0),
                    "down" => (0, amount),
                    "up" => (0, -amount),
                    _ => panic!(""),
                }
            })
            .fold((0, 0, 0), |(aim, x, y), (magnitude, delta_aim)| {
                (aim + delta_aim, x + magnitude, y + (aim * magnitude))
            });

        println!("partb: {}", x * y);
    }
}
