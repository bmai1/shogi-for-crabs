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
            },
            (PieceType::Pawn, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1FU.png")).frame(false)
            },
            (PieceType::Silver, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0GI.png")).frame(false)
            },
            (PieceType::Silver, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1GI.png")).frame(false)
            },
            (PieceType::King, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0GY.png")).frame(false)
            },
            (PieceType::King, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1OU.png")).frame(false)
            },
            (PieceType::Rook, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0HI.png")).frame(false)
            },
            (PieceType::Rook, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1HI.png")).frame(false)
            },
            (PieceType::Bishop, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0KA.png")).frame(false)
            },
            (PieceType::Bishop, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1KA.png")).frame(false)
            },
            (PieceType::Knight, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0KE.png")).frame(false)
            },
            (PieceType::Knight, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1KE.png")).frame(false)
            },
            (PieceType::Gold, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0KI.png")).frame(false)
            },
            (PieceType::Gold, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1KI.png")).frame(false)
            },
            (PieceType::Lance, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0KY.png")).frame(false)
            },
            (PieceType::Lance, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1KY.png")).frame(false)
            },
            (PieceType::ProSilver, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0NG.png")).frame(false)
            },
            (PieceType::ProSilver, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1NG.png")).frame(false)
            },
            (PieceType::ProKnight, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0NK.png")).frame(false)
            },
            (PieceType::ProKnight, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1NK.png")).frame(false)
            },
            (PieceType::ProLance, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0NY.png")).frame(false)
            },
            (PieceType::ProLance, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1NY.png")).frame(false)
            },
            (PieceType::ProRook, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0RY.png")).frame(false)
            },
            (PieceType::ProRook, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1RY.png")).frame(false)
            },
            (PieceType::ProPawn, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0TO.png")).frame(false)
            },
            (PieceType::ProPawn, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1TO.png")).frame(false)
            },
            (PieceType::ProBishop, Color::Black) => {
                ImageButton::new(include_image!("images/pieces/0UM.png")).frame(false)
            },
            (PieceType::ProBishop, Color::White) => {
                ImageButton::new(include_image!("images/pieces/1UM.png")).frame(false)
            },
            _ => {
                ImageButton::new(include_image!("images/pieces/empty.png")).frame(false)
            },
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

    pub fn is_promoted(&self) -> bool {
        let p = self.piece.unwrap().piece_type;
        p == PieceType::ProPawn || p == PieceType::ProKnight || p == PieceType::ProLance || p == PieceType::ProSilver || p == PieceType::ProRook || p == PieceType::ProBishop
    }
}

// Used to iterate over hand.rs from shogi crate.
// Checks how many of each piece are in hand.
pub static PIECE_TYPES: [Piece; 14] = [
    Piece { piece_type: PieceType::Pawn,   color: Color::White },
    Piece { piece_type: PieceType::Lance,  color: Color::White },
    Piece { piece_type: PieceType::Knight, color: Color::White },
    Piece { piece_type: PieceType::Silver, color: Color::White },
    Piece { piece_type: PieceType::Gold,   color: Color::White },
    Piece { piece_type: PieceType::Bishop, color: Color::White },
    Piece { piece_type: PieceType::Rook,   color: Color::White },
    Piece { piece_type: PieceType::Pawn,   color: Color::Black },
    Piece { piece_type: PieceType::Lance,  color: Color::Black },
    Piece { piece_type: PieceType::Knight, color: Color::Black },
    Piece { piece_type: PieceType::Silver, color: Color::Black },
    Piece { piece_type: PieceType::Gold,   color: Color::Black },
    Piece { piece_type: PieceType::Bishop, color: Color::Black },
    Piece { piece_type: PieceType::Rook,   color: Color::Black },
];
