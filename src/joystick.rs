use std::time::Duration;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::Sender;

pub struct Joystick;

impl Joystick {
    pub fn new() -> Self { 
        Self 
    }

    pub fn init(&mut self, tx: Sender<(i32, i32, i32)>) {
        let port_name = "/dev/tty.usbmodem101";
        let baud_rate = 9600;

        let serial_port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(5000))
            .open();
        match serial_port {
            Ok(port) => {
                println!("Serial port opened successfully!");
    
                let reader = BufReader::new(port);
                let mut buffer = Vec::new();
    
                for line in reader.lines() {
                    match line {
                        Ok(data) => {
                            buffer.push(data);
                            if buffer.len() == 3 {
                                if let Some((switch, x, y)) = self.parse_joystick_data(&buffer) {
                                    // Map x and y to rank and file (0..8)
                                    // X = 0 (left), 517 (rest), 1023 (right)
                                    // Y = 0 (up),   518 (rest), 1023 (down)
                                    
                                    let joystick_center = 517;
                                    let joystick_max    = 1023;
                                    let rank = ((y as f32 / joystick_max as f32) * 8.0).round().clamp(0.0, 8.0) as i32;
                                    let file = ((x as f32 / joystick_max as f32) * 8.0).round().clamp(0.0, 8.0) as i32;

                                    // println!("Switch: {}, X: {}, Y: {}, Rank: {}, File: {}", 
                                    //     switch, x, y, rank, file);

                                    if tx.send((switch, rank, file)).is_err() {
                                        eprintln!("Joystick data send failed");
                                        break;
                                    }
                                }
                                buffer.clear();
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading from serial port: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open serial port: {}", e);
            }
        }
    }

    fn parse_joystick_data(&self, lines: &[String]) -> Option<(i32, i32, i32)> {
        if lines.len() == 3 {
            let switch = lines[0].trim().parse().ok()?; 
            let x = lines[1].trim().parse().ok()?;    
            let y = lines[2].trim().parse().ok()?; 
    
            Some((switch, x, y))
        } 
        else {
            None
        }
    }
}