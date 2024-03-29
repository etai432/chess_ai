use crate::bitboard::Bitboard;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Chess {
    pub board: [Piece; 64],
    pub moves: Vec<u8>,
    pub castling: [bool; 4], //white, white long, black, black long
    pub en_passant: u8,
    pub is_white_turn: bool,
    pub white_king: u8,
    pub black_king: u8,
    pub white_attack: Bitboard,
    pub black_attack: Bitboard,
    pub white_pins: Bitboard,
    pub black_pins: Bitboard,
    last_attack: Bitboard,
    last_pin: Bitboard,
    checking_pieces: Vec<u8>,
    pub knight_moves: [[u8; 8]; 64],
    pawn_moves: [[u8; 4]; 128],
    king_moves: [[u8; 8]; 64],
    pub last_move: (u8, u8),
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
        let white_attack = Bitboard::empty();
        let black_attack = Bitboard::empty();
        Chess {
            board,
            moves: Vec::new(),
            castling: [true; 4],
            en_passant: 64,
            is_white_turn: true,
            white_king: 60,
            black_king: 4,
            white_attack,
            black_attack,
            white_pins: Bitboard::empty(),
            black_pins: Bitboard::empty(),
            last_attack: Bitboard::empty(),
            last_pin: Bitboard::empty(),
            checking_pieces: Vec::new(),
            knight_moves: Chess::precompute_knight(),
            pawn_moves: Chess::precompute_pawn(),
            king_moves: Chess::precompute_king(),
            last_move: (64, 64),
        }
    }
    fn is_opponent_piece(&self, piece1: Piece, piece2: Piece) -> bool {
        if piece1 == Piece::Empty || piece2 == Piece::Empty {
            return false;
        }
        piece1.is_white() != piece2.is_white()
    }
    pub fn is_legal(&self, from: u8, to: u8) -> bool {
        if self.is_check(self.king_loc()) {
            if self.board[from as usize] == Piece::Bking
                || self.board[from as usize] == Piece::Wking
            {
                //moving the king
                !self.is_check(to)
            } else if self.checking_pieces.len() == 1 {
                //blocking is only possible when a single check happens.
                //check if blockable
                if matches!(
                    self.board[self.checking_pieces[0] as usize],
                    Piece::Bbishop
                        | Piece::Wbishop
                        | Piece::Brook
                        | Piece::Wrook
                        | Piece::Bqueen
                        | Piece::Wqueen
                ) {
                    let blocking_ray = Bitboard::ray(self.king_loc(), self.checking_pieces[0]);
                    let from_to_ray = Bitboard::ray(from, to);
                    from_to_ray.0 & blocking_ray.0 != 0 && blocking_ray.get_bit(to)
                } else {
                    to == self.checking_pieces[0]
                }
            } else {
                false // double check- stays false
            }
        } else {
            //king is safe, just dont break pins
            if self.is_white_turn {
                if self.board[from as usize] == Piece::Wking {
                    //piece is king
                    !self.is_check(to)
                } else if self.black_pins.get_bit(from) {
                    //piece is pinned
                    self.black_pins.get_bit(to) //move doesnt break pin
                } else {
                    true // piece isnt pinned
                }
            } else if self.board[from as usize] == Piece::Bking {
                !self.is_check(to)
            } else if self.white_pins.get_bit(from) {
                self.white_pins.get_bit(to)
            } else {
                true
            }
        }
    }
    pub fn king_loc(&self) -> u8 {
        if self.is_white_turn {
            self.white_king
        } else {
            self.black_king
        }
    }
    pub fn is_check(&self, king_position: u8) -> bool {
        if self.is_white_turn {
            self.black_attack
        } else {
            self.white_attack
        }
        .get_bit(king_position)
    }
    pub fn move_piece(&mut self, from: u8, to: u8) -> ChessMove {
        let mut rook_cancel: u8 = 4;
        let mut castle_flag = false;
        let mut en_passant_flag = false;
        let king_loc = if self.is_white_turn {
            self.white_king
        } else {
            self.black_king
        };
        let mut captured_piece = self.board[to as usize];
        // self.last_attack = if self.is_white_turn {
        //     self.white_attack
        // } else {
        //     self.black_attack
        // };
        let piece = self.board[from as usize];
        if let Piece::Wpawn = piece {
            if to == self.en_passant {
                en_passant_flag = true;
                let captured_piece_index = to + 8;
                captured_piece = Piece::Bpawn;
                self.board[captured_piece_index as usize] = Piece::Empty;
            }
        } else if let Piece::Bpawn = piece {
            if to == self.en_passant {
                en_passant_flag = true;
                let captured_piece_index = to - 8;
                captured_piece = Piece::Wpawn;
                self.board[captured_piece_index as usize] = Piece::Empty;
            }
        }
        self.en_passant = 64;
        match piece {
            Piece::Bking => {
                self.castling[2] = false;
                self.castling[3] = false;
                self.black_king = to; //maybe need to save last_king in undo_move
            }
            Piece::Wking => {
                self.castling[0] = false;
                self.castling[1] = false;
                self.white_king = to;
            }
            Piece::Brook => {
                if from == 7 {
                    self.castling[2] = false;
                    rook_cancel = 2;
                } else if from == 0 {
                    self.castling[3] = false;
                    rook_cancel = 3;
                }
            }
            Piece::Wrook => {
                if from == 63 {
                    self.castling[0] = false;
                    rook_cancel = 0;
                } else if from == 56 {
                    self.castling[1] = false;
                    rook_cancel = 1;
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

        // Update the board
        self.board[from as usize] = Piece::Empty;

        if piece == Piece::Wpawn && to < 8 {
            self.board[to as usize] = Piece::Wqueen;
        } else if piece == Piece::Bpawn && to >= 56 {
            self.board[to as usize] = Piece::Bqueen;
        } else {
            self.board[to as usize] = piece;
        }
        if let Piece::Bking = piece {
            if from == 4 && to == 6 {
                // Perform kingside castling for white
                self.board[7] = Piece::Empty;
                self.board[5] = Piece::Brook;
                castle_flag = true;
            } else if from == 4 && to == 2 {
                // Perform queenside castling for white
                self.board[0] = Piece::Empty;
                self.board[3] = Piece::Brook;
                castle_flag = true;
            }
        } else if let Piece::Wking = piece {
            if from == 60 && to == 62 {
                // Perform kingside castling for black
                self.board[63] = Piece::Empty;
                self.board[61] = Piece::Wrook;
                castle_flag = true;
            } else if from == 60 && to == 58 {
                // Perform queenside castling for black
                self.board[56] = Piece::Empty;
                self.board[59] = Piece::Wrook;
                castle_flag = true;
            }
        }
        //updating the attacked squares also updates the pins
        if self.is_white_turn {
            self.white_pins = Bitboard::empty();
        } else {
            self.black_pins = Bitboard::empty();
        }
        //update bitboard of the moved piece side attack (if white moves, update white. that way black will already be updated from last move)
        self.update_attacked_squares();
        self.is_white_turn = !self.is_white_turn;
        self.last_move = (from, to);
        ChessMove {
            from,
            to,
            castle_flag,
            en_passant_flag,
            captured_piece,
            last_king_placement: king_loc,
            rook_cancel,
        }
    }
    pub fn undo_move(&mut self, chess_move: ChessMove) {
        self.board[chess_move.from as usize] = self.board[chess_move.to as usize];
        self.board[chess_move.to as usize] = chess_move.captured_piece;
        if chess_move.rook_cancel != 4 {
            self.castling[chess_move.rook_cancel as usize] = true;
        }
        self.en_passant = 64;
        self.is_white_turn = !self.is_white_turn;
        if chess_move.castle_flag {
            if self.king_loc() == 62 {
                self.castling[0] = true;
                self.board[61] = Piece::Empty;
                self.board[63] = Piece::Wrook;
            } else if self.king_loc() == 58 {
                self.castling[1] = true;
                self.board[59] = Piece::Empty;
                self.board[56] = Piece::Wrook;
            } else if self.king_loc() == 6 {
                self.castling[2] = true;
                self.board[5] = Piece::Empty;
                self.board[7] = Piece::Brook;
            } else if self.king_loc() == 2 {
                self.castling[3] = true;
                self.board[3] = Piece::Empty;
                self.board[0] = Piece::Brook;
            } else {
                panic!(
                    "failed to revert castling {} {}",
                    self.king_loc(),
                    self.is_white_turn
                )
            }
        } else if chess_move.en_passant_flag {
            self.board[chess_move.to as usize] = Piece::Empty;
            if self.is_white_turn {
                self.en_passant = chess_move.to;
                self.board[chess_move.to as usize + 8] = Piece::Bpawn;
            } else {
                self.en_passant = chess_move.to;
                self.board[chess_move.to as usize - 8] = Piece::Wpawn;
            }
        }
        if self.is_white_turn {
            self.white_king = chess_move.last_king_placement;
        } else {
            self.black_king = chess_move.last_king_placement;
        }
    }
    pub fn update_attacked_squares(&mut self) {
        self.checking_pieces = Vec::new();
        let mut attacked_squares = Bitboard::empty();
        for (i, piece) in self.board.into_iter().enumerate() {
            if piece.is_white() == self.is_white_turn && piece != Piece::Empty {
                // we want all moves, even if they cant actually move there
                attacked_squares.switch_on_indices(&self.generate_attacks(i));
            }
        }
        if self.is_white_turn {
            self.white_attack = attacked_squares;
        } else {
            self.black_attack = attacked_squares;
        }
    }
    pub fn gen_moves(&mut self, index: usize) -> Vec<u8> {
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
            .filter(|&&move_index| self.is_legal(index as u8, move_index))
            .cloned()
            .collect()
    }
    pub fn gen_castling_king(&self, index: usize) -> Vec<u8> {
        let mut moves = vec![];
        let piece = self.board[index];
        if !piece.is_white() {
            if self.castling[2]
                && self.board[5] == Piece::Empty
                && self.board[6] == Piece::Empty
                && self.board[7] == Piece::Brook
                && !self.is_check(4)
                && !self.is_check(5)
                && !self.is_check(6)
            {
                moves.push(6);
            }
            if self.castling[3]
                && self.board[3] == Piece::Empty
                && self.board[2] == Piece::Empty
                && self.board[1] == Piece::Empty
                && self.board[0] == Piece::Brook
                && !self.is_check(4)
                && !self.is_check(3)
                && !self.is_check(2)
            {
                moves.push(2);
            }
        } else {
            if self.castling[0]
                && self.board[61] == Piece::Empty
                && self.board[62] == Piece::Empty
                && self.board[63] == Piece::Wrook
                && !self.is_check(60)
                && !self.is_check(61)
                && !self.is_check(62)
            {
                moves.push(62);
            }
            if self.castling[1]
                && self.board[59] == Piece::Empty
                && self.board[58] == Piece::Empty
                && self.board[57] == Piece::Empty
                && self.board[56] == Piece::Wrook
                && !self.is_check(60)
                && !self.is_check(59)
                && !self.is_check(58)
            {
                moves.push(58);
            }
        }
        moves
    }
    pub fn gen_moves_rook(&self, index: usize) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();
        let piece = self.board[index];
        let row = index as u8 / 8;
        let col = index as u8 % 8;
        // Check horizontally to the right
        for c in (col + 1)..8 {
            let new_index = row * 8 + c;
            let new_piece = self.board[new_index as usize];
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
            let new_piece = self.board[new_index as usize];
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
            let new_piece = self.board[new_index as usize];
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
            let new_piece = self.board[new_index as usize];
            if self.board[new_index as usize] == Piece::Empty {
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
    pub fn gen_moves_bishop(&self, index: usize) -> Vec<u8> {
        let mut moves: Vec<u8> = Vec::new();
        let piece = self.board[index];
        let row = index as u8 / 8;
        let col = index as u8 % 8;
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
                let new_index = (new_row * 8 + new_col) as u8;
                let new_piece = self.board[new_index as usize];
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
    pub fn generate_attacks(&mut self, index: usize) -> Vec<u8> {
        match self.board[index] {
            Piece::Wking | Piece::Bking => self.gen_attacks_king(index),
            Piece::Wqueen | Piece::Bqueen => {
                let mut moves = self.gen_attacks_rook(index as u8);
                moves.extend(self.gen_attacks_bishop(index as u8));
                moves
            }
            Piece::Wrook | Piece::Brook => self.gen_attacks_rook(index as u8),
            Piece::Wbishop | Piece::Bbishop => self.gen_attacks_bishop(index as u8),
            Piece::Wknight | Piece::Bknight => self.gen_attacks_knight(index),
            Piece::Wpawn | Piece::Bpawn => self.gen_attacks_pawn(index),
            _ => Vec::new(),
        }
    }
    pub fn gen_attacks_rook(&mut self, index: u8) -> Vec<u8> {
        let row = index / 8;
        let col = index % 8;
        let mut pin_squares = Bitboard::empty();
        pin_squares.set_bit(index);
        let mut moves: Vec<u8> = Vec::new();
        // Check horizontally to the right
        let mut piece_count = 0;
        let mut met_king = false;
        for c in (col + 1)..8 {
            let new_index = row * 8 + c;
            let new_piece = self.board[new_index as usize];
            if new_piece == Piece::Empty && piece_count == 0 {
                moves.push(new_index);
                pin_squares.switch_on_index(new_index);
            } else if new_piece.is_white() != self.is_white_turn {
                // Enemy piece
                if new_piece == Piece::Bking || new_piece == Piece::Wking {
                    if piece_count == 0 {
                        moves.push(new_index);
                        if new_index < 63 {
                            moves.push(new_index + 1);
                        }
                        self.checking_pieces.push(index);
                    }
                    pin_squares.switch_on_index(new_index);
                    met_king = true;
                    break; // No need to consider further moves in this direction
                }
                piece_count += 1;
                pin_squares.switch_on_index(new_index);
                if piece_count == 1 {
                    moves.push(new_index);
                }
            } else {
                // Friendly piece
                if piece_count == 0 {
                    moves.push(new_index);
                }
                break; // Stop considering further moves in this direction
            }
        }
        if piece_count == 1 && met_king {
            //only valid when an enemy piece is pinned to the king
            if self.is_white_turn {
                self.white_pins.0 |= pin_squares.0;
            } else {
                self.black_pins.0 |= pin_squares.0;
            }
        }

        // Check horizontally to the left
        pin_squares = Bitboard::empty();
        pin_squares.set_bit(index);
        piece_count = 0;
        met_king = false;
        for c in (0..col).rev() {
            let new_index = row * 8 + c;
            let new_piece = self.board[new_index as usize];
            if new_piece == Piece::Empty && piece_count == 0 {
                moves.push(new_index);
                pin_squares.switch_on_index(new_index);
            } else if new_piece.is_white() != self.is_white_turn {
                // Enemy piece
                if new_piece == Piece::Bking || new_piece == Piece::Wking {
                    if piece_count == 0 {
                        moves.push(new_index);
                        if new_index >= 1 {
                            moves.push(new_index - 1);
                        }
                        self.checking_pieces.push(index);
                    }
                    pin_squares.switch_on_index(new_index);
                    met_king = true;
                    break; // No need to consider further moves in this direction
                }
                piece_count += 1;
                pin_squares.switch_on_index(new_index);
                if piece_count == 1 {
                    moves.push(new_index);
                }
            } else {
                // Friendly piece
                if piece_count == 0 {
                    moves.push(new_index);
                }
                break; // Stop considering further moves in this direction
            }
        }
        if piece_count == 1 && met_king {
            //only valid when an enemy piece is pinned to the king
            if self.is_white_turn {
                self.white_pins.0 |= pin_squares.0;
            } else {
                self.black_pins.0 |= pin_squares.0;
            }
        }

        // Check vertically upwards
        pin_squares = Bitboard::empty();
        pin_squares.set_bit(index);
        piece_count = 0;
        met_king = false;
        for r in (0..row).rev() {
            let new_index = r * 8 + col;
            let new_piece = self.board[new_index as usize];
            if new_piece == Piece::Empty && piece_count == 0 {
                moves.push(new_index);
                pin_squares.switch_on_index(new_index);
            } else if new_piece.is_white() != self.is_white_turn {
                // Enemy piece
                if new_piece == Piece::Bking || new_piece == Piece::Wking {
                    if piece_count == 0 {
                        moves.push(new_index);
                        if new_index >= 8 {
                            moves.push(new_index - 8);
                        }
                        self.checking_pieces.push(index);
                    }
                    pin_squares.switch_on_index(new_index);
                    met_king = true;
                    break; // No need to consider further moves in this direction
                }
                piece_count += 1;
                pin_squares.switch_on_index(new_index);
                if piece_count == 1 {
                    moves.push(new_index);
                }
            } else {
                // Friendly piece
                if piece_count == 0 {
                    moves.push(new_index);
                }
                break; // Stop considering further moves in this direction
            }
        }
        if piece_count == 1 && met_king {
            //only valid when an enemy piece is pinned to the king
            if self.is_white_turn {
                self.white_pins.0 |= pin_squares.0;
            } else {
                self.black_pins.0 |= pin_squares.0;
            }
        }

        // Check vertically downwards
        pin_squares = Bitboard::empty();
        pin_squares.set_bit(index);
        piece_count = 0;
        met_king = false;
        for r in (row + 1)..8 {
            let new_index = r * 8 + col;
            let new_piece = self.board[new_index as usize];
            if new_piece == Piece::Empty && piece_count == 0 {
                moves.push(new_index);
                pin_squares.switch_on_index(new_index);
            } else if new_piece.is_white() != self.is_white_turn {
                // Enemy piece
                if new_piece == Piece::Bking || new_piece == Piece::Wking {
                    if piece_count == 0 {
                        moves.push(new_index);
                        if new_index < 56 {
                            moves.push(new_index + 8);
                        }
                        self.checking_pieces.push(index);
                    }
                    pin_squares.switch_on_index(new_index);
                    met_king = true;
                    break; // No need to consider further moves in this direction
                }
                piece_count += 1;
                pin_squares.switch_on_index(new_index);
                if piece_count == 1 {
                    moves.push(new_index);
                }
            } else {
                // Friendly piece
                if piece_count == 0 {
                    moves.push(new_index);
                }
                break; // Stop considering further moves in this direction
            }
        }
        if piece_count == 1 && met_king {
            //only valid when an enemy piece is pinned to the king
            if self.is_white_turn {
                self.white_pins.0 |= pin_squares.0;
            } else {
                self.black_pins.0 |= pin_squares.0;
            }
        }
        moves
    }
    pub fn gen_attacks_bishop(&mut self, index: u8) -> Vec<u8> {
        let row = index / 8;
        let col = index % 8;
        let mut pin_squares = Bitboard::empty();
        pin_squares.set_bit(index);
        let mut moves: Vec<u8> = Vec::new();

        // Check diagonally to the top-right
        let mut piece_count = 0;
        let mut met_king = false;
        for i in 1..8 {
            let r = row as i32 + i;
            let c = col as i32 + i;
            if (0..8).contains(&r) && (0..8).contains(&c) {
                let new_index = (r * 8 + c) as usize;
                let new_piece = self.board[new_index];
                if new_piece == Piece::Empty && piece_count == 0 {
                    moves.push(new_index as u8);
                    pin_squares.switch_on_index(new_index as u8);
                } else if new_piece.is_white() != self.is_white_turn {
                    // Enemy piece
                    if new_piece == Piece::Bking || new_piece == Piece::Wking {
                        if piece_count == 0 {
                            moves.push(new_index as u8);
                            if new_index < 55 {
                                moves.push(new_index as u8 + 9);
                            }
                            self.checking_pieces.push(index);
                        }
                        pin_squares.switch_on_index(new_index as u8);
                        met_king = true;
                        break; // No need to consider further moves in this direction
                    }
                    piece_count += 1;
                    pin_squares.switch_on_index(new_index as u8);
                    if piece_count == 1 {
                        moves.push(new_index as u8);
                    }
                } else {
                    // Friendly piece
                    if piece_count == 0 {
                        moves.push(new_index as u8);
                    }
                    break; // Stop considering further moves in this direction
                }
            } else {
                break; // Out of board boundaries
            }
        }
        if piece_count == 1 && met_king {
            //only valid when an enemy piece is pinned to the king
            if self.is_white_turn {
                self.white_pins.0 |= pin_squares.0;
            } else {
                self.black_pins.0 |= pin_squares.0;
            }
        }

        // Check diagonally to the top-left
        pin_squares = Bitboard::empty();
        pin_squares.set_bit(index);
        piece_count = 0;
        met_king = false;
        for i in 1..8 {
            let r = row as i32 + i;
            let c = col as i32 - i;
            if (0..8).contains(&r) && (0..8).contains(&c) {
                let new_index = (r * 8 + c) as usize;
                let new_piece = self.board[new_index];
                if new_piece == Piece::Empty && piece_count == 0 {
                    moves.push(new_index as u8);
                    pin_squares.switch_on_index(new_index as u8);
                } else if new_piece.is_white() != self.is_white_turn {
                    // Enemy piece
                    if new_piece == Piece::Bking || new_piece == Piece::Wking {
                        if piece_count == 0 {
                            if new_index < 57 {
                                moves.push(new_index as u8 + 7);
                            }
                            moves.push(new_index as u8);
                            self.checking_pieces.push(index);
                        }
                        pin_squares.switch_on_index(new_index as u8);
                        met_king = true;
                        break; // No need to consider further moves in this direction
                    }
                    piece_count += 1;
                    pin_squares.switch_on_index(new_index as u8);
                    if piece_count == 1 {
                        moves.push(new_index as u8);
                    }
                } else {
                    // Friendly piece
                    if piece_count == 0 {
                        moves.push(new_index as u8);
                    }
                    break; // Stop considering further moves in this direction
                }
            } else {
                break; // Out of board boundaries
            }
        }
        if piece_count == 1 && met_king {
            //only valid when an enemy piece is pinned to the king
            if self.is_white_turn {
                self.white_pins.0 |= pin_squares.0;
            } else {
                self.black_pins.0 |= pin_squares.0;
            }
        }

        // Check diagonally to the bottom-right
        pin_squares = Bitboard::empty();
        pin_squares.set_bit(index);
        piece_count = 0;
        met_king = false;
        for i in 1..8 {
            let r = row as i32 - i;
            let c = col as i32 + i;
            if (0..8).contains(&r) && (0..8).contains(&c) {
                let new_index = (r * 8 + c) as usize;
                let new_piece = self.board[new_index];
                if new_piece == Piece::Empty && piece_count == 0 {
                    moves.push(new_index as u8);
                    pin_squares.switch_on_index(new_index as u8);
                } else if new_piece.is_white() != self.is_white_turn {
                    // Enemy piece
                    if new_piece == Piece::Bking || new_piece == Piece::Wking {
                        if piece_count == 0 {
                            moves.push(new_index as u8);
                            if new_index >= 7 {
                                moves.push(new_index as u8 - 7);
                            }
                            self.checking_pieces.push(index);
                        }
                        pin_squares.switch_on_index(new_index as u8);
                        met_king = true;
                        break; // No need to consider further moves in this direction
                    }
                    piece_count += 1;
                    pin_squares.switch_on_index(new_index as u8);
                    if piece_count == 1 {
                        moves.push(new_index as u8);
                    }
                } else {
                    // Friendly piece
                    if piece_count == 0 {
                        moves.push(new_index as u8);
                    }
                    break; // Stop considering further moves in this direction
                }
            } else {
                break; // Out of board boundaries
            }
        }
        if piece_count == 1 && met_king {
            //only valid when an enemy piece is pinned to the king
            if self.is_white_turn {
                self.white_pins.0 |= pin_squares.0;
            } else {
                self.black_pins.0 |= pin_squares.0;
            }
        }

        // Check diagonally to the bottom-left
        pin_squares = Bitboard::empty();
        pin_squares.set_bit(index);
        piece_count = 0;
        met_king = false;
        for i in 1..8 {
            let r = row as i32 - i;
            let c = col as i32 - i;
            if (0..8).contains(&r) && (0..8).contains(&c) {
                let new_index = (r * 8 + c) as usize;
                let new_piece = self.board[new_index];
                if new_piece == Piece::Empty && piece_count == 0 {
                    moves.push(new_index as u8);
                    pin_squares.switch_on_index(new_index as u8);
                } else if new_piece.is_white() != self.is_white_turn {
                    // Enemy piece
                    if new_piece == Piece::Bking || new_piece == Piece::Wking {
                        if piece_count == 0 {
                            moves.push(new_index as u8);
                            if new_index >= 9 {
                                moves.push(new_index as u8 - 9);
                            }
                            self.checking_pieces.push(index);
                        }
                        pin_squares.switch_on_index(new_index as u8);
                        met_king = true;
                        break; // No need to consider further moves in this direction
                    }
                    piece_count += 1;
                    pin_squares.switch_on_index(new_index as u8);
                    if piece_count == 1 {
                        moves.push(new_index as u8);
                    }
                } else {
                    // Friendly piece
                    if piece_count == 0 {
                        moves.push(new_index as u8);
                    }
                    break; // Stop considering further moves in this direction
                }
            } else {
                break; // Out of board boundaries
            }
        }
        if piece_count == 1 && met_king {
            //only valid when an enemy piece is pinned to the king
            if self.is_white_turn {
                self.white_pins.0 |= pin_squares.0;
            } else {
                self.black_pins.0 |= pin_squares.0;
            }
        }

        // Convert the Bitboard to a Vec<usize> for output
        moves
    }
    pub fn is_insufficient_material(&self) -> bool {
        // Implement your logic to check for insufficient material here
        // Return true if the game is in a draw due to insufficient material
        false
    }

    // Check for threefold repetition
    pub fn is_threefold_repetition(&self) -> bool {
        // Implement your logic to check for threefold repetition here
        // Return true if the game is in a draw due to threefold repetition
        false
    }
    pub fn get_all_moves(&mut self) -> Vec<(u8, u8)> {
        let mut moves: Vec<(u8, u8)> = Vec::new();
        for i in 0..64 {
            if self.board[i].is_white() == self.is_white_turn {
                self.get_legals(i);
                moves.extend(
                    std::mem::take(&mut self.moves)
                        .into_iter()
                        .map(|x| (i as u8, x)),
                );
            }
        }
        moves
    }
    pub fn precompute_knight() -> [[u8; 8]; 64] {
        let mut precomputed_moves: [[u8; 8]; 64] = [[64; 8]; 64];
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
        for (square, val) in precomputed_moves.iter_mut().enumerate() {
            let row: usize = square / 8;
            let col: usize = square % 8;
            // Generate potential moves for each offset
            for (index, &(row_offset, col_offset)) in offsets.iter().enumerate() {
                let new_row = row as i32 + row_offset;
                let new_col = col as i32 + col_offset;
                // Check if the new position is within the board boundaries
                if (0..8).contains(&new_row) && (0..8).contains(&new_col) {
                    val[index] = (new_row * 8 + new_col) as u8;
                }
            }
        }
        precomputed_moves
    }
    pub fn gen_moves_knight(&self, index: usize) -> Vec<u8> {
        self.knight_moves[index]
            .into_iter()
            .filter(|&i| i != 64 && self.board[i as usize].is_opponent_or_empty(self.board[index]))
            .collect()
    }
    pub fn gen_attacks_knight(&mut self, index: usize) -> Vec<u8> {
        self.knight_moves[index]
            .into_iter()
            .filter(|&i| i != 64)
            .map(|i| {
                if (self.board[index].is_white() && self.board[i as usize] == Piece::Bking)
                    || (!self.board[index].is_white() && self.board[i as usize] == Piece::Wking)
                {
                    self.checking_pieces.push(index as u8)
                }
                i
            })
            .collect()
    }
    pub fn precompute_pawn() -> [[u8; 4]; 128] {
        //twice forward, once forward, left capture, right capture
        let mut precomputed_moves: [[u8; 4]; 128] = [[64; 4]; 128];
        // White Pawns
        for (square, moves) in precomputed_moves.iter_mut().enumerate().skip(8).take(64) {
            // White Pawns
            let row: u8 = square as u8 / 8;
            let col: u8 = square as u8 % 8;
            // Two squares forward (if on starting row)
            if row == 6 {
                moves[0] = square as u8 - 16;
            }
            // One square forward
            moves[1] = square as u8 - 8;
            // Diagonal captures
            if col > 0 {
                moves[2] = square as u8 - 9; // Diagonal capture to the left
            }
            if col < 7 {
                moves[3] = square as u8 - 7; // Diagonal capture to the right
            }
        }
        // Black Pawns
        for (mut square, moves) in precomputed_moves.iter_mut().enumerate().skip(64) {
            square -= 64;
            let row: usize = square / 8;
            let col: usize = square % 8;

            // Two squares forward (if on starting row)
            if row == 1 {
                moves[0] = square as u8 + 16;
            }

            // One square forward
            moves[1] = square as u8 + 8;

            // Diagonal captures
            if col > 0 {
                moves[2] = square as u8 + 7; // Diagonal capture to the left
            }
            if col < 7 {
                moves[3] = square as u8 + 9; // Diagonal capture to the right
            }
        }
        precomputed_moves
    }
    pub fn gen_moves_pawn(&self, index: usize) -> Vec<u8> {
        let mut moves = Vec::new();
        let color_offset = if self.board[index].is_white() { 0 } else { 64 };
        let pawn_moves = &self.pawn_moves[index + color_offset];

        // Check the first move (single square forward)
        if pawn_moves[1] != 64 && self.board[pawn_moves[1] as usize] == Piece::Empty {
            moves.push(pawn_moves[1]);
            // Check the second move (double square forward from starting position)
            if pawn_moves[0] != 64 && self.board[pawn_moves[0] as usize] == Piece::Empty {
                moves.push(pawn_moves[0]);
            }
        }
        // Check the two diagonal capture moves
        if pawn_moves[2] != 64
            && (self.board[pawn_moves[2] as usize].is_opponent(self.board[index])
                || self.en_passant == pawn_moves[2])
        {
            moves.push(pawn_moves[2]);
        }
        if pawn_moves[3] != 64
            && (self.board[pawn_moves[3] as usize].is_opponent(self.board[index])
                || self.en_passant == pawn_moves[3])
        {
            moves.push(pawn_moves[3]);
        }
        moves
    }
    pub fn gen_attacks_pawn(&mut self, index: usize) -> Vec<u8> {
        let pawn_moves =
            &self.pawn_moves[index + if self.board[index].is_white() { 0 } else { 64 }];
        let mut attacking_moves = Vec::with_capacity(2);
        for &move_index in pawn_moves.iter().skip(2) {
            if move_index < 64 {
                attacking_moves.push(move_index);
                if (self.board[index].is_white() && self.board[move_index as usize] == Piece::Bking)
                    || (!self.board[index].is_white()
                        && self.board[move_index as usize] == Piece::Wking)
                {
                    self.checking_pieces.push(index as u8);
                }
            }
        }
        attacking_moves
    } // might be broken
    fn precompute_king() -> [[u8; 8]; 64] {
        let mut precomputed_moves: [[u8; 8]; 64] = [[64; 8]; 64];
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
        for (square, moves) in precomputed_moves.iter_mut().enumerate() {
            let row = square as u8 / 8;
            let col = square as u8 % 8;
            for (index, &(row_offset, col_offset)) in offsets.iter().enumerate() {
                let new_row = row as i32 + row_offset;
                let new_col = col as i32 + col_offset;
                // Check if the new position is within the board boundaries
                if (0..8).contains(&new_row) && (0..8).contains(&new_col) {
                    moves[index] = (new_row * 8 + new_col) as u8;
                }
            }
        }
        precomputed_moves
    }
    pub fn gen_moves_king(&self, index: usize) -> Vec<u8> {
        let mut moves = self.gen_castling_king(index);
        moves.extend(self.king_moves[index].into_iter().filter(|&i| {
            i != 64 && self.board[i as usize].is_opponent_or_empty(self.board[index])
        }));
        moves
    }
    pub fn gen_attacks_king(&mut self, index: usize) -> Vec<u8> {
        self.king_moves[index]
            .into_iter()
            .filter(|&i| i != 64)
            .collect()
    }
    pub fn is_ending(&mut self) -> i32 {
        if self.get_all_moves().is_empty() {
            if self.is_check(self.king_loc()) {
                if self.is_white_turn {
                    -1
                } else {
                    1
                }
            } else {
                0
            }
        } else if self.is_insufficient_material() {
            3
        } else if self.is_threefold_repetition() {
            4
        } else {
            2
        }
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
    pub fn evaluate(&self) -> f32 {
        match self {
            Piece::Wpawn => 100.0,
            Piece::Bpawn => -100.0,
            Piece::Wknight => 300.0,
            Piece::Bknight => -300.0,
            Piece::Wbishop => 320.0,
            Piece::Bbishop => -320.0,
            Piece::Wrook => 500.0,
            Piece::Brook => -500.0,
            Piece::Wqueen => 900.0,
            Piece::Bqueen => -900.0,
            _ => 0.0,
        }
    }
    pub fn is_opponent_or_empty(&self, opp: Piece) -> bool {
        if self == &Piece::Empty {
            return true;
        }
        match opp {
            Piece::Empty => true, // An empty square is considered as "opponent or empty"
            piece => piece.is_white() != self.is_white(), // Check if it's an opponent's piece
        }
    }
    pub fn is_opponent(&self, other: Piece) -> bool {
        if self == &Piece::Empty || other == Piece::Empty {
            return false;
        }
        self.is_white() != other.is_white()
    }
}

pub struct ChessMove {
    pub from: u8,
    pub to: u8,
    pub castle_flag: bool,
    pub en_passant_flag: bool,
    pub captured_piece: Piece, // Stores the captured piece, if any
    pub last_king_placement: u8,
    pub rook_cancel: u8, // 0/1/2/3/4
}
