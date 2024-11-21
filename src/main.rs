#![allow(unused)]

use eframe::egui::{
    self, CentralPanel, Context, ViewportBuilder, TextureHandle, 
    Image, Rect, Vec2, Pos2, Align2,
    Painter, Stroke, FontId, Color32
};

use std::sync::Arc;

mod image_resources;
use image_resources::ImageResources;

use shogi::{Move, Position};
use shogi::bitboard::Factory as BBFactory;
use shogi::square::consts::*;

fn main() -> Result<(), eframe::Error> {
    BBFactory::init();
    let mut pos = Position::new();
    // initial board position
    pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();

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
    images: Arc<ImageResources>,
}

impl MyApp {
    fn new(ctx: &Context, pos: Position) -> Self {
        let images = Arc::new(ImageResources::new(ctx));
        Self { pos, images }
    }

    fn move_pawn_for_fun(&mut self) {
        let m = Move::Normal{from: SQ_7G, to: SQ_7F, promote: false};
        self.pos.make_move(m).unwrap();  
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // allocates space and pusheds other widgets down
            // ui.image(egui::include_image!("images/boards/painting1.jpg"));

            let min = Pos2::new(20.0, 20.0); // Top-left corner
            let size = Vec2::new(400.0, 400.0); // Width and height
            let rect = Rect::from_min_size(min, size);

            egui::Image::new(egui::include_image!("images/boards/kaya1.jpg")).paint_at(ui, rect);

            let painter = ui.painter();

            // Remember to offset by min position (20.0) for lines and labels
            for i in 0i8..9 {
                // Paint rows
                let y     =  20.0 + 44.44 * f32::from(i);
                let start  = Pos2::new(20.0, y);
                let end    = Pos2::new(420.0, y);
                let stroke = Stroke::new(1.0, Color32::BLACK);
                let row_label = ((b'a' + i as u8) as char).to_string();

                painter.line_segment([start, end], stroke);
                painter.text(
                    Pos2::new(430.0, y + 20.0),
                    egui::Align2::CENTER_CENTER,
                    row_label,
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
        
                // Paint cols
                let x      = 20.0 + 44.44 * f32::from(i);
                let start  = Pos2::new(x, 20.0);
                let end    = Pos2::new(x, 420.0);
                let stroke = Stroke::new(1.0, Color32::BLACK);
                let col_label = (9 - i).to_string();

                painter.line_segment([start, end], stroke);
                painter.text(
                    Pos2::new(x + 20.0, 10.0),
                    egui::Align2::CENTER_CENTER,
                    col_label,
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );

            }

            ui.monospace(format!("{}", self.pos));

            if ui.button("move").clicked() {
                self.move_pawn_for_fun(); 
            }
        }); 
    }
}