use std::io::{Read, Write};
use std::time::Duration;

fn main() {
    let ports = serialport::available_ports().expect("System error");
    let port = ports.first().expect("No ports available");
    let baud = 9600;

    println!("Receiving data on {} at {} baud:", &port.port_name, baud);

    let mut port = serialport::new(&port.port_name, baud)
        .timeout(Duration::from_millis(10))
        .open()
        .expect(&format!("Unable to open serial port '{}'", port.port_name));

    let mut serial_buf = [0u8; 1024];

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let slice = &serial_buf[..t];
                eprintln!("{} bytes received: {:?}", t, slice);
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            other => {
                other.unwrap();
            },
        }
    }
}
