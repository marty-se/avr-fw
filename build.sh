#!/bin/sh

export RUST_TARGET_PATH=`pwd`

export XARGO_RUST_SRC=/home/marka/avr-rust/rust/src
rustup run avr-toolchain xargo build --target atmega32u4 --release
