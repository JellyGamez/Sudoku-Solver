pub struct Sudoku
{
    board: [[i32; 10]; 10],
    used_row: [[bool; 10]; 10],
    used_col: [[bool; 10]; 10],
    used_box: [[bool; 10]; 10],
    empty: Vec<(usize, usize)>,
    solved: bool
}
impl Sudoku
{
    pub fn new(board: [[i32; 10]; 10]) -> Self 
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
            if !self.used_row[row][digit] && !self.used_col[col][digit] && !self.used_box[ind][digit]
            {
                self.board[row][col] = digit as i32;

                if pos == self.empty.len() - 1
                {
                    self.solved = true;
                    //self.print();
                    return;
                }

                self.used_row[row][digit] = true;
                self.used_col[col][digit] = true;
                self.used_box[ind][digit] = true;

                self.bkt(pos + 1);
                
                if self.solved
                {
                    return;
                }

                self.used_row[row][digit] = false;
                self.used_col[col][digit] = false;
                self.used_box[ind][digit] = false;
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