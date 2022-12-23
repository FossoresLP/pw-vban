VBAN module for PipeWire
========================

Crates
------

### arr_macro

This crate contains a macro to initalize more complex array structures without writing out the whole arrays.

### generic_module

This crate contains a generic example of how to write a module in Rust.

### receiver

This crate contains the VBAN receiver module. It's still WIP.

### transmitter

This crate contains the VBAN transmitter module.

It is written completely in Rust and works just like any other PipeWire module.

Patch
-----

The reason that pipewire-rs is added as a submodule is that it currently needs to be patched with `core.patch`.
```sh
patch pipewire-rs/pipewire/src/core_.rs core.patch
```
