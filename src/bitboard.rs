use crate::chess::Piece;
#[derive(Debug, Clone, Copy)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn empty() -> Self {
        Bitboard(0)
    }

    // pub fn set_bit(&mut self, index: usize) {
    //     self.0 |= 1 << index;
    // }

    // pub fn clear_bit(&mut self, index: usize) {
    //     self.0 &= !(1 << index);
    // }

    pub fn get_bit(&self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
    }

    pub fn get_pieces(&self) -> Vec<usize> {
        let mut pieces = Vec::new();
        let mut bitboard = self.0;
        while bitboard != 0 {
            let index = bitboard.trailing_zeros() as usize;
            pieces.push(index);
            bitboard &= bitboard - 1;
        }
        pieces
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

    pub fn from_index(index: usize) -> Bitboard {
        let mask = 1u64 << index;
        Bitboard(mask)
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
