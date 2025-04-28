use clap::{command, Parser};
use shogi::Position;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc};
use std::thread;
use std::io::{BufRead, BufReader};

mod shogi_game;
use shogi_game::ShogiGame;
mod board;
use board::Board;
mod piece_button;
use piece_button::{PieceButton, PIECE_TYPES};
mod joystick;
use joystick::Joystick;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the USI engine
    #[arg(long)]
    engine: String,

    /// Settings for the engine
    #[arg(long, value_parser = parse_key_val, num_args = 0..)]
    engine_option: Vec<(String, String)>,
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let parts: Vec<_> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("invalid KEY=VALUE: no `=` found in `{}`", s));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();
    let engine_options: HashMap<_, _> = args.engine_option.into_iter().collect();

    shogi::bitboard::Factory::init();
    let board = Board::new();
    let mut pos = Position::new();
    pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();  
    
    // Run apery engine
    let mut child = Command::new(&args.engine)
        .stdin(Stdio::piped())  
        .stdout(Stdio::piped()) 
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start Shogi engine");

    let engine_input = child.stdin.take().expect("Failed to open stdin");
    let engine_output = child.stdout.take().expect("Failed to open stdout");

    let (engine_tx, engine_rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let reader = BufReader::new(engine_output);
        for line in reader.lines() {
            match line {
                Ok(output) => {
                    if let Err(err) = engine_tx.send(output) {
                        eprintln!("Error sending engine output: {}", err);
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("Error reading engine output: {}", err);
                    break;
                }
            }
        }
    });

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([780.0, 740.0]).with_resizable(true).with_icon(Arc::new(load_icon())), 
        ..Default::default()
    };
    eframe::run_native(
        "Shogi",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ShogiGame::new(
                &cc.egui_ctx, 
                pos, 
                board,
                engine_input,
                engine_rx,
                &engine_options,
            )))
        }),
    )
}

// Load shogi icon (black king)
fn load_icon() -> egui::IconData {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("images/pieces/0GY.png");
		let image = image::load_from_memory(icon).expect("Failed to open icon path").into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	
	egui::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	}
}