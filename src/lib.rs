#![no_std]
#![warn(rust_2018_idioms, clippy::pedantic)]

//! libuiohook provides cross-platform keyboard and mouse event hooks from userland.

// TODO(Unavailable): High level API.
//
// The actual design of the API, is _mostly_ straight forward, the only thing
// that I don't have a clear idea how to do is how to automatically provide
// bindings for users.
//
// I think there are two approches for this:
//
// 1. feature gate (as a default feature) the high level API.
//
// A build.rs could compile and link `libuiohook` for the intendent target, and
// error out if it fails (for whatever reason).
//
// If a user wants to link their own libuiohook .so, they can use the
// [package.links] option on their Cargo.toml.
//
// 2. split in two crates: libuiohook-sys and uiohook
//
// This seems to be the approach that the rust team recommends. This just feels
// kinda unfortunate...
//
// TODO(Unavailable): Alternative crate names:
//
// - uiohook
// - easyhook
// - kbmevent
// - kbmhook
// - hookrs
//
// DOCS(Unavailable): Available targets and OSes.
