#![allow(unused)]

use eframe::egui::{
    self, CentralPanel, Context, ViewportBuilder, TextureHandle, Ui,
    Image, ImageButton, Rect, Vec2, Pos2, Align2,
    Painter, Rounding, Stroke, FontId, Color32,
};

use shogi::{
    Move, 
    // Position, 
    Square,
    // Piece,
    PieceType,
    Color
};

mod piece; 
use piece::Piece;

mod position;
use position::Position;

use shogi::bitboard::Factory as BBFactory;
use shogi::square::consts::*;


// use piece_button::PieceButton;

fn main() -> Result<(), eframe::Error> {
    BBFactory::init();
    let mut pos = Position::new();
    // initial board position
    pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();

    // init piece buttons and activation status
    for rank in 0..9 {
        for file in (0..9).rev() {
            let sq = shogi::Square::new(file, rank).unwrap();
            let piece = pos.piece_at(sq);
            if *piece != None {
                let mut piece = piece.unwrap();
                piece.set_button();
                piece.set_inactive();
            }
        }
    }

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Shogi",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); // image support
            Ok(Box::new(MyApp::new(&cc.egui_ctx, pos))) // init MyApp with context
        }),
    )
}

// #[derive(Default)]
struct MyApp {
    pos: Position,
}

impl MyApp {
    fn new(ctx: &Context, pos: Position) -> Self {
        Self { pos }
    }

    // https://github.com/nozaq/shogi-rs/blob/main/src/square.rs
    fn move_pawn(&mut self, sq: shogi::Square, color: shogi::Color) {
        // let m = Move::Normal{from: SQ_7G, to: SQ_7F, promote: false};
        // df is move file (left/right), dr is moving rank (up/down)

        let forward_sq = if color == shogi::Color::Black {
            sq.shift(0, -1).unwrap()
        } 
        else {
            sq.shift(0, 1).unwrap()
        };
        
        let m = Move::Normal{from: sq, to: forward_sq, promote: false};
        
        // unwrap or default to not crash when invalid.
        self.pos.make_move(m).unwrap();  
    }

    fn render_pieces(&mut self, ui: &mut egui::Ui) {
        for rank in 0..9 {
            for file in (0..9).rev() {
                let sq = shogi::Square::new(file, rank).unwrap();
                let piece = self.pos.piece_at(sq);

                // see piece_type.rs: https://github.com/nozaq/shogi-rs/blob/main/src/piece_type.rs#L23
                if *piece != None {
                    let mut piece = piece.unwrap();
                    let min  = Pos2::new(375.0 - (file as f32 * 44.44), 20.0 + (rank as f32 * 44.44));
                    let rect = Rect::from_min_size(min, piece.size);
                    
                    if ui.put(rect, piece.piece_button).clicked() {
                        if piece.active {
                            self.move_pawn(sq, piece.color);
                            piece.set_inactive();
                        }
                        else {
                            piece.set_active();
                        }
                    }

                    // let size = Vec2::new(44.44, 44.44);
                    // let rect = Rect::from_min_size(min, size);

                    // // unwrap Option<Piece> 
                    // let piece = piece.unwrap();
                    // match (piece.piece_type, piece.color) {
                    //     (shogi::PieceType::Pawn, shogi::Color::Black) => { 
                            
                    //         let piece_image_button = egui::ImageButton::new(egui::include_image!("images/pieces/0FU.png")).frame(false);
                    //         if ui.put(rect, piece_image_button).clicked() {
                    //             // self.show_available_moves(ui, piece, file, rank);
                    //             self.move_pawn(sq, piece.color); 
                    //         }
                    
                    //         // egui::Image::new(egui::include_image!("images/pieces/0FU.png")).paint_at(ui, rect);
                    //     },

                    //     (shogi::PieceType::Pawn, shogi::Color::White) => { 
                    //         let piece_image_button = egui::ImageButton::new(egui::include_image!("images/pieces/1FU.png")).frame(false);
                    //         // i can separate this later.
                    //         if ui.put(rect, piece_image_button).clicked() {
                    //             self.move_pawn(sq, piece.color);
                    //         }
                    //     },

    
                    //     (shogi::PieceType::Silver, shogi::Color::Black) => { egui::Image::new(egui::include_image!("images/pieces/0GI.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Silver, shogi::Color::White) => { egui::Image::new(egui::include_image!("images/pieces/1GI.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::King,   shogi::Color::Black) => { egui::Image::new(egui::include_image!("images/pieces/0GY.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::King,   shogi::Color::White) => { egui::Image::new(egui::include_image!("images/pieces/1GY.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Rook,   shogi::Color::Black) => { egui::Image::new(egui::include_image!("images/pieces/0HI.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Rook,   shogi::Color::White) => { egui::Image::new(egui::include_image!("images/pieces/1HI.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Bishop, shogi::Color::Black) => { egui::Image::new(egui::include_image!("images/pieces/0KA.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Bishop, shogi::Color::White) => { egui::Image::new(egui::include_image!("images/pieces/1KA.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Knight, shogi::Color::Black) => { egui::Image::new(egui::include_image!("images/pieces/0KE.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Knight, shogi::Color::White) => { egui::Image::new(egui::include_image!("images/pieces/1KE.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Gold,   shogi::Color::Black) => { egui::Image::new(egui::include_image!("images/pieces/0KI.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Gold,   shogi::Color::White) => { egui::Image::new(egui::include_image!("images/pieces/1KI.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Lance,  shogi::Color::Black) => { egui::Image::new(egui::include_image!("images/pieces/0KY.png")).paint_at(ui, rect); },
                    //     (shogi::PieceType::Lance,  shogi::Color::White) => { egui::Image::new(egui::include_image!("images/pieces/1KY.png")).paint_at(ui, rect); },
                    //     _ => (),
                    // }
                }
            }
        }
    }

