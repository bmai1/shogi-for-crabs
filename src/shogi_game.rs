use eframe::egui::{CentralPanel, Context, Rect, Vec2, Pos2};
use shogi::{Position, Square, Move};
use std::process::ChildStdin;
use std::sync::mpsc;
use std::thread;
use std::io::Write;

use crate::Board;
use crate::Joystick;
use crate::PieceButton;
use crate::PIECE_TYPES;

pub struct ShogiGame<'a> {
    pos: Position,
    board: Board<'a>,
    promotion_flag: bool,
    error_message: String,
    engine_input: ChildStdin,
    engine_rx: mpsc::Receiver<String>,
    engine_ms: String, // Duration for engine calculation in ms
    joystick_rx: mpsc::Receiver<(i32, i32, i32)>,
    joystick_state: (i32, i32, i32), // (switch, rank, file)
}

impl<'a> ShogiGame<'a> {
    pub fn new(_ctx: &Context, pos: Position, board: Board<'a>, mut engine_input: ChildStdin, engine_rx: mpsc::Receiver<String>) -> Self {
        writeln!(engine_input, "isready").expect("Failed to start engine"); // Start engine

        // Start reading joystick
        let (joystick_tx, joystick_rx) = mpsc::channel();
        let mut joystick = Joystick::new();
        thread::spawn(move || {
            joystick.init(joystick_tx);
        });

        Self { 
            pos, 
            board, 
            promotion_flag: false,
            error_message: String::new(), 
            engine_input, 
            engine_rx, 
            engine_ms: String::from("3000"),
            joystick_rx,
            joystick_state: (-1, -1, -1),
        }
    }

    // Handle normal and drop moves, called from render_pieces when an ImageButton is clicked
    fn handle_piece_move(&mut self, rank: usize, file: usize, curr_piece: PieceButton) {
        let active      = self.board.active;
        let active_hand = self.board.active_hand;

        // Attempt normal move with active piece
        if active != [-1, -1] {
            let active_piece = &self.board.piece_buttons[active[0] as usize][active[1] as usize];

            if active_piece.piece != None && 
                (curr_piece.piece == None || 
                (curr_piece.piece != None && curr_piece.piece.unwrap().color != active_piece.piece.unwrap().color)) {

                // FILE ORDER IS REVERSED, GOES FROM 9 to 1, rank a-i
                // Square::new(file, rank)

                let from_sq = Square::new(active[1] as u8, active[0] as u8).unwrap();
                let to_sq = Square::new(file as u8, rank as u8).unwrap();

                // Force promotion for now for manual moves
                let m = if self.promotion_flag && !active_piece.is_promoted() && ((rank < 3 && self.pos.side_to_move() == shogi::Color::Black) || (rank > 5 && self.pos.side_to_move() == shogi::Color::White)) {
                    Move::Normal{from: from_sq, to: to_sq, promote: true}
                }
                else {
                    Move::Normal{from: from_sq, to: to_sq, promote: false}
                };

                self.error_message = format!("{}", m);
                self.pos.make_move(m).unwrap_or_else(|err| {
                    self.error_message = format!("Error in make_move: {}", err);
                    Default::default()
                });  
            }

            // Change selection of ally piece (active piece is same color as curr piece but different location)
            if active_piece.piece != None && curr_piece.piece != None && curr_piece.piece.unwrap().color == active_piece.piece.unwrap().color && active != [rank as i32, file as i32] {
                self.board.reset_activity();
                self.board.set_active(rank as i32, file as i32);
                let sq = Square::new(file as u8, rank as u8).unwrap();
                let piece = self.pos.piece_at(sq).unwrap();
                self.board.set_active_moves(&self.pos, Some(sq), piece)
            }
            else {
                self.board.reset_activity();
            }
        }
        // Clicked on side-to-move piece from inactive state
        else if curr_piece.piece != None && curr_piece.piece.unwrap().color == self.pos.side_to_move() {
            self.board.reset_activity();
            self.board.set_active(rank as i32, file as i32);
            let sq = Square::new(file as u8, rank as u8).unwrap();
            let piece = self.pos.piece_at(sq).unwrap();
            self.board.set_active_moves(&self.pos, Some(sq), piece)
        }
        // Attempt drop move with active hand piece if active hand piece matches side to move
        else if active_hand != usize::MAX {
            if (self.pos.side_to_move() == shogi::Color::Black && active_hand >= 7) || (self.pos.side_to_move() == shogi::Color::White && active_hand < 7) {
                let to_sq = Square::new(file as u8, rank as u8).unwrap();
                let m = Move::Drop{to: to_sq, piece_type: PIECE_TYPES[active_hand].piece_type};

                self.error_message = format!("{}", m);
                self.pos.make_move(m).unwrap_or_else(|err| {
                    self.error_message = format!("Error in make_move: {}", err);
                    Default::default()
                });  
            }
            self.board.reset_activity();         
        }
    }

