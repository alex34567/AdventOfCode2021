use std::collections::HashMap;

static INPUT: &str = include_str!("input/Day4.txt");

const BINGO: u8 = 0x1F;

struct WinInfo {
    score: i32,
    num: u8,
}

struct BingoBoard {
    nums: [[u8; 5]; 5],
    x_marks: [u8; 5],
    y_marks: [u8; 5],
    win_info: Option<WinInfo>,
}

pub fn day4() {
    let mut input_boards = INPUT.trim().split("\n\n");
    let bingo_numbers = input_boards
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap());
    let mut bingo_boards = input_boards
        .map(|board| {
            let nums = <[[u8; 5]; 5]>::try_from(
                board
                    .split('\n')
                    .map(|line| {
                        <[u8; 5]>::try_from(
                            line.split(' ')
                                .filter(|x| !x.trim().is_empty())
                                .map(|num| num.parse::<u8>().unwrap())
                                .collect::<Vec<_>>(),
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            BingoBoard {
                nums,
                x_marks: [0; 5],
                y_marks: [0; 5],
                win_info: None,
            }
        })
        .collect::<Vec<_>>();

    let mut bingo_quick_num = HashMap::new();
    for (b_id, board) in bingo_boards.iter().enumerate() {
        for (y, line) in board.nums.iter().enumerate() {
            for (x, num) in line.iter().enumerate() {
                let boards_with_num = bingo_quick_num.entry(*num).or_insert_with(Vec::new);
                boards_with_num.push((b_id, y, x));
            }
        }
    }

    let mut first_won = None;
    let mut last_won = None;

    for (num, boards) in
        bingo_numbers.filter_map(|num| bingo_quick_num.get(&num).map(|boards| (num, boards)))
    {
        for (b_id, y, x) in boards.iter().map(|x| *x) {
            let board = &mut bingo_boards[b_id];
            if board.win_info.is_some() {
                continue;
            }
            let y_bit = 1 << y;
            let x_bit = 1 << x;
            board.x_marks[y] |= x_bit;
            board.y_marks[x] |= y_bit;
            if board.x_marks[y] == BINGO || board.y_marks[x] == BINGO {
                let score = board
                    .nums
                    .iter()
                    .enumerate()
                    .flat_map(|(y, line)| line.iter().enumerate().map(move |a| (y, a)))
                    .filter_map(|(y, (x, num))| {
                        let y_bit = 1 << y;
                        let marked = board.y_marks[x] & y_bit != 0;
                        Some((*num) as i32).filter(|_| !marked)
                    })
                    .sum::<i32>();
                board.win_info = Some(WinInfo { score, num });
                if first_won.is_none() {
                    first_won = Some(b_id)
                }
                last_won = Some(b_id)
            }
        }
    }

    let first_won_board = bingo_boards[first_won.unwrap()].win_info.as_ref().unwrap();
    let last_won_board = bingo_boards[last_won.unwrap()].win_info.as_ref().unwrap();

    println!(
        "Part1: {}",
        first_won_board.score * (first_won_board.num as i32)
    );
    println!(
        "Part2: {}",
        last_won_board.score * (last_won_board.num as i32)
    );
}
