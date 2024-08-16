// source for this code:
// https://github.com/de-vri-es/serial2-rs/blob/main/examples/serial-cat.rs
// Usage on raspberry pi with connected serial to usb cable. Connect the usb on your pc
// connect the haxophone with the cable in the following way (from left to right):
// pin 10 - pin 8 - pin 6
// UART RX - UART TX - GND --> these are the connectors on the haxophone
// TXD - RXD - GND ---> this is the cable
// Important note: connect RX to TX, and TX to RX
// connect to the haxophone via ssh
// ./pi_serial_test /dev/ttyAMA0 115200
// then connect cutecom or minicom
// write something in the ssh shell on the haxophone -> it will be displayed in cutecom
// write something as input on cutecom -> it will be displayed on stdout on the haxophone

use std::io::{Read, Write};
use std::sync::Arc;

use serial2::SerialPort;

fn do_main() -> Result<(), ()> {
	let args: Vec<_> = std::env::args().collect();
	if args.len() != 3 {
		let prog_name = args[0].rsplit_once('/').map(|(_parent, name)| name).unwrap_or(&args[0]);
		eprintln!("Usage: {} PORT BAUD", prog_name);
		return Err(());
	}

	let port_name = &args[1];
	let baud_rate: u32 = args[2]
		.parse()
		.map_err(|_| eprintln!("Error: invalid baud rate: {}", args[2]))?;

	let port = SerialPort::open(port_name, baud_rate)
		.map_err(|e| eprintln!("Error: Failed to open {}: {}", port_name, e))?;
	let port = Arc::new(port);

	// Spawn a thread to read from stdin and write to the serial port.
	std::thread::spawn({
		let port = port.clone();
		let port_name = port_name.to_owned();
		move || {
			if let Err(()) = read_stdin_loop(port, &port_name) {
				std::process::exit(1);
			}
		}
	});

	// Read from serial port and write to stdout in main thread.
	read_serial_loop(port, port_name)?;

	Ok(())
}

fn read_stdin_loop(port: Arc<SerialPort>, port_name: &str) -> Result<(), ()> {
	let stdin = std::io::stdin();
	let mut stdin = stdin.lock();
	let mut buffer = [0; 512];
	loop {
		let read = stdin
			.read(&mut buffer)
			.map_err(|e| eprintln!("Error: Failed to read from stdin: {}", e))?;
		if read == 0 {
			return Ok(());
		} else {
			port.write(&buffer[..read])
				.map_err(|e| eprintln!("Error: Failed to write to {}: {}", port_name, e))?;
		}
	}
}

fn read_serial_loop(port: Arc<SerialPort>, port_name: &str) -> Result<(), ()> {
	let mut buffer = [0; 512];
	loop {
		match port.read(&mut buffer) {
			Ok(0) => return Ok(()),
			Ok(n) => {
				std::io::stdout()
					.write_all(&buffer[..n])
					.map_err(|e| eprintln!("Error: Failed to write to stdout: {}", e))?;
			},
			Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
			Err(e) => {
				eprintln!("Error: Failed to read from {}: {}", port_name, e);
				return Err(());
			},
		}
	}
}

fn main() {
	if let Err(()) = do_main() {
		std::process::exit(1);
	}
}
