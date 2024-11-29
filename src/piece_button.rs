use egui::{ ImageButton, include_image };
use shogi::{ Piece, PieceType, Color };

pub struct PieceButton<'a> {
    pub button: ImageButton<'a>,
    pub piece: Option<Piece>,
}

impl<'a> PieceButton<'a> {
    pub fn new_piece(piece: Piece) -> Self {
        let button = match (piece.piece_type, piece.color) {
            (PieceType::Pawn, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0FU.png")).frame(false)
            }
            _ => {
                ImageButton::new(include_image!("images/pieces/1FU.png")).frame(false)
            }
        };

        PieceButton {
            button,
            piece: Some(piece.clone()),
        }
    }

    // Default constructor for empty cell
    pub fn new() -> Self {
        PieceButton {
            button: ImageButton::new(include_image!("images/pieces/empty.png")).frame(false),
            piece: None,
        }
    }
}