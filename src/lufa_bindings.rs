// Not much we can do about clippy lints in bindgen output.
#![allow(clippy::all)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));