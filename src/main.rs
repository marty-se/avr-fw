#![feature(lang_items)]
#![no_std]
#![no_main]

mod lufa_bindings;
mod rust_ctypes;

extern crate panic_halt;

#[lang = "eh_personality"] extern fn eh_personality() {}

#[no_mangle]
pub extern "C" fn main() {
    loop {
        unsafe { lufa_bindings::USB_USBTask(); }
    }
}

//fn main() {}