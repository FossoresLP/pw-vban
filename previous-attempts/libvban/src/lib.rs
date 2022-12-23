use std::ffi::CStr;
use std::net::UdpSocket;

fn header(sample_rate: SampleRate, channels: u8, format: Format, name: String) -> [u8; 24] {
	let mut arr: [u8; 24] = [0; 24];
	// VBAN little endian
	arr[0] = 0x56;
	arr[1] = 0x42;
	arr[2] = 0x41;
	arr[3] = 0x4E;
	// SampleRate
	arr[4] = sr_u8(&sample_rate);
	// Samples arr[5] = 0;
	// Channels
	arr[6] = channels - 1;
	// Format
	arr[7] = format_u8(&format);
	// Name
	for (i, &x) in name.as_bytes().iter().enumerate() {
		arr[i + 8] = x;
	}
	// Counter arr[24..28] = 0
	return arr;
}

#[repr(u8)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
enum SampleRate {
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

fn sr_u8(sr: &SampleRate) -> u8 {
	*sr as u8
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Format {
	U8,  // 0x00
	S16, // 0x01
	S24, // 0x02
	S32, // 0x03
	F32, // 0x04
	F64, // 0x05
	     // 12b 0x06 - not supported by SPA/pipewire
	     // 10b 0x07 - not supported by SPA/pipewire
}

fn format_u8(format: &Format) -> u8 {
	*format as u8
}

fn format_map(format: u32) -> Format {
	match format {
		258 => return Format::U8,
		259 => return Format::S16,
		271 => return Format::S24,
		267 => return Format::S32,
		283 => return Format::F32,
		285 => return Format::F64,
		_ => return Format::U8,
	}
}

fn format_size(format: Format) -> usize {
	match format {
		Format::U8 => return 1,
		Format::S16 => return 2,
		Format::S24 => return 3,
		Format::S32 => return 4,
		Format::F32 => return 4,
		Format::F64 => return 8,
	}
}

pub trait Increment {
	fn increment(&mut self);
}

#[derive(Debug)]
pub struct Conn {
	socket: UdpSocket,
	header: [u8; 24],
	frame_size: usize,
	counter: u32,
}

impl Increment for Conn {
	fn increment(&mut self) {
		self.counter += 1;
	}
}

#[no_mangle]
pub extern "C" fn vban_init(
	a_target: *const std::os::raw::c_char,
	a_name: *const std::os::raw::c_char,
	a_format: u32,
	a_channels: u32,
	a_rate: u32,
) -> Box<Conn> {
	let target: &str;
	let name: String;

	unsafe {
		let t = CStr::from_ptr(a_target);
		target = t.to_str().expect("Failed to turn target into string");
		let n = CStr::from_ptr(a_name);
		name = n
			.to_str()
			.expect("Failed to turn name into string")
			.to_owned();
	}

	let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to create socket");
	socket.connect(target).expect("Failed to set target");

	let format = format_map(a_format);
	let channels = a_channels as u8;
	// TODO: Implement sample rate
	let frame_size = format_size(format) * channels as usize;

	println!(
		"Samplerate: {}, Channels: {}, Format: {:?}, Framesize: {}, Name: {}, Target: {}",
		a_rate, channels, format, frame_size, name, target
	);

	return Box::new(Conn {
		socket: socket,
		header: header(SampleRate::V48kHz, channels, format, name),
		frame_size: frame_size,
		counter: 0,
	});
}

#[no_mangle]
pub extern "C" fn vban_send(
	mut conn: Box<Conn>,
	raw: *const std::os::raw::c_void,
	size: u32,
) -> i32 {
	let data = unsafe { std::slice::from_raw_parts(raw as *const u8, size as usize) };
	for s in data.chunks(256 * conn.frame_size) {
		match send(&conn, s) {
			Ok(_) => conn.increment(),
			Err(_) => {
				Box::into_raw(conn);
				return -1;
			}
		}
	}

	// Keep box from getting freed at the end of the function
	Box::into_raw(conn);
	return 0;
}

fn send(conn: &Box<Conn>, slice: &[u8]) -> std::io::Result<usize> {
	let mut vec: Vec<u8> = Vec::with_capacity(slice.len() + 28);
	vec.extend_from_slice(&conn.header[..]);
	vec[5] = (slice.len() / conn.frame_size - 1) as u8;
	vec.extend_from_slice(&conn.counter.to_le_bytes()[..]);
	vec.extend_from_slice(slice);
	conn.socket.send(vec.as_slice())
}

#[no_mangle]
pub extern "C" fn vban_free(_conn: Box<Conn>) {}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
