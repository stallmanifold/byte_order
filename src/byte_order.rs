use std::fmt;
use std::mem;


/// The `ByteOrder` enum records the possible byte orders a machine can have.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

impl fmt::Display for ByteOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt:: Result {
        match *self {
            ByteOrder::BigEndian    => write!(f, "Big Endian"),
            ByteOrder::LittleEndian => write!(f, "Little Endian"),
        }
    }
}

/// The function `byte_order` calculates the system's byte order.
///
/// # Examples
///
/// Calculate a system's byte order:
///
/// ```rust
///
/// use byte_order::{ByteOrder};
///
/// fn main() {
///     let byte_order = byte_order::byte_order();
///
///     println!("This machine has byte order: {}", byte_order);
/// }
/// ```
#[allow(unused_assignments)]
pub fn byte_order() -> ByteOrder {
    let mut arr: [u8; 2] = [0 as u8,0 as u8];
    arr = unsafe { mem::transmute(0xBEEF as u16) };

    if arr[0] == 0xEF {
        return ByteOrder::LittleEndian;
    }

    ByteOrder::BigEndian

}

/// Decides whether a system has big endian byte order.
pub fn is_big_endian() -> bool {
    match byte_order() {
        ByteOrder::BigEndian    => true,
        ByteOrder::LittleEndian => false,
    }
}

/// Decides whether a system has big endian byte order.
pub fn is_little_endian() -> bool {
    match byte_order() {
        ByteOrder::BigEndian    => false,
        ByteOrder::LittleEndian => true,
    }
}
