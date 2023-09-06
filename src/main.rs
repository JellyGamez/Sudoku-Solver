use std::io::{self, BufRead};
use std::time::Instant;
use std::fs::File;

mod sudoku;
use crate::sudoku::Sudoku;
mod config;
use crate::config::
{
    FILENAME as filename, 
    GRID_LIMIT as grid_limit, 
    PRINT_STATS as print_stats,
    PRINT_SOLVED_GRIDS as print_solved_grids,
    FEWEST_CANDIDATES_HEURISTIC as heuristic
};

fn main() 
{
    let mut solved = 0;
    let mut board = [0; 81];

    let start = Instant::now();

    if let Ok(file) = File::open(filename)
    {
        let lines = io::BufReader::new(file).lines();

        for (i, line) in lines.enumerate()
        {
            if i == grid_limit
            {
                break;
            }
            if let Ok(grid) = line
            {
                for (i, digit) in grid.trim().bytes().enumerate()
                {
                    board[i] = digit as u32 - '0' as u32;
                }

                let mut sudoku: Sudoku = Sudoku::new(board, heuristic);
                sudoku.solve(0);

                if sudoku.solved
                {
                    solved += 1;
                }

                if print_solved_grids
                {
                    println!("{}", sudoku);
                }

            }
        }

        let elapsed = start.elapsed().as_secs_f32();

        if print_stats
        {
            println!("Grids solved: {}/{grid_limit}", solved);
            println!("Time elapsed: {} seconds", elapsed);
            println!("Average time per grid: {} seconds", elapsed / grid_limit as f32);
            println!("Average grids per second: {}", grid_limit as f32 / elapsed);
        }
    }
    else 
    {
        eprintln!("File not found.");
    }

}