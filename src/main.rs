use std::io::{Read, Write};
use std::time::Duration;
use nmea_parser::{NmeaParser, ParsedMessage};

fn main() {
    let ports = serialport::available_ports().expect("System error");
    let port = ports.first().expect("No ports available");
    println!("Receiving data on {} at {} baud:", &port.port_name, 9600);

    let mut port = serialport::new(&port.port_name, 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect(&format!("Unable to open serial port '{}'", port.port_name));

    let mut buffer = Vec::new();
    let mut last = 0;
    let mut serial_buf = [0u8; 1024];

    let mut parser = NmeaParser::new();

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let slice = &serial_buf[..t];
                let len = buffer.len();
                buffer.extend_from_slice(slice);
                if let Some(idx) = slice.iter().position(|c| *c == b'\n') {
                    let line = String::from_utf8_lossy(&buffer[last..(len + idx)]);
                    last = len + idx + 1;

                    if let Ok(sentence) = parser.parse_sentence(&line) {
                        match sentence {
                            ParsedMessage::Rmc(rmc) => {
                                if let Some(timestamp) = rmc.timestamp {
                                    println!("Time:    {}", timestamp);
                                }
                            },
                            _ => {}
                        }
                    }
                }
                // std::io::stdout().write_all(&serial_buf[..t]).unwrap()
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            other => {
                other.unwrap();
            },
        }
    }
}
