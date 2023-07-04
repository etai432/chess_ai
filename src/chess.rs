use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Chess {
    pub board: [Piece; 64],
    pub moves: Vec<usize>,
    casting: [bool; 4], //white, white long, black, black long
    en_passant: usize,
    pub is_white_turn: bool,
}

impl Chess {
    pub fn new() -> Self {
        let mut board: [Piece; 64] = [Piece::Empty; 64];
        // Set up black pieces
        board[0] = Piece::Brook;
        board[1] = Piece::Bknight;
        board[2] = Piece::Bbishop;
        board[3] = Piece::Bqueen;
        board[4] = Piece::Bking;
        board[5] = Piece::Bbishop;
        board[6] = Piece::Bknight;
        board[7] = Piece::Brook;
        board
            .iter_mut()
            .take(16)
            .skip(8)
            .for_each(|piece| *piece = Piece::Bpawn);
        // Set up white pieces
        board[56] = Piece::Wrook;
        board[57] = Piece::Wknight;
        board[58] = Piece::Wbishop;
        board[59] = Piece::Wqueen;
        board[60] = Piece::Wking;
        board[61] = Piece::Wbishop;
        board[62] = Piece::Wknight;
        board[63] = Piece::Wrook;
        board
            .iter_mut()
            .take(56)
            .skip(48)
            .for_each(|piece| *piece = Piece::Wpawn);
        Chess {
            board,
            moves: Vec::new(),
            casting: [true; 4],
            en_passant: 64,
            is_white_turn: true,
        }
    }
    fn is_opponent_piece(&self, piece1: Piece, piece2: Piece) -> bool {
        if piece1 == Piece::Empty || piece2 == Piece::Empty {
            return false;
        }
        piece1.is_white() != piece2.is_white()
    }
    pub fn is_legal(&mut self, from: usize, to: usize) -> bool {
        let temp_board = self.board;
        self.move_piece(from, to, false);
        let check = !self.is_check();
        self.board = temp_board;
        check
    }
    pub fn is_check(&mut self) -> bool {
        let king_piece = if self.is_white_turn {
            Piece::Wking
        } else {
            Piece::Bking
        };
        let king = self
            .board
            .iter()
            .position(|&piece| piece == king_piece)
            .expect("couldnt find king position (wtf)");
        let mut moves: Vec<usize>;
        if self.is_white_turn {
            for i in 0..64 {
                if !self.board[i].is_white() {
                    moves = self.gen_moves(i);
                    for move1 in moves {
                        if move1 == king {
                            return true;
                        }
                    }
                }
            }
        } else {
            for i in 0..64 {
                if self.board[i].is_white() {
                    moves = self.gen_moves(i);
                    for move1 in moves {
                        if move1 == king {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    pub fn move_piece(&mut self, from: usize, to: usize, apply_castling: bool) {
        let piece = self.board[from];
        if apply_castling {
            self.en_passant = 64;
            match piece {
                Piece::Bking => {
                    self.casting[2] = false;
                    self.casting[3] = false;
                }
                Piece::Wking => {
                    self.casting[0] = false;
                    self.casting[1] = false;
                }
                Piece::Brook => {
                    if from == 7 {
                        self.casting[2] = false;
                    } else if from == 0 {
                        self.casting[3] = false;
                    }
                }
                Piece::Wrook => {
                    if from == 63 {
                        self.casting[0] = false;
                    } else if from == 56 {
                        self.casting[1] = false;
                    }
                }
                Piece::Wpawn => {
                    if from - to == 16 {
                        self.en_passant = to + 8;
                    }
                }
                Piece::Bpawn => {
                    if to - from == 16 {
                        self.en_passant = to - 8;
                    }
                }
                _ => (),
            }
        }
        self.board[from] = Piece::Empty;
        if let Piece::Wpawn = piece {
            if to < 8 {
                self.board[to] = Piece::Wqueen;
            } else if from - to == 7 || from - to == 9 {
                self.board[to + 8] = Piece::Empty;
                self.board[to] = piece;
            } else {
                self.board[to] = piece;
            }
        } else if let Piece::Bpawn = piece {
            if to >= 56 {
                self.board[to] = Piece::Bqueen;
            } else if to - from == 7 || to - from == 9 {
                self.board[to - 8] = Piece::Empty;
                self.board[to] = piece;
            } else {
                self.board[to] = piece;
            }
        } else {
            self.board[to] = piece;
        }
        if let Piece::Wpawn = piece {
            if to == self.en_passant {
                let captured_piece_index = to - 8; // Assuming en passant captures happen in the row above
                self.board[captured_piece_index] = Piece::Empty;
            }
        } else if let Piece::Bpawn = piece {
            if to == self.en_passant {
                let captured_piece_index = to + 8; // Assuming en passant captures happen in the row below
                self.board[captured_piece_index] = Piece::Empty;
            }
        }
        if let Piece::Bking = piece {
            if from == 4 && to == 6 {
                // Perform kingside castling for white
                self.board[7] = Piece::Empty;
                self.board[5] = Piece::Brook;
            } else if from == 4 && to == 2 {
                // Perform queenside castling for white
                self.board[0] = Piece::Empty;
                self.board[3] = Piece::Brook;
            }
        } else if let Piece::Wking = piece {
            if from == 60 && to == 62 {
                // Perform kingside castling for black
                self.board[63] = Piece::Empty;
                self.board[61] = Piece::Wrook;
            } else if from == 60 && to == 58 {
                // Perform queenside castling for black
                self.board[56] = Piece::Empty;
                self.board[59] = Piece::Wrook;
            }
        }
    }
    pub fn gen_moves(&mut self, index: usize) -> Vec<usize> {
        match self.board[index] {
            Piece::Wking | Piece::Bking => self.gen_moves_king(index),
            Piece::Wqueen | Piece::Bqueen => {
                let mut moves = self.gen_moves_rook(index);
                moves.extend(self.gen_moves_bishop(index));
                moves
            }
            Piece::Wrook | Piece::Brook => self.gen_moves_rook(index),
            Piece::Wbishop | Piece::Bbishop => self.gen_moves_bishop(index),
            Piece::Wknight | Piece::Bknight => self.gen_moves_knight(index),
            Piece::Wpawn | Piece::Bpawn => self.gen_moves_pawn(index),
            _ => Vec::new(),
        }
    }
    pub fn get_legals(&mut self, index: usize) {
        self.moves = self
            .gen_moves(index)
            .iter()
            .filter(|&&move_index| self.is_legal(index, move_index))
            .cloned()
            .collect()
    }
    fn gen_moves_king(&mut self, index: usize) -> Vec<usize> {
        let mut moves = vec![];
        let piece = self.board[index];
        // Define the possible king moves in terms of row and column offsets
        let offsets: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let row = index / 8;
        let col = index % 8;
        // Generate potential moves for each offset
        for &(row_offset, col_offset) in offsets.iter() {
            let new_row = row as i32 + row_offset;
            let new_col = col as i32 + col_offset;
            // Check if the new position is within the board boundaries
            if (0..8).contains(&new_row) && (0..8).contains(&new_col) {
                let new_index = (new_row * 8 + new_col) as usize;
                // Check if the destination is empty or occupied by an opponent's piece
                if self.is_opponent_piece(self.board[new_index], piece)
                    || self.board[new_index] == Piece::Empty
                {
                    moves.push(new_index);
                }
            }
        }
        if !piece.is_white() {
            if self.casting[2]
                && self.board[5] == Piece::Empty
                && self.board[6] == Piece::Empty
                && self.board[7] == Piece::Wrook
            {
                moves.push(6);
            }
            if self.casting[3]
                && self.board[3] == Piece::Empty
                && self.board[2] == Piece::Empty
                && self.board[1] == Piece::Empty
                && self.board[0] == Piece::Wrook
            {
                moves.push(2);
            }
        } else {
            if self.casting[0]
                && self.board[61] == Piece::Empty
                && self.board[62] == Piece::Empty
                && self.board[63] == Piece::Brook
            {
                moves.push(62);
            }
            if self.casting[1]
                && self.board[59] == Piece::Empty
                && self.board[58] == Piece::Empty
                && self.board[57] == Piece::Empty
                && self.board[56] == Piece::Brook
            {
                moves.push(58);
            }
        }
        moves
    }
    fn gen_moves_rook(&self, index: usize) -> Vec<usize> {
        let mut moves = Vec::new();
        let piece = self.board[index];
        let row = index / 8;
        let col = index % 8;
        // Check horizontally to the right
        for c in (col + 1)..8 {
            let new_index = row * 8 + c;
            let new_piece = self.board[new_index];
            if new_piece == Piece::Empty {
                moves.push(new_index);
            } else if self.is_opponent_piece(new_piece, piece) {
                moves.push(new_index);
                break;
            } else {
                break;
            }
        }
        // Check horizontally to the left
        for c in (0..col).rev() {
            let new_index = row * 8 + c;
            let new_piece = self.board[new_index];
            if new_piece == Piece::Empty {
                moves.push(new_index);
            } else if self.is_opponent_piece(new_piece, piece) {
                moves.push(new_index);
                break;
            } else {
                break;
            }
        }
        // Check vertically upwards
        for r in (0..row).rev() {
            let new_index = r * 8 + col;
            let new_piece = self.board[new_index];
            if new_piece == Piece::Empty {
                moves.push(new_index);
            } else if self.is_opponent_piece(new_piece, piece) {
                moves.push(new_index);
                break;
            } else {
                break;
            }
        }
        // Check vertically downwards
        for r in (row + 1)..8 {
            let new_index = r * 8 + col;
            let new_piece = self.board[new_index];
            if new_piece == Piece::Empty {
                moves.push(new_index);
            } else if self.is_opponent_piece(new_piece, piece) {
                moves.push(new_index);
                break;
            } else {
                break;
            }
        }
        moves
    }
    fn gen_moves_bishop(&self, index: usize) -> Vec<usize> {
        let mut moves = Vec::new();
        let piece = self.board[index];
        let row = index / 8;
        let col = index % 8;
        // Define the possible bishop moves in terms of row and column offsets
        let offsets: [(i32, i32); 4] = [
            (-1, -1), // Up-left
            (-1, 1),  // Up-right
            (1, -1),  // Down-left
            (1, 1),   // Down-right
        ];
        // Generate potential moves for each offset
        for &(row_offset, col_offset) in offsets.iter() {
            let mut new_row = row as i32 + row_offset;
            let mut new_col = col as i32 + col_offset;
            // Keep moving in the diagonal direction until out of bounds or blocked
            while (0..8).contains(&new_row) && (0..8).contains(&new_col) {
                let new_index = (new_row * 8 + new_col) as usize;
                let new_piece = self.board[new_index];
                if new_piece == Piece::Empty {
                    moves.push(new_index);
                } else if self.is_opponent_piece(new_piece, piece) {
                    moves.push(new_index);
                    break;
                } else {
                    break;
                }
                new_row += row_offset;
                new_col += col_offset;
            }
        }
        moves
    }
    fn gen_moves_knight(&mut self, index: usize) -> Vec<usize> {
        let mut moves = vec![];
        let piece = self.board[index];
        // Define the possible king moves in terms of row and column offsets
        let offsets: [(i32, i32); 8] = [
            (-2, -1),
            (-1, 2),
            (-1, -2),
            (2, -1),
            (2, 1),
            (1, -2),
            (1, 2),
            (-2, 1),
        ];
        let row = index / 8;
        let col = index % 8;
        // Generate potential moves for each offset
        for &(row_offset, col_offset) in offsets.iter() {
            let new_row = row as i32 + row_offset;
            let new_col = col as i32 + col_offset;
            // Check if the new position is within the board boundaries
            if (0..8).contains(&new_row) && (0..8).contains(&new_col) {
                let new_index = (new_row * 8 + new_col) as usize;
                // Check if the destination is empty or occupied by an opponent's piece
                if self.is_opponent_piece(self.board[new_index], piece)
                    || self.board[new_index] == Piece::Empty
                {
                    moves.push(new_index);
                }
            }
        }
        moves
    }
    fn gen_moves_pawn(&self, position: usize) -> Vec<usize> {
        let mut moves: Vec<usize> = Vec::new();
        let row = position / 8;
        let col = position % 8;
        let is_white = self.board[position].is_white();
        let direction = if is_white { -1 } else { 1 };
        // Check for forward movement
        let forward_row = (row as isize) + direction;
        if (0..8).contains(&forward_row) {
            let forward_pos = (forward_row as usize) * 8 + col;
            if self.board[forward_pos] == Piece::Empty {
                moves.push(forward_pos);
            }
        }
        if is_white
            && row == 6
            && self.board[col + 32] == Piece::Empty
            && self.board[col + 40] == Piece::Empty
        {
            moves.push(col + 32)
        }
        if !is_white
            && row == 1
            && self.board[col + 24] == Piece::Empty
            && self.board[col + 16] == Piece::Empty
        {
            moves.push(col + 24)
        }
        // Check for capturing diagonally
        if col > 0 {
            let left_dia_pos = (forward_row as usize) * 8 + (col - 1);
            if self.is_opponent_piece(self.board[left_dia_pos], self.board[position]) {
                moves.push(left_dia_pos);
            }
            if self.board[left_dia_pos] == Piece::Empty && self.en_passant == left_dia_pos {
                moves.push(left_dia_pos);
            }
        }
        if col < 7 {
            let right_dia_pos = (forward_row as usize) * 8 + (col + 1);
            if self.is_opponent_piece(self.board[right_dia_pos], self.board[position]) {
                moves.push(right_dia_pos);
            }
            if self.board[right_dia_pos] == Piece::Empty && self.en_passant == right_dia_pos {
                moves.push(right_dia_pos);
            }
        }
        moves
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Empty,
    Bpawn,
    Wpawn,
    Bknight,
    Wknight,
    Bbishop,
    Wbishop,
    Brook,
    Wrook,
    Bqueen,
    Wqueen,
    Bking,
    Wking,
}

impl Piece {
    pub fn is_white(&self) -> bool {
        matches!(
            self,
            Piece::Wpawn
                | Piece::Wknight
                | Piece::Wbishop
                | Piece::Wrook
                | Piece::Wqueen
                | Piece::Wking
        )
    }
}
