use std::fmt;
pub struct Sudoku
{
    board: [u32; 81],
    used_row: [u32; 9],
    used_col: [u32; 9],
    used_box: [u32; 9],
    empty: Vec<(usize, usize, usize)>,
    pub solved: bool,
    pub heuristic: bool,
}

impl Sudoku
{
    #[inline]
    pub fn new(board: [u32; 81], heuristic: bool) -> Sudoku 
    {
        let mut empty: Vec<(usize, usize, usize)> = Vec::new();
        let (mut used_row, mut used_col, mut used_box) = ([0; 9], [0; 9], [0; 9]);
        for row in 0..9
        {
            for col in 0..9
            {
                let digit = board[row * 9 + col];
                let ind = row / 3 * 3 + col / 3;
                if board[9 * row + col] == 0
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
        Sudoku
        {
            board: board,
            used_row: used_row,
            used_col: used_col,
            used_box: used_box,
            empty: empty,
            solved: false,
            heuristic: heuristic,

        }
    }

    //#[inline]
    pub fn solve(&mut self, pos: usize)
    {
        if self.solved 
        {
            return;
        }
        if pos == self.empty.len() {
            self.solved = true;
            return;
        }
        if self.heuristic
        {
            let mut best = 1;
            let mut fewest_candidates = 9;
            for i in pos..self.empty.len()
            {
                let (row, col, ind) = self.empty[i];
                let current_candidates = 9 - (self.used_row[row] | self.used_col[col] | self.used_box[ind]).count_ones();

                if current_candidates < fewest_candidates
                {
                    fewest_candidates = current_candidates;
                    best = i;
                }
            }
            (self.empty[pos], self.empty[best]) = (self.empty[best], self.empty[pos]);
        }
        let (row, col, ind) = self.empty[pos];
        let mut candidates = self.used_row[row] | self.used_col[col] | self.used_box[ind];

        while candidates != 511
        {
            let candidate = 1u32.checked_shl(candidates.trailing_ones()).unwrap_or(0);
            self.board[row * 9 + col] = candidates.trailing_ones() + 1;

            self.used_row[row] |= candidate;
            self.used_col[col] |= candidate;
            self.used_box[ind] |= candidate;

            self.solve(pos + 1);

            if self.solved 
            {
                return;
            }
            
            self.used_row[row] ^= candidate;
            self.used_col[col] ^= candidate;
            self.used_box[ind] ^= candidate;

            candidates |= candidate;
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+-------+-------+-------+")?;
        for i in 0..9
        {
            write!(f, "| ")?;
            for j in 0..9
            {
                write!(f, "{} ", self.board[i * 9 + j])?;
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