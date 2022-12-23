use std::{io::Error, net::UdpSocket};

const CHUNK_SIZE: usize = 1024;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum SampleRate {
	V6kHz,
	V12kHz,
	V24kHz,
	V48kHz,
	V96kHz,
	V192kHz,
	V384kHz,
	V8kHz,
	V16kHz,
	V32kHz,
	V64kHz,
	V128kHz,
	V256kHz,
	V512kHz,
	V11_025kHz,
	V22_050kHz,
	V44_1kHz,
	V88_2kHz,
	V176_4kHz,
	V352_8kHz,
	V705_6kHz,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum Format {
	U8,  // 0x00
	S16, // 0x01
	S24, // 0x02
	S32, // 0x03
	F32, // 0x04
	F64, // 0x05
	B12, // 12b 0x06 - not supported by SPA/pipewire
	B10, // 10b 0x07 - not supported by SPA/pipewire
}

impl Format {
	fn size(self) -> usize {
		match self {
			Format::U8 => return 1,
			Format::S16 => return 2,
			Format::S24 => return 3,
			Format::S32 => return 4,
			Format::F32 => return 4,
			Format::F64 => return 8,
			Format::B10 => return 0,
			Format::B12 => return 0,
		}
	}
}

#[derive(Debug)]
pub struct Conn {
	socket: UdpSocket,
	frame_size: usize,
	counter: u32,
	data: Vec<u8>,
}

impl Conn {
	pub fn new(
		target: &str,
		format: Format,
		channels: u8,
		sample_rate: SampleRate,
		name: String,
	) -> Self {
		let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to create socket");
		socket.connect(target).expect("Failed to set target");

		let frame_size = format.size() * channels as usize;

		assert!(name.len() <= 16);

		println!(
			"Samplerate: {:?}, Channels: {}, Format: {:?}, Framesize: {}, Name: {}, Target: {}",
			sample_rate, channels, format, frame_size, name, target
		);
		println!("Socket: {:?}", socket);

		let mut vec: Vec<u8> = Vec::with_capacity(CHUNK_SIZE + 28);
		vec.resize(8, 0);
		// VBAN little endian
		vec[0] = 0x56;
		vec[1] = 0x42;
		vec[2] = 0x41;
		vec[3] = 0x4E;
		// SampleRate
		vec[4] = sample_rate as u8;
		// Samples
		// vec[5] see send
		// Channels
		vec[6] = channels - 1;
		// Format
		vec[7] = format as u8;
		vec.extend_from_slice(name.as_bytes());
		vec.resize(24, 0);

		Conn {
			socket: socket,
			frame_size: frame_size,
			counter: 0,
			data: vec,
		}
	}

	pub fn send(&mut self, data: &[u8], offset: u32, size: u32) -> Result<(), Error> {
		for s in data[offset as usize..(offset + size) as usize].chunks(CHUNK_SIZE) {
			self.data.truncate(24);
			self.data[5] = (s.len() / self.frame_size - 1) as u8;
			self.data.extend_from_slice(&self.counter.to_le_bytes()[..]);
			self.data.extend_from_slice(s);

			match self.socket.send(self.data.as_slice()) {
				Err(e) => println!("Failed to send data: {}", e),
				Ok(_) => {
					self.counter += 1;
				}
			};
		}
		return Ok(());
	}
}

impl Default for Conn {
	fn default() -> Self {
		Conn::new(
			"10.0.1.1:6980",
			Format::F32,
			2,
			SampleRate::V48kHz,
			String::from("Stream1"),
		)
	}
}
