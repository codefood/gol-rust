use grid::*;
use std::{thread, time};

fn main() {
    let mut board = grid![
            [ true, false, false ]
            [ false, true, true ]
            [ true, false, false ]
        ];
    loop {
        board = iterate(board);
        let duration = time::Duration::from_millis(300);
        thread::sleep(duration);
    }
}

fn iterate(board: Grid<bool>) -> Grid<bool> {
    println!("{board:?}");
    let mut new_board = Grid::<bool>::new(board.rows(), board.cols());

    for ((row, col), i) in board.indexed_iter() {
        let neighbours : i32 = count_live_neighbours(&board, row, col);
        new_board[(row, col)] = *i;
        if neighbours == 0 {
            new_board[(row, col)] = false;
        }
        if neighbours == 3 && !*i {
            new_board[(row, col)] = true;
        }
        if neighbours >= 4 {
            new_board[(row, col)] = false;
        }
    };
    return new_board;
}

fn count_live_neighbours(board: &Grid<bool>, row: usize, col: usize) -> i32 {
    let mut count = 0;
    if board.get(row, col + 1).is_some_and(|x| x == &true) {
        count += 1;
    }
    if board.get(row + 1, col).is_some_and(|x| x == &true) {
        count += 1;
    }
    if board.get(row + 1, col + 1).is_some_and(|x| x == &true) {
        count += 1;
    }
    if row > 0 {
        if board.get(row - 1, col).is_some_and(|x| x == &true) {
            count += 1;
        }
        if board.get(row - 1, col + 1).is_some_and(|x| x == &true) {
            count += 1;
        }
        if col > 0 {
            if board.get(row - 1, col - 1).is_some_and(|x| x == &true) {
                count += 1;
            }
        }
    }
    if col > 0 {
        if board.get(row, col - 1).is_some_and(|x| x == &true) {
            count += 1;
        }
        if board.get(row + 1, col - 1).is_some_and(|x| x == &true) {
            count += 1;
        }
        if row > 0 {
            if board.get(row - 1, col - 1).is_some_and(|x| x == &true) {
                count += 1;
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_board() -> Grid<bool> {
        let board = grid![
            [ false, false, false ]
            [ false, false, false ]
            [ false, false, false ]
        ];
        return board;
    }
 
    #[test]
    fn given_solitary_cell_dies() {
        let mut board = default_board();
        board[(1, 1)] = true;

        let output = iterate(board);
        assert_all_dead(output);
    }

    #[test]
    fn given_overpopulated_cell_dies() {
        let mut board = default_board();
        board[(0, 0)] = true;
        board[(0, 1)] = true;
        board[(0, 2)] = true;
        board[(1, 0)] = true;
        board[(1, 1)] = true;
        let output = iterate(board);
        //assert_all_dead(output);
        assert_eq!(output.get(1, 1).is_some_and(|x| x == &false), true);
    }

    #[test]
    fn given_two_neighbours_survives() {
        let mut board = default_board();
        board[(0, 0)] = true;
        board[(0, 1)] = true;
        board[(1, 0)] = true;
        let output = iterate(board);
        assert_eq!(output.get(1, 0).is_some_and(|x| x == &true), true);
    }

    #[test]
    fn given_three_neighbours_survives() {
        let mut board = default_board();
        board[(0, 0)] = true;
        board[(0, 1)] = true;
        board[(1, 0)] = true;
        board[(1, 1)] = true;
        let output = iterate(board);
        assert_eq!(output.get(1, 0).is_some_and(|x| x == &true), true);
    }
    
    #[test]
    fn given_three_neighbours_becomes_alive() {
        let mut board = default_board();
        board[(0, 0)] = true;
        board[(0, 1)] = true;
        board[(1, 0)] = false;
        board[(1, 1)] = true;
        let output = iterate(board);
        assert_eq!(output.get(1, 0).is_some_and(|x| x == &true), true);
    }

    fn assert_all_dead(board : Grid<bool>) {
        for ((_row, _col), i) in board.indexed_iter() {
            assert_eq!(i, &false);
        };
    }

}