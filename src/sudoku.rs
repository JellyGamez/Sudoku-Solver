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
                    used_row[row] |= 1 << digit;
                    used_col[col] |= 1 << digit;
                    used_box[ind] |= 1 << digit;
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

        for digit in 1..10
        {
            let (row_bit, col_bit, box_bit) = (row_bit << (digit - 1), col_bit << (digit - 1), box_bit << (digit - 1));
            if (self.used_row & row_bit) | (self.used_col & col_bit) | (self.used_box & box_bit) == 0
            {
                self.board[row][col] = digit as i32;

                if pos == self.empty.len() - 1
                {
                    self.solved = true;
                    // self.print();
                    // println!("{:b}", self.used_row);
                    // println!("{:b}", self.used_col);
                    // println!("{:b}", self.used_box);
                    return;
                }

                self.used_row |= row_bit;
                self.used_col |= col_bit;
                self.used_box |= box_bit;

                self.solve(pos + 1);
                
                if self.solved
                {
                    return;
                }

                self.used_row ^= row_bit;
                self.used_col ^= col_bit;
                self.used_box ^= box_bit;
            }
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