use holium_transformation::types::{
    HoliumPackDataType, HoliumPackPlaceHolder, Io, Package, PackageBytecode, Transformation,
};

/*************************************************************
 * Package bytecode testing
 *************************************************************/

#[test]
fn test_new_package_bytecode() {
    let cid = String::from("cid");
    let bytecode: Vec<u8> = vec![];

    let package_bytecode: PackageBytecode = PackageBytecode::new(cid.clone(), bytecode.clone());

    assert_eq!(cid, package_bytecode.cid);
    assert_eq!(&bytecode, package_bytecode.bytecode());
}

#[test]
fn test_update_package_bytecode() {
    let cid = String::from("cid");
    let bytecode: Vec<u8> = vec![];

    let mut package_bytecode: PackageBytecode = PackageBytecode::new(cid, bytecode);

    let cid = String::from("cid2");
    let bytecode: Vec<u8> = vec![0, 1];

    package_bytecode.update(cid.clone(), bytecode.clone());

    assert_eq!(cid, package_bytecode.cid);
    assert_eq!(&bytecode, package_bytecode.bytecode());
}

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
    assert_eq!(hp_type, io.hp_type);
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
    let bytecode_cid = String::from("cid");
    let bytecode: Vec<u8> = vec![];
    let package_bytecode = PackageBytecode::new(bytecode_cid, bytecode);

    let package = Package::new(
        name.clone(),
        package_bytecode.clone(),
        transformations_vec.clone(),
    );

    assert_eq!(String::new(), package.version);
    assert_eq!(name, package.name);
    assert_eq!(String::new(), package.documentation);
    assert_eq!(&package_bytecode, package.bytecode());
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
    let bytecode_cid = String::from("cid");
    let bytecode: Vec<u8> = vec![];
    let package_bytecode = PackageBytecode::new(bytecode_cid, bytecode);

    let mut package = Package::new(name.clone(), package_bytecode, transformations_vec);

    // Prepare new bytecode & transformation information
    let bytecode_cid = String::from("cid2");
    let bytecode: Vec<u8> = vec![0, 1];
    let package_bytecode = PackageBytecode::new(bytecode_cid, bytecode);

    let transformation_name = String::from("new_name");

    let transformation = Transformation::new(transformation_name, inputs, outputs);
    let transformations_vec = vec![transformation];

    // Update package
    package.update(package_bytecode.clone(), transformations_vec.clone());
    // TODO not checking cid
    assert_eq!(&package_bytecode, package.bytecode());
    assert_eq!(transformations_vec, package.transformations());
}
