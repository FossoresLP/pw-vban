#[repr(C)]
pub struct r#impl {
	context: *mut pipewire_sys::pw_context,
	props: *mut pipewire_sys::pw_properties,
	module: *mut pipewire_sys::pw_impl_module,
	work: *mut pipewire_sys::pw_work_queue,
	module_listener: *mut libspa_sys::spa_hook,
	core: *mut pipewire_sys::pw_core,
	core_proxy_listener: *mut libspa_sys::spa_hook,
	core_listener: *mut libspa_sys::spa_hook,
	stream_props: *mut pipewire_sys::pw_properties,
	stream: *mut pipewire_sys::pw_stream,
	stream_listener: *mut libspa_sys::spa_hook,
	info: *mut libspa_sys::spa_audio_info_raw,
	frame_size: u32,
	do_disconnect: u32,
	unloading: u32,
}

#[no_mangle]
pub unsafe extern "C" fn pipewire__module_init(
	module: *mut pipewire_sys::pw_impl_module,
	args: *const i8,
) -> i32 {
	let ctx = pipewire_sys::pw_impl_module_get_context(module);

	let PW_TYPE_INTERFACE_Core =
		std::ffi::CString::new("PipeWire:Interface:Core\0").expect("CString::new failed");

	let out: *mut r#impl = &mut r#impl {
		context: ctx,
		props: pipewire_sys::pw_properties_new_string(args),
		module: module,
		work: pipewire_sys::pw_context_get_work_queue(ctx),
		module_listener: std::ptr::null_mut(),
		core: pipewire_sys::pw_context_get_object(ctx, PW_TYPE_INTERFACE_Core.as_ptr())
			as *mut pipewire_sys::pw_core,
		core_proxy_listener: std::ptr::null_mut(),
		core_listener: std::ptr::null_mut(),
		stream_props: pipewire_sys::pw_properties_new(&0, &0),
		stream: std::ptr::null_mut(),
		stream_listener: std::ptr::null_mut(),
		info: std::ptr::null_mut(),
		frame_size: 0,
		do_disconnect: 1,
		unloading: 0,
	};

	let id: u32 = pipewire_sys::pw_global_get_id(pipewire_sys::pw_impl_module_get_global(module));

	if (pw_properties_get(props, PW_KEY_NODE_GROUP) == NULL)
			pw_properties_set(props, PW_KEY_NODE_GROUP, "pipewire.dummy");
		if (pw_properties_get(props, PW_KEY_NODE_VIRTUAL) == NULL)
			pw_properties_set(props, PW_KEY_NODE_VIRTUAL, "true");

		if (pw_properties_get(props, PW_KEY_MEDIA_CLASS) == NULL)
			pw_properties_set(props, PW_KEY_MEDIA_CLASS, "Audio/Sink");

		if (pw_properties_get(props, PW_KEY_NODE_NAME) == NULL)
			pw_properties_setf(props, PW_KEY_NODE_NAME, "vban-sink-%u", id);
		if (pw_properties_get(props, PW_KEY_NODE_DESCRIPTION) == NULL)
			pw_properties_set(props, PW_KEY_NODE_DESCRIPTION,
							  pw_properties_get(props, PW_KEY_NODE_NAME));

		if ((str = pw_properties_get(props, "stream.props")) != NULL)
			pw_properties_update_string(impl->stream_props, str, strlen(str));

	/*struct

		PW_LOG_TOPIC_INIT(mod_topic);

		pw_log_debug("module %p: new %s", impl, args);

		if (pw_properties_get(props, PW_KEY_NODE_GROUP) == NULL)
			pw_properties_set(props, PW_KEY_NODE_GROUP, "pipewire.dummy");
		if (pw_properties_get(props, PW_KEY_NODE_VIRTUAL) == NULL)
			pw_properties_set(props, PW_KEY_NODE_VIRTUAL, "true");

		if (pw_properties_get(props, PW_KEY_MEDIA_CLASS) == NULL)
			pw_properties_set(props, PW_KEY_MEDIA_CLASS, "Audio/Sink");

		if (pw_properties_get(props, PW_KEY_NODE_NAME) == NULL)
			pw_properties_setf(props, PW_KEY_NODE_NAME, "vban-sink-%u", id);
		if (pw_properties_get(props, PW_KEY_NODE_DESCRIPTION) == NULL)
			pw_properties_set(props, PW_KEY_NODE_DESCRIPTION,
							  pw_properties_get(props, PW_KEY_NODE_NAME));

		if ((str = pw_properties_get(props, "stream.props")) != NULL)
			pw_properties_update_string(impl->stream_props, str, strlen(str));

		copy_props(impl, props, PW_KEY_AUDIO_RATE);
		copy_props(impl, props, PW_KEY_AUDIO_CHANNELS);
		copy_props(impl, props, SPA_KEY_AUDIO_POSITION);
		copy_props(impl, props, PW_KEY_NODE_NAME);
		copy_props(impl, props, PW_KEY_NODE_DESCRIPTION);
		copy_props(impl, props, PW_KEY_NODE_GROUP);
		copy_props(impl, props, PW_KEY_NODE_LATENCY);
		copy_props(impl, props, PW_KEY_NODE_VIRTUAL);
		copy_props(impl, props, PW_KEY_MEDIA_CLASS);

		if ((res = parse_audio_info(impl)) < 0) {
			pw_log_error("can't parse audio format");
			goto error;
		}

		impl->core = pw_context_get_object(impl->context, PW_TYPE_INTERFACE_Core);
		if (impl->core == NULL) {
			str = pw_properties_get(props, PW_KEY_REMOTE_NAME);
			impl->core = pw_context_connect(
				impl->context, pw_properties_new(PW_KEY_REMOTE_NAME, str, NULL), 0);
			impl->do_disconnect = true;
		}
		if (impl->core == NULL) {
			res = -errno;
			pw_log_error("can't connect: %m");
			goto error;
		}

		pw_proxy_add_listener((struct pw_proxy*)impl->core,
							  &impl->core_proxy_listener, &core_proxy_events, impl);
		pw_core_add_listener(impl->core, &impl->core_listener, &core_events, impl);

		if ((res = create_stream(impl)) < 0) goto error;

		pw_impl_module_add_listener(module, &impl->module_listener, &module_events,
									impl);

		pw_impl_module_update_properties(module,
										 &SPA_DICT_INIT_ARRAY(module_props));

		char* addr_str = pw_properties_get(props, "vban.ip");
		if (addr_str == NULL) {
			res = -1;
			pw_log_error("no IP provided");
			goto error;
		}

		char* port = pw_properties_get(props, "vban.port");
		if (port == NULL) {
			port = DEFAULT_PORT;
		}

		struct addrinfo* info;
		if (getaddrinfo(addr_str, port, NULL, &info) != 0) {
			res = -1;
			pw_log_error(stderr, "failed to parse address %s", addr_str);
			goto error;
		}

		if ((impl->udp_sock = socket(info->ai_family, info->ai_socktype,
									 info->ai_protocol)) < 0) {
			res = -errno;
			pw_log_error("failed to create UDP socket");
			goto error;
		}

		return 0;

	error:
		impl_destroy(impl);
		return res;*/
	return -1;
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
