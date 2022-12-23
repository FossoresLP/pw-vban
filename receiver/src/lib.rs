use libspa as spa;
use libspa_sys as spa_sys;
use pipewire as pw;
use pipewire_sys as pw_sys;
use pw::prelude::ListenerBuilderT;
use std::os::raw::{c_char, c_int};

use std::boxed::Box;

mod audioinfo;
use audioinfo::AudioInfo;
use audioinfo::Format;

mod vban;
use vban::Conn;

use pipewire::prelude::ReadableDict;
use pipewire::prelude::WritableDict;

static MODULE_PROPERTIES: spa::StaticDict = spa::static_dict! {
	"module.author" => env!("CARGO_PKG_AUTHORS"),
	"module.description" => env!("CARGO_PKG_DESCRIPTION"),
	"module.usage" => "",
	"module.version" => env!("CARGO_PKG_VERSION")
};

#[no_mangle]
pub unsafe extern "C" fn pipewire__module_init(
	module: *mut pw_sys::pw_impl_module,
	args: *const c_char,
) -> c_int {
	let context = pw_sys::pw_impl_module_get_context(module);
	let id = pw_sys::pw_global_get_id(pw_sys::pw_impl_module_get_global(module));

	println!("Loading module");

	if args.is_null() {
		println!("arguments is null - aborting...");
		return -1;
	}

	let mut props = {
		let raw = pw_sys::pw_properties_new_string(args);
		let opt = std::ptr::NonNull::new(raw);
		match opt {
			None => return -1,
			Some(nn) => pw::Properties::from_ptr(nn),
		}
	};

	let _work = pw_sys::pw_context_get_work_queue(context);

	if props.get("node.group").is_none() {
		props.insert("node.group", "pipewire.dummy");
	}
	if props.get("node.virtual").is_none() {
		props.insert("node.virtual", "true");
	}
	if props.get("media.class").is_none() {
		props.insert("media.class", "Audio/Sink");
	}
	if props.get("node.name").is_none() {
		props.insert("node.name", &format!("test-sink-{}", id));
	}
	if props.get("node.description").is_none() {
		props.insert("node.description", &format!("Test Sink (ID: {})", id));
	}
	let target = match props.get("vban.target") {
		Some(str) => str.to_string(),
		None => {
			println!("vban.target is required");
			return -1;
		}
	};
	let name = match props.get("vban.name") {
		Some(str) => str.to_string(),
		None => {
			println!("vban.name is required");
			return -1;
		}
	};

	let stream_props = match props.get("stream.props") {
		None => {
			let raw = pw_sys::pw_properties_new(std::ptr::null(), std::ptr::null::<c_char>());
			let opt = std::ptr::NonNull::new(raw);
			match opt {
				None => return -1,
				Some(nn) => pw::Properties::from_ptr(nn),
			}
		}
		Some(str) => {
			let cstr = std::ffi::CString::new(str).expect("CString::new failed");
			let raw = pw_sys::pw_properties_new_string(cstr.as_ptr());
			let opt = std::ptr::NonNull::new(raw);
			match opt {
				None => return -1,
				Some(nn) => pw::Properties::from_ptr(nn),
			}
		}
	};

	println!("Stream props: {:?}", stream_props);

	pw_sys::pw_impl_module_update_properties(module, MODULE_PROPERTIES.get_dict_ptr());

	let core = {
		let cstr = std::ffi::CString::new("Pipewire:Interface:Core").expect("CString::new failed");
		let ptr = pw_sys::pw_context_get_object(context, cstr.as_ptr()) as *mut pw_sys::pw_core;
		match std::ptr::NonNull::new(ptr) {
			Some(c) => Box::new(pw::Core::from_ptr(c)),
			None => {
				let ptr = pw_sys::pw_context_connect(context, std::ptr::null_mut(), 0);
				match std::ptr::NonNull::new(ptr) {
					Some(c) => Box::new(pw::Core::from_ptr(c)),
					None => return -1,
				}
			}
		}
	};

	let mut stream = match pw::stream::Stream::<Conn>::new(&core, "vban sink", props) {
		Err(_) => {
			println!("Failed to create stream");
			return -1;
		}
		Ok(s) => Box::new(s),
	};

	let info = match AudioInfo::stereo(Format::F32LE, 48000).build() {
		Err(_) => {
			println!("Failed to generate spa_pod");
			return -1;
		}
		Ok(i) => i,
	};

	let conn = Conn::new(
		&target,
		vban::Format::F32,
		2,
		vban::SampleRate::V48kHz,
		name,
	);

	let listener = match stream
		.add_local_listener_with_user_data(conn)
		.state_changed(state_changed)
		.process(process)
		.register()
	{
		Err(_) => {
			println!("Failed to register listener");
			return -1;
		}
		Ok(l) => Box::new(l),
	};

	match stream.connect(
		spa::Direction::Input,
		None,
		pw::stream::StreamFlags::AUTOCONNECT
			| pw::stream::StreamFlags::MAP_BUFFERS
			| pw::stream::StreamFlags::RT_PROCESS,
		&mut [info],
	) {
		Err(_) => {
			println!("Failed to connect stream");
			return -1;
		}
		Ok(_) => {}
	};

	Box::into_raw(listener);
	Box::into_raw(stream);
	Box::into_raw(core);

	return 0;
}

fn state_changed(old: pw::stream::StreamState, new: pw::stream::StreamState) {
	println!("Stream state changed from {:?} to {:?}", old, new);
}

fn process(s: &pw::stream::Stream<Conn>, conn: &mut Conn) {
	let mut buf = s.dequeue_buffer().expect("Failed to dequeue buffer");
	for d in buf.datas_mut() {
		let chunk = d.chunk();
		let offset = chunk.offset();
		let size = chunk.size();
		let _stride = chunk.stride();
		let data = d.data().expect("Failed to get data");

		conn.send(data, offset, size)
			.expect("Unhandled IO error when sending packet");
	}
}

/*
node.group
node.virtual
media.class
node.name
node.description
stream.props
audio.rate
audio.channels
audio.position
*/
