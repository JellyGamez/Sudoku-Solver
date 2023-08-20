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

        //println!("{:b} {:b} {:b}", self.used_row[row], self.used_col[col], self.used_box[ind]);

        while candidates != 511
        {
            
            let candidate = 1u32.checked_shl(candidates.trailing_ones()).unwrap_or(0);

            //println!("{:b} {}", candidates, candidate);

            candidates |= candidate;
            self.board[row][col] = candidate;

            if pos == self.empty.len() - 1
            {
                self.solved = true;
                // self.print();
                return;
            }

            self.used_row[row] |= candidate;
            self.used_col[col] |= candidate;
            self.used_box[ind] |= candidate;

            self.solve(pos + 1);

            self.used_row[row] ^= candidate;
            self.used_col[col] ^= candidate;
            self.used_box[ind] ^= candidate;
        }
    }

    #[allow(dead_code)]
    pub fn print(&self)
    {
        for i in 1..10
        {
            for j in 1..10
            {
                print!("{} ", self.board[i][j]);
            }
            println!();
        }
        println!();
    }
}