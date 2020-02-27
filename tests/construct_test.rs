use custom_string_serde::custom_string_serde;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

custom_string_serde!(10);

fn main() {
    let a = CustomString10::new();
    let b = CustomString10::from("");

    assert_eq!(a, b);
    assert_eq!(&*a, "");
}
