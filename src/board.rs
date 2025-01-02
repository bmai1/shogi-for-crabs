

use shogi::{Position, Piece, Square};
use crate::PieceButton;

pub struct Board<'a> {
    pub piece_buttons: [[PieceButton<'a>; 9]; 9], 
    pub active: [i32; 2],
    pub active_hand: usize, // 0 - 13 representing piece types
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
            active_hand: usize::MAX,
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

    pub fn set_active_hand(&mut self, i: usize) {
        self.active_hand = i;
    }

    pub fn set_active_moves(&mut self, pos: &Position, sq: Option<Square>, p: Piece) {
        self.active_moves = [[false; 9]; 9];

        // Drop move when the square of the piece is None
        if sq.is_none() {
            self.drop_candidates(pos, p);
        }
        // Normal moves from Bitboard
        else {
            let moves = pos.move_candidates(sq.unwrap(), p);
            for sq in moves {
                let rank = 8 - (sq.index() / 9); 
                let file = sq.index() % 9;
                self.active_moves[rank][file] = true;
            }
        }
    }

    pub fn reset_activity(&mut self) {
        self.set_active(-1, -1);
        self.set_active_hand(usize::MAX);
        self.active_moves = [[false; 9]; 9];
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
    
    // TODO: Find potential drop moves
    pub fn drop_candidates(&mut self, pos: &Position, p: Piece) {
        // If pawn, can drop in any unoccupied square in a file without pawn
        if p.piece_type == shogi::PieceType::Pawn {
            let mut pawn_files = [false; 9];
            for rank in 0..9 {
                for file in 0..9 {
                    let sq = Square::new(file, rank).unwrap();
                    if let Some(piece) = pos.piece_at(sq) {
                        if piece.piece_type == shogi::PieceType::Pawn && piece.color == pos.side_to_move() {
                            pawn_files[file as usize] = true;
                        }
                    }
                }
            }
            for rank in 0..9 {
                for file in 0..9 {
                    let sq = Square::new(file, rank).unwrap();
                    if !pawn_files[file as usize] && pos.piece_at(sq).is_none() {
                        let r = 8 - (sq.index() / 9); 
                        let f = sq.index() % 9;
                        self.active_moves[r][f] = true;
                    }
                }
            }
        }
        // Other pieces can drop in any unoccupied square
        else {
            for rank in 0..9 {
                for file in 0..9 {
                    let sq = Square::new(file, rank).unwrap();
                    if pos.piece_at(sq).is_none() {
                        let r = 8 - (sq.index() / 9); 
                        let f = sq.index() % 9;
                        self.active_moves[r][f] = true;
                    }
                }
            }
        }
    }
}