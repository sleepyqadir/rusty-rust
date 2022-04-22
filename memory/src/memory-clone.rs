use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn main() {
    println!("Hello, world!");

    let (pointer, length) = get_memory_location();

    let message = get_str_at_location(pointer, length);

    println!(
        "the bytes {} at 0x{:X} stored : {}",
        length, pointer, message
    );
}

fn get_memory_location() -> (usize, usize) {
    let string = "beluga";

    let pointer = string.as_ptr() as usize;

    let length = string.len();

    (pointer, length)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}