use holium_wasm::set_payload;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyStruct {
    string: String,
    uint: u32,
}

#[no_mangle]
pub extern "C" fn main() {
    let payload = MyStruct {
        string: String::from("hello"),
        uint: 0,
    };
    let res = holium_wasm::set_payload("testA", &payload).unwrap();
}
