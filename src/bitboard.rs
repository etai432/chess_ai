use crate::chess::Piece;
#[derive(Debug, Clone, Copy)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn empty() -> Self {
        Bitboard(0)
    }

    pub fn set_bit(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    // pub fn clear_bit(&mut self, index: usize) {
    //     self.0 &= !(1 << index);
    // }

    pub fn get_bit(&self, index: u8) -> bool {
        (self.0 & (1 << index)) != 0
    }

    // pub fn get_pieces(&self) -> Vec<usize> {
    //     let mut pieces = Vec::new();
    //     let mut bitboard = self.0;
    //     while bitboard != 0 {
    //         let index = bitboard.trailing_zeros() as usize;
    //         pieces.push(index);
    //         bitboard &= bitboard - 1;
    //     }
    //     pieces
    // }
    pub fn switch_on_indices(&mut self, indices: &[u8]) {
        for &index in indices {
            self.0 |= 1 << index;
        }
    }
    pub fn switch_on_index(&mut self, index: u8) {
        self.0 |= 1 << index;
    }
    pub fn print_bit_representation(&self) {
        for row in 0..8 {
            for col in 0..8 {
                let index = row * 8 + col;
                let bit = (self.0 >> index) & 1;
                let square = if bit == 1 { "X" } else { "." };
                print!("{} ", square);
            }
            println!();
        }
        println!("-------------");
    }
    // pub fn get_piece(&self) -> usize {
    //     if self.0 == 0 {
    //         panic!("Bitboard is empty.");
    //     }
    //     self.0.trailing_zeros() as usize
    // }

    // pub fn from_piece_type(board: &[Piece; 64], piece: Piece) -> Self {
    //     let mut bitboard = Bitboard::empty();
    //     for (i, &p) in board.iter().enumerate() {
    //         if p == piece {
    //             bitboard.set_bit(i);
    //         }
    //     }
    //     bitboard
    // }

    // pub fn from_pieces(board: &[Piece; 64], is_white: bool) -> Self {
    //     let mut bitboard = Bitboard::empty();
    //     for (i, &p) in board.iter().enumerate() {
    //         if p.is_white() == is_white {
    //             bitboard.set_bit(i);
    //         }
    //     }
    //     bitboard
    // }

    pub fn from_index(index: u8) -> Bitboard {
        if (0..64).contains(&index) {
            let mask = 1u64 << index;
            return Bitboard(mask);
        }
        Bitboard(0)
    }

    pub fn ray(from: u8, to: u8) -> Bitboard {
        let from_row = from / 8;
        let from_col = from % 8;
        let to_row = to / 8;
        let to_col = to % 8;

        // Calculate the row and column differences
        let row_diff = (to_row as i32 - from_row as i32).signum();
        let col_diff = (to_col as i32 - from_col as i32).signum();

        let mut current_square = from;
        let mut ray_bitboard = Bitboard::empty();

        // Generate the ray by moving along the row and column differences
        while current_square != to {
            current_square = ((current_square as i32) + row_diff * 8 + col_diff) as u8;
            if !(0..63).contains(&current_square) {
                break;
            }
            ray_bitboard.set_bit(current_square);
        }
        ray_bitboard
    }
    // pub fn from_piece(piece: Piece, board: &[Piece; 64]) -> Bitboard {
    //     let mask = board
    //         .iter()
    //         .enumerate()
    //         .filter(|(_, &p)| p == piece)
    //         .fold(0u64, |acc, (i, _)| acc | (1u64 << i));
    //     Bitboard(mask)
    // }
}
