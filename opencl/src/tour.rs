//The struct with all info for a knight's tour solver
pub struct Tour{
    board: Vec<i8>,
    current_pos: usize,
    offset_x : [i8; 8],
    offset_y : [i8; 8],
    current_move: i8,
    board_size : usize
}

impl Tour {
    //Contructor for the tour solver
    pub fn new(start : usize, size : usize) -> Tour{
        let mut tour = Tour{
            board: vec![0; size.pow(2)],
            current_pos: start,
            offset_x: [   1,  2,  2,  1,  -1, -2, -2, -1],
            offset_y: [   2,  1,  -1, -2, -2, -1, 1,  2],
            current_move: 1i8,
            board_size: size
        };

        tour.board[start] = tour.current_move;
        return tour;
    }

    //Checks if the board has been solved
    pub fn is_solved(&mut self) -> bool{
        !self.board.contains(&0i8)
    }

    //Returns a vector containting all possible moves from the current position
    pub fn get_move_list(&self) -> Vec<usize>{
        let mut moves = Vec::new();

        for n in 0..self.offset_x.len() {
            let pos_x = ((self.current_pos as i8) % (self.board_size as i8)) + self.offset_x[n];
            let pos_y = ((self.current_pos as i8) / (self.board_size as i8)) + self.offset_y[n];

            if  pos_x >= 0 && 
                pos_x <= ((self.board_size as i8) - 1) && 
                pos_y >= 0 && 
                pos_y <= ((self.board_size as i8) - 1) {

                let pos = pos_x + (pos_y * self.board_size as i8);

                if self.board[pos as usize] == 0{
                    moves.push(pos as usize);
                }
            } 
        }

        return moves;
    }

    //Move from the current position to the new
    pub fn set_move(&mut self, index : usize) -> bool{
        if self.get_move_list().contains(&index) {
            self.current_move += 1;
            self.current_pos = index;
            self.board[index] = self.current_move;

            return true;
        }

        return false;
    }

    //Go back one position from the current position
    pub fn move_back(&mut self) -> bool{
        if self.current_move != 1 {
            let pos = self.current_pos;
            self.current_move -= 1;

            for n in 0..self.board_size.pow(2) {
                if self.current_move == self.board[n] {
                    self.current_pos = n;
                }
            }

            self.board[pos] = 0;
            return true;
        }

        return false;
    }

    #[allow(dead_code)]
    pub fn print_board(&self){
        for x in 0..self.board_size {
            for y in 0..self.board_size{
                print!("[{}]\t", self.board[y + (x * self.board_size)]);
            }
            print!("\n");
        }
        print!("\n");
    }

    //The solving algorithm for the knight's tour
    pub fn solve(&mut self) -> Vec<i8>{
        //Stack for possible moves per node
        let mut nodes : Vec<Vec<usize>> = Vec::new();

        //Push all moves from the start position
        nodes.push(self.get_move_list());

        loop{
            //Check if the board is solved and return the solution if true
            if self.is_solved() {
                return self.board.to_vec();
            }

            //Get the move list for the current position
            let node_option = nodes.pop();
            let mut node;

            match node_option {
                None => break,
                _ => node = node_option.unwrap()
            }

            //Check if their are still moves possible
            if node.len() > 0 {
                //Get the first move in the list
                let next_move = node.pop().unwrap();
                //Put the moves back on the node stack
                nodes.push(node);
                
                //Perform the move the new postion
                self.set_move(next_move);

                //Push the possible moves for the new position to the stack
                nodes.push(self.get_move_list());
            }else{
                //Go back one move and don't put the moves back on the stack as their are none possible
                self.move_back();
            }
        }

        //Return an empty vector if no moves are found
        return Vec::new();
    }
}