    // Renders grid lines, promotion zone circles, and possible active moves
    fn render_grid(&mut self, ui: &mut egui::Ui) {
        let position_factor = 62.22;              // Multiplied by rank and file to get position (560 / 9 = 62.22)
        let (offset_x, offset_y) = (106.5, 56.5); // Offset from top-left
        let board_size = 560.0;                   // 560 x 560 px 
        let painter = ui.painter();

        let stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);

        for label in 0..9 {
            // Paint rows a-i
            let y      = label as f32 * position_factor + offset_y;
            let start  = Pos2::new(offset_x, y);
            let end    = Pos2::new(offset_x + board_size, y);
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

        // Render four promotion zone circles
        let radius = 3.0;
        let fill = egui::Color32::BLACK;

        painter.circle(Pos2::new(3.0 * position_factor + offset_x, 3.0 * position_factor + offset_y), radius, fill, stroke);
        painter.circle(Pos2::new(6.0 * position_factor + offset_x, 3.0 * position_factor + offset_y), radius, fill, stroke);
        painter.circle(Pos2::new(3.0 * position_factor + offset_x, 6.0 * position_factor + offset_y), radius, fill, stroke);
        painter.circle(Pos2::new(6.0 * position_factor + offset_x, 6.0 * position_factor + offset_y), radius, fill, stroke);
        
        // Render possible active moves 
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

    // Renders piece_buttons on board based on rank and file. Also renders pieces in hand and joystick location.
    fn render_pieces(&mut self, ui: &mut egui::Ui) {
        let position_factor = 62.22;               // Multiplied by rank and file to get (x, y) position
        let (offset_x, offset_y) = (106.5, 56.5);  // Offset from top-left
        let board_size = 560.0;                    // 560 x 560 px
    
        // Joystick input
        let mut switch_flag = false;
        if let Ok((switch, j_rank, j_file)) = self.joystick_rx.try_recv() {
            switch_flag = self.joystick_state.0 == 1 && switch == 0; // Detect when switch changes from 0 to 1 to simulate one click
            self.joystick_state = (switch, j_rank, j_file);
        }
        let (switch, j_rank, j_file) = self.joystick_state;
    
        // Green fill/stroke for active pieces
        let fill = egui::Color32::from_rgba_unmultiplied(60, 110, 40, 128);
        let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(60, 110, 40, 128));
    
        // Board needs to be drawn before pieces
        ui.add(egui::Image::new(egui::include_image!("images/boards/painting1.jpg")).fit_to_exact_size(egui::vec2(board_size, board_size)));
    
        // Render pieces on board
        for rank in 0..9 {
            for file in 0..9 {
                let (min, size) = (
                    Pos2::new(board_size - ((file + 1) as f32 * position_factor) + offset_x, rank as f32 * position_factor + offset_y), 
                    Vec2::new(60.0, 60.0)
                );
                let rect = Rect::from_min_size(min, size);
               
                // Marks active square
                if self.board.active == [rank as i32, file as i32] {
                    ui.painter().rect(rect, 0.0, fill, stroke);
                }
    
                // Clone curr_piece and curr_piece.button to avoid borrowing issues
                let curr_piece = self.board.piece_buttons[rank][file].clone(); // PieceButton
                if ui.put(rect, curr_piece.button.clone()).clicked() || (switch_flag && j_rank == rank as i32 && (8 - j_file) == file as i32) {
                    self.handle_piece_move(rank, file, curr_piece);
                }
            }
        }
    
        // Render pieces in hand
        for i in 0..14 {
            let p = PIECE_TYPES[i];
            let pb = PieceButton::new_piece(p);
            let count = self.pos.hand(p);
    
            let (x, y) = match p.color {
                shogi::Color::Black => (board_size + offset_x + 25.0, board_size - 10.0 - ((i % 7) as f32 * position_factor)),
                shogi::Color::White => (25.0, offset_y - 1.0 + (i % 7) as f32 * position_factor),
            };
    
            let min  = Pos2::new(x, y);
            let size = Vec2::new(60.0, 60.0);
            let rect = Rect::from_min_size(min, size);
    
            if count != 0 {
                // Mark active hand piece
                if self.board.active_hand == i {
                    ui.painter().rect(rect, 0.0, fill, stroke);
                }
                if ui.put(rect, pb.button).clicked() && p.color == self.pos.side_to_move() {
                    let tmp = self.board.active_hand; // Deselect hand piece on reclick
                    self.board.reset_activity();
                    if tmp != i {
                        self.board.set_active_hand(i);
                        self.board.set_active_moves(&self.pos, None, p);
                    }
                }
            }
            else {
                ui.put(rect, pb.button);
                // Hand pieces with count of 0 are semi-opaque
                let fill = egui::Color32::from_rgba_unmultiplied(23, 23, 23, 128);
                let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(23, 23, 23, 128));
                ui.painter().rect(rect, 0.0, fill, stroke);
            }
        }
    