    // fn show_available_moves(&mut self, ui: &mut egui::Ui, piece: shogi::Piece, rank: u8, file: u8) {
    //     let painter = ui.painter();
    //     match (piece.piece_type, piece.color) {
    //         (shogi::PieceType::Pawn,   shogi::Color::White) => {
    //             let min  = Pos2::new(20.0 + (file as f32 * 44.44), 375.4 - ((rank as f32 + 1.0) * 44.44));
    //             let size = Vec2::new(44.44, 44.44);

    //             let rect  = egui::Rect::from_min_size(min, size);
    //             let color = egui::Color32::from_rgba_unmultiplied(255, 0, 0, 128); // 50% opacity
    //             painter.rect_filled(rect, egui::Rounding::ZERO, color);
    //         },
    //         _ => (),
    //     }
    // }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {

            let min  = Pos2::new(20.0, 20.0);
            let size = Vec2::new(400.0, 400.0);
            let rect = Rect::from_min_size(min, size);
            egui::Image::new(egui::include_image!("images/boards/kaya1.jpg")).paint_at(ui, rect);

            let painter = ui.painter();

            // Remember to offset by min position (20.0) for lines and labels
            // rank = row, file = col
            // formula for position: 20.0 + 44.44 * (# file or rank)

            for i in 0i8..9 {
                // Paint rows
                let y     =  20.0 + 44.44 * f32::from(i);
                let start  = Pos2::new(20.0, y);
                let end    = Pos2::new(420.0, y);
                let stroke = Stroke::new(1.0, Color32::BLACK);
                let rank_label = ((b'a' + i as u8) as char).to_string();

                painter.line_segment([start, end], stroke);
                painter.text(
                    Pos2::new(430.0, y + 20.0),
                    egui::Align2::CENTER_CENTER,
                    rank_label,
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
        
                // Paint cols
                let x      = 20.0 + 44.44 * f32::from(i);
                let start  = Pos2::new(x, 20.0);
                let end    = Pos2::new(x, 420.0);
                let stroke = Stroke::new(1.0, Color32::BLACK);
                let file_label = (9 - i).to_string();

                painter.line_segment([start, end], stroke);
                painter.text(
                    Pos2::new(x + 20.0, 10.0),
                    egui::Align2::CENTER_CENTER,
                    file_label,
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
            }

            ui.monospace(format!("{}", self.pos));

            self.render_pieces(ui);

            // for rank in 0..9 {
            //     for file in (0..9).rev() {
            //         let sq = shogi::Square::new(file, rank).unwrap();
            //         let piece = self.pos.piece_at(sq);

            //         if *piece != None {
            //             let piece = piece.unwrap();
            //             self.show_available_moves(ui, piece, rank, file);
            //         }
            //     }
            // }
            
        }); 
    }
}