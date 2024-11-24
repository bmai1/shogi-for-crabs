use std::time::Duration;
use std::io::{BufRead, BufReader};
// use serialport::SerialPort;
// use mouse_rs::{types::keys::Keys, Mouse};
use mouse_rs::Mouse;

fn main() {
    let mouse = Mouse::new();
    // center mouse
    mouse.move_to(1280, 800).expect("Unable to move mouse");

    let port_name = "/dev/tty.usbmodem101";
    let baud_rate = 9600;

    // Open the serial port
    let serial_port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(5000))
        .open();

    match serial_port {
        Ok(port) => {
            println!("Serial port opened successfully!");

            let reader = BufReader::new(port);
            let mut buffer = Vec::new(); // To accumulate lines

            for line in reader.lines() {
                match line {
                    Ok(data) => {
                        buffer.push(data);

                        // Process data once we have 3 lines
                        if buffer.len() == 3 {
                            if let Some((switch, x, y)) = parse_joystick_data(&buffer) {
                                println!("Switch: {}, X-axis: {}, Y-axis: {}", switch, x, y);

                                // 2560 × 1600, (517, 1023) min/max value on joystick
                                mouse.move_to(2560 / 1023 * x, 1600 / 1023 * y).expect("Unable to move mouse");
                                
                            }

                            // Clear the buffer for the next set of readings
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
        let switch = lines[0].trim().parse().ok()?; // First line: Switch
        let x = lines[1].trim().parse().ok()?;     // Second line: X-axis
        let y = lines[2].trim().parse().ok()?;     // Third line: Y-axis

        Some((switch, x, y))
    } else {
        None
    }
}