#![allow(unused)]

use egui::{
    ViewportBuilder, Context, CentralPanel, TextureHandle, Vec2, Image
};

use std::sync::Arc;

mod image_resources;
use image_resources::ImageResources;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Shogi",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); // image support
            Ok(Box::new(MyApp::new(&cc.egui_ctx))) // init MyApp with context
        }),
    )
}

// #[derive(Default)]
struct MyApp {
    images: Arc<ImageResources>,
}

impl MyApp {
    fn new(ctx: &Context) -> Self {
        let images = Arc::new(ImageResources::new(ctx));
        Self { images }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // ui.image(&self.images.board);
            ui.image(egui::include_image!("images/boards/painting1.jpg"));
        }); 
    }
}

// use shogi::{Move, Position};
// use shogi::bitboard::Factory as BBFactory;
// use shogi::square::consts::*;

// fn main() {
//     BBFactory::init();
//     let mut pos = Position::new();

//     // initial board position
//     pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();

// }
