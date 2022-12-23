project('pipewire-vban', ['c','rust'])

conf_data = configuration_data()
conf_data.set('PACKAGE_VERSION', '"0.0.1"')
configure_file(output: 'config.h', configuration: conf_data)

vban_inc = include_directories()
libvban = static_library('vban', 'libvban/src/lib.rs', rust_crate_type : 'staticlib')
vban_dep = declare_dependency(link_with: libvban, include_directories: vban_inc)

deps = [dependency('libpipewire-0.3'), dependency('libspa-0.2'), vban_dep, meson.get_compiler('c').find_library('dl', required: false), meson.get_compiler('c').find_library('m', required: false), dependency('threads')]
library('module-vban-sink', 'module-vban-sink.c', dependencies: deps, install: true, install_dir: '/usr/lib64/pipewire-0.3')

#executable('receiver', 'receiver/src/main.rs')

executable('playback', 'playback.c', dependencies: deps)
