use std::io::{self, BufRead};
use std::time::Instant;
use std::fs::File;

mod sudoku;
use crate::sudoku::Sudoku;

fn main() 
{
    let mut solved = 0;
    let mut board = [[0; 9]; 9];

    let filename = "./datasets/easy.txt";
    let limit = 1;

    let start = Instant::now();

    if let Ok(file) = File::open(filename)
    {
        let lines = io::BufReader::new(file).lines();
        let lines = lines;

        for (i, line) in lines.enumerate()
        {
            if i == limit
            {
                break;
            }
            if let Ok(grid) = line
            {
                for (i, digit) in grid.trim().bytes().enumerate()
                {
                    board[i / 9][i % 9] = digit as u32 - '0' as u32;
                }
                let mut sudoku: Sudoku = Sudoku::new(board);
                sudoku.solve(0);
                if sudoku.solved
                {
                    solved += 1;
                }
            }
        }

        let elapsed = start.elapsed().as_secs_f32();

        println!("Grids solved: {}/{limit}", solved);
        println!("Time elapsed: {} seconds", elapsed);
        println!("Average time per grid: {} seconds", elapsed / limit as f32);
        println!("Average grids per second: {}", limit as f32 / elapsed);
    }
    else 
    {
        eprintln!("File not found.");
    }

}