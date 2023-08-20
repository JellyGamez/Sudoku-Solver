pub struct Sudoku
{
    
    board: [[i32; 10]; 10],
    used_row: i128,
    used_col: i128,
    used_box: i128,
    empty: Vec<(usize, usize)>,
    pub solved: bool
}
impl Sudoku
{
    pub fn new(board: [[i32; 10]; 10]) -> Self 
    {
        let mut empty: Vec<(usize, usize)> = Vec::new();
        let (mut used_row, mut used_col, mut used_box) = (0, 0, 0);
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
                    let digit = digit as usize;
                    used_row |= 1 << (9 * row + digit);
                    used_col |= 1 << (9 * col + digit);
                    used_box |= 1 << (9 * ind + digit);
                    // used_row[row][digit as usize] = true;
                    // used_col[col][digit as usize] = true;
                    // used_box[ind][digit as usize] = true;
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

    pub fn bkt(&mut self, pos: usize)
    {
        if self.solved
        {
            return;
        }
        let (row, col) = self.empty[pos];
        let ind = (row - 1) / 3 * 3 + (col - 1) / 3 + 1;
        for digit in 1..10
        {
            let (row_bit, col_bit, box_bit) = (1 << (9 * row + digit), 1 << (9 * col + digit), 1 << (9 * ind + digit));
            if (self.used_row & row_bit) | (self.used_col & col_bit) | (self.used_box & box_bit) == 0
            {
                self.board[row][col] = digit as i32;

                if pos == self.empty.len() - 1
                {
                    self.solved = true;
                    //self.print();
                    return;
                }

                self.used_row |= row_bit;
                self.used_col |= col_bit;
                self.used_box |= box_bit;

                self.bkt(pos + 1);
                
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