        // Show joystick location
        if switch != -1 {
            let (min, size) = (
                Pos2::new(board_size - ((9 - j_file) as f32 * position_factor) + offset_x, j_rank as f32 * position_factor + offset_y),
                Vec2::new(60.0, 60.0),
            );
            let rect = Rect::from_min_size(min, size);
            ui.painter().rect(rect, 0.0, fill, stroke);
            // self.error_message = format!("{} {} {}", switch, rank, file);
        }
    }

    // Apery engine communication
    fn make_engine_move(&mut self) {
        if let Ok(parsed) = self.engine_ms.parse::<i32>() {
            if parsed <= 0 {
                self.error_message = String::from("Engine calculation time must be positive.");
                return;
            }
            if parsed > 10000 {
                self.error_message = String::from("Engine calculation time must be less than 10000 ms.");
                return;
            }
        } 
        else {
            self.error_message = String::from("Engine calculation time must be an integer.");
            return;
        }

        writeln!(self.engine_input, "position sfen {}", self.pos.to_sfen()).expect("Failed to set board position");
        writeln!(self.engine_input, "go byoyomi {}", self.engine_ms).expect("Failed to find best move");

        while let Ok(line) = self.engine_rx.recv() {
            if line.starts_with("bestmove") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let best_move = parts[1].to_string();
                self.error_message = format!("{}", best_move);

                let m = Move::from_sfen(&best_move).unwrap();
                self.pos.make_move(m).unwrap_or_else(|err| {
                    self.error_message = format!("Error in make_move: {}", err);
                    Default::default()
                });

                self.board.reset_activity();
                break;
            }
        }
    }

    #[allow(dead_code)]
    fn engine_vs_player(&mut self) {
        // let engine play against you
        todo!();
    }

    #[allow(dead_code)]
    fn engine_vs_engine(&mut self) {
        // let engine play against itself
        todo!();
    }

    #[allow(dead_code)]
    fn replay_game(&mut self) {
        // https://www.youtube.com/watch?v=6z8MzWRm__s&list=PLB3D925021814AD0A
        // extract sfen strings of each state and replay
        todo!();
    }

    // New game: reset board, position, and engine
    fn new_game(&mut self) {
        self.board = Board::new();
        self.pos = Position::new();
        self.pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();  
        writeln!(self.engine_input, "position startpos").expect("Failed to reset board position");
        self.error_message.clear();
    }

    // Undo move with shogi::Position MoveHistory
    fn undo_move(&mut self) {
        self.pos.unmake_move().unwrap();
        writeln!(self.engine_input, "position sfen {}", self.pos.to_sfen()).expect("Failed to undo move");
        self.error_message.clear();
    }
}

impl<'a> eframe::App for ShogiGame<'_> {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            egui::Frame::default()
                .inner_margin(egui::Margin { left: 100.0, right: 100.0, top: 50.0, bottom: 50.0 })
                .show(ui, |ui| {
                    self.board.update_board(&self.pos);
                    self.render_pieces(ui);
                    self.render_grid(ui); 

                    ui.add_space(390.0);
                    ui.horizontal(|ui| {
                        if ui.button(format!("Make Engine Move ({})", self.pos.side_to_move())).clicked() {
                            self.make_engine_move();
                        }
                        ui.label("Duration:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.engine_ms)
                                .desired_width(40.0)
                        );
                        ui.label("ms");
                    });
                    ui.horizontal(|ui| {
                        if ui.button(format!("New game")).clicked() {
                            self.new_game();
                        }
                        if ui.button(format!("Undo move")).clicked() {
                            self.undo_move();
                        }
                        if ui.button(format!("Promotion: {}", self.promotion_flag)).clicked() {
                            self.promotion_flag = !self.promotion_flag;
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Print SFEN").clicked {
                            println!("{}", self.pos.to_sfen());
                        }
                        if ui.button("Castle Presets").clicked() {
                            // TODO: egui-dropdown listing castles
                            self.new_game();
                            let castle_sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 moves 2h7h 8b3b 5i4h 5a6b 4h3h 6b7b 3h2h 7b8b 3i3h 7a7b 6i5h 4a5b 1g1f";
                            self.pos.set_sfen(castle_sfen).expect("Failed to set castle position.");
                        }
                    });
                    if !self.error_message.is_empty() {
                        ui.label(format!("{}", self.error_message));
                    }

                    ctx.request_repaint(); // Manual repaint for joystick location
                });
        }); 
    }
}