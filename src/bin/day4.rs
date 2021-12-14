const INPUT: &str = include_str!("../../input/day4.txt");

const WINNING_SEQUENCES: &[&[u32]] = &[
    &[
        1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    &[
        1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    ],
    &[
        0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0,
    ],
    &[
        0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0,
    ],
    &[
        0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0,
    ],
    &[
        0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
    ],
    &[
        0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    &[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    &[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
    ],
    &[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,
    ],
];

const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

fn main() {
    println!("day 4");
    {
        let mut lines = INPUT.lines();
        let win_sequence: Vec<u32> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let (mut boards, board): (Vec<Vec<(u32, bool)>>, Vec<(u32, bool)>) =
            lines.fold((Vec::new(), Vec::new()), |(mut boards, mut board), line| {
                if line.len() == 0 {
                    if board.len() > 0 {
                        boards.push(board);
                    }
                    (boards, Vec::new())
                } else {
                    let mut board_line: Vec<(u32, bool)> = line
                        .split(' ')
                        .filter_map(|n| {
                            // println!("# {}", n);
                            n.parse::<u32>().ok()
                        })
                        .map(|n| (n, false))
                        .collect();
                    board.append(&mut board_line);
                    (boards, board)
                }
            });

        if board.len() > 0 {
            boards.push(board);
        }

        // println!("win sequence: {:?}", win_sequence);
        // println!("boards: {:?}", boards);

        let (first_win, winning_number): (Vec<(u32, bool)>, u32) = win_sequence
            .iter()
            .scan(boards, |state, next| {
                println!("marking {} on all {} boards", next, state.len());
                let mut winning_boards = Vec::new();
                let mut remove_list = Vec::new();
                for (i, board) in state.iter_mut().enumerate() {
                    for square in board.iter_mut() {
                        if square.0 == *next {
                            square.1 = true;
                        }
                    }
                    if WINNING_SEQUENCES.iter().any(|winning_board| {
                        winning_board
                            .iter()
                            .zip(board.iter())
                            .all(|(win, (_, test))| *win == 0 || *test)
                    }) {
                        println!("winner: {}", i);
                        winning_boards.push((board.clone(), *next));
                        remove_list.push(i);
                    }
                }
                let mut i = 0;
                while i < state.len() {
                    if remove_list.contains(&i) {
                        println!("removing board {}", i);
                        let _ = state.remove(i);
                    }
                    i += 1;
                }
                Some(winning_boards)
            })
            .filter(|w| w.len() > 0)
            .next()
            .unwrap()
            .pop()
            .unwrap();
        let unmarked_sum: u32 = first_win
            .iter()
            .filter(|(_num, mark)| !*mark)
            .map(|(n, _)| n)
            .sum();

        println!("first_win {:?}", first_win);
        println!("unmarked sum: {}", unmarked_sum);
        println!("last number called: {}", winning_number);
        println!("parta {}", unmarked_sum * winning_number);
    }

    {
        let mut lines = INPUT.lines();
        let win_sequence: Vec<u32> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let (mut boards, board): (Vec<Vec<(u32, bool)>>, Vec<(u32, bool)>) =
            lines.fold((Vec::new(), Vec::new()), |(mut boards, mut board), line| {
                if line.len() == 0 {
                    if board.len() > 0 {
                        boards.push(board);
                    }
                    (boards, Vec::new())
                } else {
                    let mut board_line: Vec<(u32, bool)> = line
                        .split(' ')
                        .filter_map(|n| {
                            // println!("# {}", n);
                            n.parse::<u32>().ok()
                        })
                        .map(|n| (n, false))
                        .collect();
                    board.append(&mut board_line);
                    (boards, board)
                }
            });

        if board.len() > 0 {
            boards.push(board);
        }

        // println!("win sequence: {:?}", win_sequence);
        // println!("boards: {:?}", boards);

        let mut all_wins: Vec<Vec<(Vec<(u32, bool)>, u32)>> = win_sequence
            .iter()
            .scan(boards, |state, next| {
                if state.len() == 0 {
                    return None;
                }
                println!("marking {} on all {} boards", next, state.len());
                let mut winning_boards = Vec::new();
                let mut remove_list = Vec::new();
                for (i, board) in state.iter_mut().enumerate() {
                    for square in board.iter_mut() {
                        if square.0 == *next {
                            square.1 = true;
                        }
                    }
                    if WINNING_SEQUENCES.iter().any(|winning_board| {
                        winning_board
                            .iter()
                            .zip(board.iter())
                            .all(|(win, (_, test))| *win == 0 || *test)
                    }) {
                        println!("winner: {}", i);
                        winning_boards.push((board.clone(), *next));
                        remove_list.push(i);
                    }
                }
                let mut i = 0;
                let mut removed = 0;
                while i < state.len() {
                    if remove_list.contains(&(i + removed)) {
                        println!("removing board {}", i);
                        let _ = state.remove(i);
                        removed += 1;
                    }
                    i += 1;
                }
                Some(winning_boards)
            })
            .filter(|w| w.len() > 0)
            .collect();
        let (last_win, winning_number) = all_wins.pop().unwrap().pop().unwrap();
        let unmarked_sum: u32 = last_win
            .iter()
            .filter(|(_num, mark)| !*mark)
            .map(|(n, _)| n)
            .sum();

        println!("last_win {:?}", last_win);
        println!("unmarked sum: {}", unmarked_sum);
        println!("last number called: {}", winning_number);
        println!("partb: {}", unmarked_sum * winning_number);
    }
}
