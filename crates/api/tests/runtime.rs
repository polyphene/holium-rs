use holium::data::data_tree::Node;
use holium::runtime::Runtime;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn get_file_as_byte_vec(filename: &PathBuf) -> Vec<u8> {
    let mut f = File::open(&filename).unwrap();
    let metadata = std::fs::metadata(&filename).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).unwrap();

    buffer
}

#[test]
fn can_run_wasm_module() {
    // Instantiate runtime & module
    let mut runtime = Runtime::new().unwrap();

    // Module will do a simple addition
    let mut file = std::env::current_dir().unwrap();
    file.push("tests/assets/module.wasm");

    let wasm_bytes = get_file_as_byte_vec(&file);

    runtime.instantiate(&wasm_bytes).unwrap();

    // Prepare payload data
    let cbor_value = serde_cbor::value::to_value(vec![2, 3]).unwrap();
    let data_tree = Node::new(cbor_value);
    let payload_cbor: Vec<u8> = serde_cbor::to_vec(&data_tree).unwrap();

    // Call function
    let res_bytes = runtime.run("main", &payload_cbor).unwrap();

    // Awaited res
    let cbor_value = serde_cbor::value::to_value([5]).unwrap();
    let data_tree = Node::new(cbor_value);
    let awaited_res: Vec<u8> = serde_cbor::to_vec(&data_tree).unwrap();

    assert_eq!(awaited_res, res_bytes);
}
