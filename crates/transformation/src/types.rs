
/// A `Package` structure is a Rust representation of a package in the Holium Framework. A `Package`
/// is mainly composed of a wasm bytecode that contains `Transformation`.
#[derive(Debug, Eq, PartialEq)]
pub struct Package {
    pub version: String,
    pub name: String,
    pub documentation: String,
    pub bytecode: Vec<u8>,
    pub handles: Vec<Transformation>
}

impl Package {
    pub fn new() -> Self {
        Package {
            version: String::new(),
            name: String::new(),
            documentation: String::new(),
            bytecode: vec![],
            handles: vec![]
        }
    }
}

/// A `Transformation` is a wasm function that can be accessed in a `Package` bytecode.
#[derive(Debug, Eq, PartialEq)]
pub struct Transformation {
    pub name: String,
    pub documentation: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>
}

impl Transformation {
    pub fn new() -> Self {
        Transformation {
            name: String::new(),
            documentation: String::new(),
            inputs: vec![],
            outputs: vec![]
        }
    }
}