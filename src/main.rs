#![allow(unused)]

use eframe::egui::{
    self, CentralPanel, Context, ViewportBuilder, Rect, Vec2, Pos2,
};
use shogi::{
    Position, Square, Move
};

mod board;
use board::Board;

mod piece_button;
use piece_button::PieceButton;


fn main() -> Result<(), eframe::Error> {
    shogi::bitboard::Factory::init();

    let mut pos = Position::new();
    pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();

    let mut board = Board::new();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Shogi",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ShogiGame::new(&cc.egui_ctx, pos, board)))
        }),
    )
}

struct ShogiGame<'a> {
    pos: Position,
    board: Board<'a>,
    error_message: String,
}

impl<'a> ShogiGame<'a> {
    fn new(_ctx: &Context, pos: Position, board: Board<'a>) -> Self {
        Self { pos, board, error_message: String::new() }
    }

    // runs in update function, renders piece_button based on board row/col
    fn render_pieces(&mut self, ui: &mut egui::Ui) {
        for rank in 0..9 {
            for file in 0..9 {
                let size = if self.board.active[0] == rank as i32 && self.board.active[1] == file as i32 {
                    Vec2::new(70.0, 70.0)
                } else {
                    Vec2::new(60.0, 60.0)
                };
    
                let min = Pos2::new(file as f32 * 60.0, rank as f32 * 60.0);
                let rect = Rect::from_min_size(min, size);
    
                let curr_piece = &self.board.piece_buttons[rank][file]; // PieceButton
    
                // pass in curr_piece PieceButton's ImageButton to ui
                if ui.put(rect, curr_piece.button.clone()).clicked() {
                    self.error_message = String::new();
                    let active = self.board.active.clone();
                   
                    // Try moving active piece into curr empty cell or capturing enemy piece
                    if active != [-1, -1] {
            
                        let active_piece = &self.board.piece_buttons[active[0] as usize][active[1] as usize];

                        if active_piece.piece != None && 
                            (curr_piece.piece == None || 
                            (curr_piece.piece != None && curr_piece.piece.unwrap().color != active_piece.piece.unwrap().color)) {

                            // FILE ORDER IS REVERSED, GOES FROM 9 to 1, rank a-i
                            // Square::new(file, rank), FILE FIRST

                            let from_sq = Square::new(active[1] as u8, active[0] as u8).unwrap();
                            let to_sq = Square::new(file as u8, rank as u8).unwrap();

                            // println!("{}", from_sq);
                            // println!("{}", to_sq);

                            let m = Move::Normal{from: from_sq, to: to_sq, promote: false};
                            self.pos.make_move(m).unwrap_or_else(|err| {
                                self.error_message = format!("Error in make_move: {}", err);
                                Default::default()
                            });  
                        }

                        self.board.set_active(-1, -1);

                    }
                    // change selection of ally piece
                    else if curr_piece.piece != None {
                        self.board.set_active(rank as i32, file as i32);
                        // println!("Rank: {}", rank);
                        // println!("File: {}", 9 - file);
                    }
                }

                
            }
        }
    }
}

impl<'a> eframe::App for ShogiGame<'_> {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {

            self.board.update_board(&self.pos);
            self.render_pieces(ui);

            ui.label(format!(" {}", self.error_message));
        }); 
    }
}