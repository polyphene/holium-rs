use holium_transformation::types::{
    HoliumPackDataType, HoliumPackPlaceHolder, Io, Package, Transformation,
};

/*************************************************************
 * Io testing
 *************************************************************/

#[test]
fn test_new_io() {
    let name = String::from("name");
    let hp_type = HoliumPackDataType::Simple(HoliumPackPlaceHolder::Type0);

    let io = Io::new(name.clone(), hp_type.clone());

    assert_eq!(name, io.name);
    assert_eq!(String::new(), io.documentation);
    assert_eq!(hp_type, io.data_type);
}

/*************************************************************
 * Transformation testing
 *************************************************************/

#[test]
fn test_new_transformation() {
    let name = String::from("name");
    let input_type = HoliumPackDataType::Simple(HoliumPackPlaceHolder::Type0);
    let inputs = vec![Io::new(String::from("input0"), input_type.clone())];
    let output_type = HoliumPackDataType::Simple(HoliumPackPlaceHolder::Type1);
    let outputs = vec![Io::new(String::from("output0"), output_type.clone())];

    let transformation = Transformation::new(name.clone(), inputs.clone(), outputs.clone());

    assert_eq!(name, transformation.name);
    assert_eq!(&inputs, transformation.inputs());
    assert_eq!(&outputs, transformation.outputs());
    assert_eq!(String::new(), transformation.documentation);
}

/*************************************************************
 * Package testing
 *************************************************************/

#[test]
fn test_new_package() {
    // Prepare transformations
    let transformation_name = String::from("name");
    let input_type = HoliumPackDataType::Simple(HoliumPackPlaceHolder::Type0);
    let inputs = vec![Io::new(String::from("input0"), input_type)];
    let output_type = HoliumPackDataType::Simple(HoliumPackPlaceHolder::Type1);
    let outputs = vec![Io::new(String::from("output0"), output_type)];

    let transformation = Transformation::new(transformation_name, inputs, outputs);
    let transformations_vec = vec![transformation];

    // Prepare package metadata
    let name = String::from("name");
    let bytecode: Vec<u8> = vec![];

    let package = Package::new(name.clone(), bytecode.clone(), transformations_vec.clone());

    assert_eq!(String::new(), package.version);
    assert_eq!(name, package.name);
    assert_eq!(String::new(), package.documentation);
    assert_eq!(&bytecode, package.bytecode());
    assert_eq!(&transformations_vec, package.transformations());
}

#[test]
fn test_update_package() {
    // Prepare transformations
    let transformation_name = String::from("name");
    let input_type = HoliumPackDataType::Simple(HoliumPackPlaceHolder::Type0);
    let inputs = vec![Io::new(String::from("input0"), input_type.clone())];
    let output_type = HoliumPackDataType::Simple(HoliumPackPlaceHolder::Type1);
    let outputs = vec![Io::new(String::from("output0"), output_type.clone())];

    let transformation = Transformation::new(transformation_name, inputs.clone(), outputs.clone());
    let transformations_vec = vec![transformation];

    // Prepare package metadata
    let name = String::from("name");
    let bytecode: Vec<u8> = vec![];

    let mut package = Package::new(name.clone(), bytecode, transformations_vec);

    // Prepare new bytecode & transformation information
    let bytecode: Vec<u8> = vec![0, 1];

    let transformation_name = String::from("new_name");

    let transformation = Transformation::new(transformation_name, inputs, outputs);
    let transformations_vec = vec![transformation];

    // Update package
    package.update(bytecode.clone(), transformations_vec.clone());
    // TODO not checking cid
    assert_eq!(&bytecode, package.bytecode());
    assert_eq!(transformations_vec, package.transformations());
}
