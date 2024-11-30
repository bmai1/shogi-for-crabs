

use shogi::{Position, Bitboard, Piece, Square};
use crate::PieceButton;

pub struct Board<'a> {
    pub piece_buttons: [[PieceButton<'a>; 9]; 9], 
    pub active: [i32; 2],
    pub active_moves: [[bool; 9]; 9],
}

impl<'a> Board<'a> {
    pub fn new() -> Self {
        let piece_buttons = std::array::from_fn(|_| {
            std::array::from_fn(|_| PieceButton::new())
        });

        Self {
            piece_buttons,
            active: [-1, -1],
            active_moves: [[false; 9]; 9],
        }
    }

    pub fn set_active(&mut self, rank: i32, file: i32) {
        if self.active == [rank, file] {
            self.active = [-1, -1];
        }
        else {
            self.active = [rank, file]
        }
    }

    pub fn set_active_moves(&mut self, pos: &Position, sq: Square, p: Piece) {
        self.active_moves = [[false; 9]; 9];

        let moves = pos.move_candidates(sq, p);
        for sq in moves {
            // println!("{}", sq);

            let rank = 8 - (sq.index() / 9); 
            let file = sq.index() % 9;

            self.active_moves[rank][file] = true;
        }
    }

    pub fn update_board(&mut self, pos: &Position) {
        for rank in 0..9 {
            for file in 0..9 {
                let sq = Square::new(file, rank).unwrap();
                if let Some(piece) = pos.piece_at(sq) {
                    self.piece_buttons[rank as usize][file as usize] = PieceButton::new_piece(*piece);
                } 
                else {
                    self.piece_buttons[rank as usize][file as usize] = PieceButton::new();
                }
            }
        }
    }
}