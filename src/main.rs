use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};

static mut solved: i32 = 0;
struct Sudoku
{
    board: [[i32; 10]; 10],
    used_row: [[bool; 10]; 10],
    used_col: [[bool; 10]; 10],
    used_box: [[bool; 10]; 10],
    empty: Vec<(usize, usize)>,
    found: bool
}
impl Sudoku
{
    pub fn new(board: [[i32; 10]; 10]) -> Sudoku 
    {
        let mut empty: Vec<(usize, usize)> = Vec::new();
        let mut used_row = [[false; 10]; 10];
        let mut used_col = [[false; 10]; 10];
        let mut used_box = [[false; 10]; 10];
        for row in 1..10
        {
            for col in 1..10
            {
                let digit = board[row][col];
                let ind = (row - 1) / 3 * 3 + (col - 1) / 3 + 1;
                if board[row][col] == 0
                {
                    empty.push((row, col));
                }
                else 
                {
                    used_row[row][digit as usize] = true;
                    used_col[col][digit as usize] = true;
                    used_box[ind][digit as usize] = true;
                }
            }
        }
        Sudoku 
        {
            board: board,
            used_row: used_row,
            used_col: used_col,
            used_box: used_box,
            empty: empty,
            found: false
        }
    }
    pub fn bkt(&mut self, pos: usize)
    {
        if self.found
        {
            return;
        }
        let (row, col) = self.empty[pos];
        let ind = (row - 1) / 3 * 3 + (col - 1) / 3 + 1;
        for digit in 1..10
        {
            if !self.used_row[row][digit] && !self.used_col[col][digit] && !self.used_box[ind][digit]
            {
                self.board[row][col] = digit as i32;

                if pos == self.empty.len() - 1
                {
                    self.found = true;
                    unsafe 
                    {
                        solved += 1;
                    }
                    //self.print();
                    return;
                }

                self.used_row[row][digit] = true;
                self.used_col[col][digit] = true;
                self.used_box[ind][digit] = true;

                self.bkt(pos + 1);


                self.used_row[row][digit] = false;
                self.used_col[col][digit] = false;
                self.used_box[ind][digit] = false;
            }
        }
    }
    pub fn print(&self)
    {
        for i in 1..10
        {
            for j in 1..10
            {
                print!("{}", self.board[i][j]);
            }
            println!();
        }
    }
}

fn main() {
    let mut board = [[0; 10]; 10];
    let start = Instant::now();

    //file input used for benchmarking
    let filename = "./datasets/100000.txt";
    if let Ok(file) = File::open(filename)
    {
        let lines = io::BufReader::new(file).lines();
        for line in lines
        {
            if let Ok(grid) = line
            {
                for (i, digit) in grid.trim().bytes().enumerate()
                {
                    board[i / 9 + 1][i % 9 + 1] = digit as i32 - '0' as i32;
                }
                let mut sudoku: Sudoku = Sudoku::new(board);
                sudoku.bkt(0);
            }
        }
    }

    unsafe 
    {
        println!("Grids solved: {}", solved);
    }
    println!("Time elapsed: {:?}", start.elapsed());
}  
