use std::fmt;
use std::mem;

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


#[allow(unused_assignments)]
pub fn byte_order() -> ByteOrder {
    let mut arr: [u8; 2] = [0 as u8,0 as u8];
    arr = unsafe { mem::transmute(0xBEEF as u16) };

    if arr[0] == 0xEF {
        return ByteOrder::LittleEndian;
    }

    ByteOrder::BigEndian

}


pub fn is_big_endian() -> bool {
    match byte_order() {
        ByteOrder::BigEndian    => true,
        ByteOrder::LittleEndian => false,
    }
}


pub fn is_little_endian() -> bool {
    match byte_order() {
        ByteOrder::BigEndian    => false,
        ByteOrder::LittleEndian => true,
    }
}

