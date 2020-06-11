pub struct Tour{
    board: [i8; 64],
    current_pos: usize,
    offset_x : [i8; 8],
    offset_y : [i8; 8],
    current_move: i8
}

impl Tour {
    pub fn new(start : usize) -> Tour{
        let mut tour = Tour{
            board: [0i8; 64],
            current_pos: start,
            offset_x: [   1,  2,  2,  1,  -1, -2, -2, -1],
            offset_y: [   2,  1,  -1, -2, -2, -1, 1,  2],
            current_move: 1i8
        };

        tour.board[start] = tour.current_move;
        return tour;
    }

    pub fn is_solved(&mut self) -> bool{
        !self.board.contains(&0i8)
    }

    pub fn get_move_list(&self) -> Vec<usize>{
        let mut moves = Vec::new();

        for n in 0..self.offset_x.len() {
            let pos_x = ((self.current_pos as i8) % 8) + self.offset_x[n];
            let pos_y = ((self.current_pos as i8) / 8) + self.offset_y[n];

            if pos_x >= 0 && pos_x <= 7 && pos_y >= 0 && pos_y <= 7{
                let pos = pos_x + (pos_y * 8);

                if self.board[pos as usize] == 0{
                    moves.push(pos as usize);
                }
            } 
        }

        return moves;
    }

    pub fn set_move(&mut self, index : usize) -> bool{
        if self.get_move_list().contains(&index) {
            self.current_move += 1;
            self.current_pos = index;
            self.board[index] = self.current_move;

            return true;
        }

        return false;
    }

    pub fn move_back(&mut self) -> bool{
        if self.current_move != 1 {
            let pos = self.current_pos;
            self.current_move -= 1;

            for n in 0..64 {
                if self.current_move == self.board[n] {
                    self.current_pos = n;
                }
            }

            self.board[pos] = 0;
            return true;
        }

        return false;
    }

    pub fn print_board(&self){
        for x in 0..8 {
            for y in 0..8{
                print!("[{}]\t", self.board[y + (x * 8)]);
            }
            print!("\n");
        }
        print!("\n");
    }
}   