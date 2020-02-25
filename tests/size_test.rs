use custom_string_serde::custom_string_serde;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::mem::size_of;

custom_string_serde!(10, 60, 100);

#[repr(C, packed)]
pub struct Block {
    x: i32,
    y: i32,
    z: u8,
    name10: CustomString10,
}

fn main() {
    assert_eq!(size_of::<CustomString10>(), 10);
    assert_eq!(size_of::<CustomString60>(), 60);
    assert_eq!(size_of::<CustomString100>(), 100);
    assert_eq!(size_of::<Block>(), 19);
}
