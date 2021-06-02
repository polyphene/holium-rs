use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyStruct {
    string: String,
    uint: u32,
}

#[no_mangle]
pub extern "C" fn main() {
    let res: String = holium_wasm::get_payload("input").unwrap();

    let payload = MyStruct {
        string: res,
        uint: 0,
    };

    let _res = holium_wasm::set_payload("output", &payload).unwrap();
}
