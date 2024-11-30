#![allow(unused)]

use eframe::egui::{
    self, CentralPanel, Context, ViewportBuilder, Rect, Vec2, Pos2,
};

use shogi::{Position, Square, Move};

mod board;
use board::Board;

mod piece_button;
use piece_button::PieceButton;


fn main() -> Result<(), eframe::Error> {
    shogi::bitboard::Factory::init();

    let mut pos      = Position::new();
    let mut board    = Board::new();
    pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();
    
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1000.0, 675.0]), 
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
        Self { pos, board, error_message: String::new()}
    }
    
    fn render_board(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Image::new(egui::include_image!("images/boards/kaya1.jpg")));

        let position_factor = 62.22;              // Multiplied by rank and file to get position (560 / 9 = 62.22)
        let (offset_x, offset_y) = (106.5, 56.5); // Offset from top-left
        let board_size = 560.0;                   // 560 x 560 px 
        let painter = ui.painter();

        for label in 0..9 {
            // Paint rows a-i
            let y      = label as f32 * position_factor + offset_y;
            let start  = Pos2::new(offset_x, y);
            let end    = Pos2::new(offset_x + board_size, y);
            let stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
            let rank_label = ((b'a' + label as u8) as char).to_string();

            painter.line_segment([start, end], stroke);
            painter.text(
                Pos2::new(board_size + offset_x + 10.0, y + offset_y - 25.0),
                egui::Align2::CENTER_CENTER,
                rank_label,
                egui::FontId::default(),
                egui::Color32::GRAY,
            );
    
            // Paint cols 9-1
            let x      = label as f32 * position_factor + offset_x;
            let start  = Pos2::new(x, offset_y);
            let end    = Pos2::new(x, offset_y + board_size);
            let stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
            let file_label = (9 - label).to_string();

            painter.line_segment([start, end], stroke);
            painter.text(
                Pos2::new(x + 30.0, offset_y - 10.0),
                egui::Align2::CENTER_CENTER,
                file_label,
                egui::FontId::default(),
                egui::Color32::GRAY,
            );
        }

        // render possible active moves 
        for rank in 0..9 {
            for file in 0..9 {
                if self.board.active_moves[rank][file] {
                    let center = Pos2::new(rank as f32 * position_factor + offset_x + position_factor / 2.0, file as f32 * position_factor + offset_y + position_factor / 2.0);
                    let radius = 7.0;
                    let fill = egui::Color32::from_rgba_unmultiplied(60, 110, 40, 128);
                    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(60, 110, 40, 128));
                    painter.circle(center, radius, fill, stroke);
                }
            }
        }
    }


    // runs in update function, renders piece_button based on board row/col
    fn render_pieces(&mut self, ui: &mut egui::Ui) {
        for rank in 0..9 {
            for file in 0..9 {

                let active = self.board.active;
                let position_factor = 62.22;               // Multiplied by rank and file to get position
                let (offset_x, offset_y) = (106.5, 56.5);     // Offset from top-left
                let board_size = 560.0;

                let (min, size) = if [rank as i32, file as i32] == active {
                    (
                        Pos2::new(board_size - ((file + 1) as f32 * position_factor) + offset_x - 2.5, rank as f32 * position_factor - 2.5 + offset_y),
                        Vec2::new(65.0, 65.0),
                    )
                } 
                else {
                    (
                        Pos2::new(board_size - ((file + 1) as f32 * position_factor) + offset_x, rank as f32 * position_factor + offset_y),
                        Vec2::new(60.0, 60.0),
                    )
                };
                
                let rect = Rect::from_min_size(min, size);
                let curr_piece = &self.board.piece_buttons[rank][file]; // PieceButton
    
                // pass in curr_piece PieceButton's ImageButton to ui
                if ui.put(rect, curr_piece.button.clone()).clicked() {
                    
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

                            self.error_message = format!("{} to {}", from_sq, to_sq);

                            let m = Move::Normal{from: from_sq, to: to_sq, promote: false};
                            self.pos.make_move(m).unwrap_or_else(|err| {
                                self.error_message = format!("Error in make_move: {}", err);
                                Default::default()
                            });  
                        }

                        self.board.set_active(-1, -1);
                        self.board.active_moves = [[false; 9]; 9];
                    }
                    // change selection of ally piece
                    else if curr_piece.piece != None {
                        self.board.set_active(rank as i32, file as i32);
                       
                        // println!("Rank: {}", rank);
                        // println!("File: {}", 9 - file);

                        let sq = Square::new(file as u8, rank as u8).unwrap();
                        let piece = self.pos.piece_at(sq).unwrap();
                        self.board.set_active_moves(&self.pos, sq, piece)
                    }
                }

                
            }
        }
    }
}

impl<'a> eframe::App for ShogiGame<'_> {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            egui::Frame::default()
                .inner_margin(egui::Margin { left: 100.0, right: 100.0, top: 50.0, bottom: 50.0 })
                .show(ui, |ui| {
                    self.board.update_board(&self.pos);
                    self.render_board(ui);
                    self.render_pieces(ui);

                    // ui.monospace(format!("{}", self.pos));

                    if !self.error_message.is_empty() {
                        ui.add_space(15.0);
                        ui.label(format!("{}", self.error_message));
                    }
                });
        }); 
    }
}