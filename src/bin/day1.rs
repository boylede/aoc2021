const INPUT: &str = include_str!("../../input/day1.txt");
fn main() {
    println!("day 1");
    
    let soundings: Vec<i32> = INPUT
        .split('\n')
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();
    {
        let increasing_count = soundings
            .iter()
            .zip(soundings.clone().iter().skip(1))
            .filter(|(a, b)| b > a)
            .count();
        println!("parta: {}", increasing_count);
    }
    {
        let smooth_soundings: Vec<i32> = soundings.windows(3).map(|t| t.iter().sum()).collect();
        let increasing_count = smooth_soundings
            .iter()
            .zip(smooth_soundings.clone().iter().skip(1))
            .filter(|(a, b)| b > a)
            .count();
        println!("partb: {}", increasing_count);
    }
}
