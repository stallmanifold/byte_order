use super::byte_order::{ByteOrder, byte_order};
use std::mem;


#[test]
#[allow(unused_assignments)]
fn test_byte_order_correctly_calculated() {
    let mut arr: [u8; 2] = [0 as u8, 0 as u8];
    arr = unsafe { mem::transmute(0xABCD as u16) };

    if arr[0] == 0xAB as u8 {
        // The system is little endian.
        assert_eq!(ByteOrder::LittleEndian, byte_order());
    } else if arr[0] == 0xCD as u8 {
        // The system is big endian.
        assert_eq!(ByteOrder::BigEndian, byte_order());
    } else {
            // Something went wrong. Automatically Fail.
            assert!(false, "Something went wrong with byte_order.");
    }
}
