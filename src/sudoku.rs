use std::fmt;
pub struct Sudoku
{
    board: [[u32; 9]; 9],
    used_row: [u32; 9],
    used_col: [u32; 9],
    used_box: [u32; 9],
    empty: Vec<(usize, usize, usize)>,
    pub solved: bool
}
impl Sudoku
{
    pub fn new(board: [[u32; 9]; 9]) -> Self 
    {
        let mut empty: Vec<(usize, usize, usize)> = Vec::new();
        let (mut used_row, mut used_col, mut used_box) = ([0; 9], [0; 9], [0; 9]);
        for row in 0..9
        {
            for col in 0..9
            {
                let digit = board[row][col];
                let ind = row / 3 * 3 + col / 3;
                if board[row][col] == 0
                {
                    empty.push((row, col, ind));
                }
                else 
                {
                    let digit = digit as usize;
                    used_row[row] |= 1 << (digit - 1);
                    used_col[col] |= 1 << (digit - 1);
                    used_box[ind] |= 1 << (digit - 1);
                }
            }
        }
        Self 
        {
            board: board,
            used_row: used_row,
            used_col: used_col,
            used_box: used_box,
            empty: empty,
            solved: false
        }
    }

    pub fn solve(&mut self, pos: usize)
    {
        if self.solved
        {
            return;
        }
        let (row, col, ind) = self.empty[pos];
        let mut candidates = self.used_row[row] | self.used_col[col] | self.used_box[ind];

        while candidates != 511
        {
            let candidate = 1u32.checked_shl(candidates.trailing_ones()).unwrap_or(0);
            
            self.board[row][col] = candidates.trailing_ones() + 1;

            if pos == self.empty.len() - 1
            {
                self.solved = true;
                //println!("{}", self);
                return;
            }

            self.used_row[row] |= candidate;
            self.used_col[col] |= candidate;
            self.used_box[ind] |= candidate;

            self.solve(pos + 1);

            self.used_row[row] ^= candidate;
            self.used_col[col] ^= candidate;
            self.used_box[ind] ^= candidate;

            candidates |= candidate;
        }
    }

    // pub fn get_best(&self) -> usize
    // {
    //     let mut best = 0;
    //     let mut set = 300;
    //     for (i, (row, col, ind)) in self.empty.iter().enumerate()
    //     {
    //         let cur = (self.used_row[*row] | self.used_col[*col] | self.used_box[*ind]).count_zeros();
    //         if cur != 23 && set > cur
    //         {
    //             set = cur;
    //             best = i;
    //         }
    //     }
    //     best
    // }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+-------+-------+-------+")?;
        for i in 0..9
        {
            write!(f, "| ")?;
            for j in 0..9
            {
                write!(f, "{} ", self.board[i][j])?;
                if j % 3 == 2
                {
                    write!(f, "| ")?;
                }
            }
            writeln!(f)?;
            if i % 3 == 2
            {
                writeln!(f, "+-------+-------+-------+")?;
            }
        }
        writeln!(f)
    }
}