use std::time::Duration;
use std::io::{BufRead, BufReader};
use mouse_rs::Mouse;

fn main() {
    let mouse = Mouse::new();
    mouse.move_to(1280, 800).expect("Unable to move mouse");

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
                            if let Some((switch, x, y)) = parse_joystick_data(&buffer) {
                                println!("Switch: {}, X-axis: {}, Y-axis: {}", switch, x, y);
                                // 2560 × 1600, (517, 1023) min/max value on joystick
                                mouse.move_to(2560 / 1023 * x, 1600 / 1023 * y).expect("Unable to move mouse");
                                
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


fn parse_joystick_data(lines: &[String]) -> Option<(i32, i32, i32)> {
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