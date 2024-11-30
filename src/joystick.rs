use std::time::Duration;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;

pub struct Joystick {
    pub switch: i32,
    pub x: i32,
    pub y: i32,
    pub rank: u8,
    pub file: u8,
}

impl Joystick {
    pub fn new() -> Self { 
        Self { 
            switch: 0, 
            x: 0, 
            y: 0, 
            rank: 0, 
            file: 0,
        }
    }

    pub fn init(&mut self) {
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
                                    self.switch = switch;
                                    self.x = x;
                                    self.y = y;

                                    // Map x and y to rank and file (0..9)
                                    // X = 0 (left), 517 (rest), 1023 (right)
                                    // Y = 0 (up),   518 (rest), 1023 (down)
                                    self.rank = (y / 102).min(9) as u8;
                                    self.file = (x / 102).min(9) as u8;
                                    
                                    println!("Switch: {}, X: {}, Y: {}, Rank: {}, File: {}", 
                                        self.switch, self.x, self.y, self.rank, self.file);
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

impl Clone for Joystick {
    fn clone(&self) -> Self {
        Self {
            switch: self.switch,
            x: self.x,
            y: self.y,
            rank: self.rank,
            file: self.file,
        }
    }
}