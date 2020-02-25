use custom_string_serde::custom_string_serde;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

custom_string_serde!(10, 60, 100);

pub struct Block {
    x: i32,
    y: i32,
    name10: CustomString10,
    _name60: CustomString60,
    _name100: CustomString100,
}

fn main() {
    let block = Block {
        x: 10,
        y: 20,
        name10: CustomString10 { bytes: [0; 10] },
        _name60: CustomString60 { bytes: [0; 60] },
        _name100: CustomString100 { bytes: [0; 100] },
    };

    assert_eq!(block.x, 10);
    assert_eq!(block.y, 20);
    assert_eq!(block.name10.bytes, [0; 10]);
}
