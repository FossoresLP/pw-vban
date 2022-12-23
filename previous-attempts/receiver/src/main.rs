use pipewire;

mod audioinfo;

use std::net::UdpSocket;

use std::sync::mpsc;
use std::thread;

use ringbuf::{Producer, RingBuffer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::{Read, Write};
use std::pin::Pin;

use debug_print::debug_println as debug;

use pipewire::keys;
use pipewire::prelude::*;
use pipewire::properties;
use pipewire::stream;
use pipewire::MainLoop;

use audioinfo::AudioInfo;
use audioinfo::Format;

static mut STREAMS: Vec<stream::Stream<u8>> = Vec::new();

fn main() {
	let mainloop = MainLoop::new().expect("couldn't create mainloop");
	let (pw_sender, pw_receiver) = pipewire::channel::channel::<String>();
	let (web_sender, web_receiver) = mpsc::channel();
	let clone = mainloop.clone();
	let _recv = pw_receiver.attach(&mainloop, move |name| {
		println!("Creating stream: {}", name);
		let mainloop = &clone;
		let buf = RingBuffer::new(16384);

		let (prod, cons) = buf.split();
		let cons = RefCell::new(cons);

		web_sender.send(prod).unwrap();

		let stream = stream::Stream::simple(
			mainloop,
			&name,
			properties! {
				*keys::MEDIA_TYPE => "Audio",
				*keys::MEDIA_CATEGORY => "Playback",
				*keys::MEDIA_ROLE => "Music",
			},
		)
		.state_changed(|old, new| {
			println!("State changed: {:?} -> {:?}", old, new);
		})
		.process(move |stream, _| {
			let mut rb = cons.borrow_mut();
			if !rb.is_empty() {
				let opt = stream.dequeue_buffer();
				if opt.is_none() {
					return;
				}
				let mut pwbuf = opt.unwrap();
				let data = Pin::new(&mut pwbuf.datas_mut()[0]).get_mut();
				debug!("Found {} bytes ({} chunks)", rb.len(), rb.len() / 4);
				let opt = rb.read(data.data().unwrap());
				if opt.is_err() {
					return;
				}
				let l = opt.unwrap();
				debug!("Wrote {} bytes ({} chunks)", l, l / 4);
				let len = rb.len();
				if len > l {
					rb.discard(len - l);
				}

				let chunk = pwbuf.datas_mut()[0].chunk_mut();
				let offset = chunk.offset_mut();
				*offset = 0;
				let stride = chunk.stride_mut();
				*stride = 4;
				let size = chunk.size_mut();
				*size = l as u32;
			}
		})
		.create()
		.unwrap();

		let pod: Cursor<Vec<u8>> = Cursor::new(Vec::new());

		let pod = libspa::pod::serialize::PodSerializer::serialize(
			pod,
			&AudioInfo::stereo(Format::S16_LE, 48000),
		)
		.expect("failed to serialize pod")
		.0;

		println!("{:?}", pod);

		let vec = pod.into_inner();

		let ptr = vec.as_ptr();

		stream
			.connect(
				libspa::Direction::Output,
				None,
				pipewire::stream::StreamFlags::AUTOCONNECT
					| pipewire::stream::StreamFlags::MAP_BUFFERS
					| pipewire::stream::StreamFlags::RT_PROCESS,
				&mut [ptr as *const libspa_sys::spa_pod],
			)
			.unwrap();
		unsafe {
			STREAMS.push(stream);
		}
	});

	thread::spawn(move || {
		let mut producers: HashMap<String, Producer<u8>> = HashMap::new();
		let socket = UdpSocket::bind("0.0.0.0:6980").expect("couldn't bind to address");
		let mut buf: Vec<u8> = Vec::with_capacity(1464); // 1464 = 1500 - 36 (IP and UDP header) leaving 1436 bytes for data after 28 byte VBAN header
		loop {
			buf.resize(1464, 0);
			let (n, src_addr) = socket
				.recv_from(&mut buf.as_mut_slice())
				.expect("Didn't receive data");
			buf.truncate(n);
			debug!(
				"Received {} bytes from {}. Buffer size is {} of {}.",
				n,
				src_addr,
				buf.len(),
				buf.capacity()
			);
			if buf.len() < 28 {
				continue;
			}
			let header = &buf[..28];
			let name = string_from_bytes(&header[8..24]).expect("Invalid stream name");
			if !producers.contains_key(&name) {
				pw_sender.send(name.clone()).unwrap();
				let p: Producer<u8> = web_receiver.recv().unwrap();
				producers.insert(name.clone(), p);
			}
			let prod = producers.get_mut(&name).unwrap();
			//TODO: Parse header
			for chunk in buf.chunks_exact(4).skip(7) {
				if prod.remaining() < 4 {
					break;
				}
				prod.write_all(chunk)
					.expect("Fatal error when writing to buffer");
			}
		}
	});

	mainloop.run()
}

fn string_from_bytes(b: &[u8]) -> Result<String, std::str::Utf8Error> {
	let mut nul_index = 0;
	for (i, n) in b.iter().enumerate() {
		if n == &b'\0' {
			nul_index = i;
			break;
		}
	}
	Ok(std::str::from_utf8(&b[..nul_index])?.to_owned())
}

struct VBANHeader {}

fn vban_parse_header(b: &[u8]) -> Result<VBANHeader, &str> {
	return Err("not implemented");
}
