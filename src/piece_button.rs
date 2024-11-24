use egui::{ ImageButton, Pos2, Vec2, include_image };
use shogi:: { Piece, PieceType, Color };

pub struct PieceButton {
    pub button: ImageButton,
    pub piece: Piece,
    pub min: Pos2,
    pub size: Vec2,
    pub active: bool
}

impl PieceButton {
    pub fn new(&piece: Piece, &min: Pos2, &size: Vec2) {
        let mut image_button;
        match (piece.piece_type, piece.color) {
            (PieceType::Pawn, Color::Black) => { 
                image_button = ImageButton::new(include_image!("images/pieces/0FU.png")).frame(false);
            },
            _ => (),
        }

        Some(PieceButton {
            image_button,
            piece,
            min,
            size,
            false,  
        })
    }

    pub fn set_active(self) {
        self.active = true;
        self.size = Vec2::new(50.0, 50.0);
    }

    pub fn set_inactive(self) {
        self.active = false;
        self.size = Vec2::new(44.44, 44.44);
    }
}