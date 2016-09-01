/*!
This crate is a library that provides methods for determining the byte order of the system a rust
program is running on.
*/
#![crate_name="byte_order"]
pub use byte_order::ByteOrder;
pub use byte_order::{byte_order, is_big_endian, is_little_endian};

pub mod byte_order;

#[cfg(test)]
mod tests;
