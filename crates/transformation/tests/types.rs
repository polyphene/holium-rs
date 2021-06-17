use holium_transformation::types::{Transformation, Package};

#[test]
fn new_transformation() {
    let should_be = Transformation {
        name: String::new(),
        documentation: String::new(),
        inputs: vec![],
        outputs: vec![]
    };

    let is = Transformation::new();

    assert_eq!(should_be, is);
}

#[test]
fn new_package() {
    let should_be = Package {
        version: String::new(),
        name: String::new(),
        documentation: String::new(),
        bytecode: vec![],
        handles: vec![]
    };

    let is = Package::new();

    assert_eq!(should_be, is);
}