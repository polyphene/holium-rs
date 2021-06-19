use holium_transformation::types::{Transformation, Package, HoliumPackPlaceHolder, Io};

/*************************************************************
 * Io testing
 *************************************************************/

#[test]
fn test_new_io() {
    let name = String::from("name");
    let hp_type = HoliumPackPlaceHolder::Type0;

    let io = Io::new(name.clone(), hp_type.clone());

    assert_eq!(name, io.name);
    assert_eq!(String::new(), io.documentation);
    assert_eq!(hp_type, io.hp_type);
}

#[test]
fn test_document_io() {
    let name = String::from("name");
    let hp_type = HoliumPackPlaceHolder::Type0;

    let mut io = Io::new(name.clone(), hp_type.clone());

    let documentation = String::from("documentation");

    io.document(documentation.clone());

    assert_eq!(documentation, io.documentation);
}

/*************************************************************
 * Transformation testing
 *************************************************************/

#[test]
fn test_new_transformation() {
    let name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type.clone())];
    let output_type = HoliumPackPlaceHolder::Type1;
    let outputs = vec![Io::new(String::from("output0"), output_type.clone())];

    let transformation = Transformation::new(name.clone(), inputs.clone(), outputs.clone());

    assert_eq!(name, transformation.name);
    assert_eq!(inputs, transformation.inputs());
    assert_eq!(outputs, transformation.outputs());
    assert_eq!(String::new(), transformation.documentation);
}

#[test]
fn test_document_transformation() {
    let name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type.clone())];
    let output_type = HoliumPackPlaceHolder::Type1;
    let outputs = vec![Io::new(String::from("output0"), output_type.clone())];

    let mut transformation = Transformation::new(name, inputs, outputs);

    let documentation = String::from("documentation");

    transformation.document(documentation.clone());

    assert_eq!(documentation, transformation.documentation);
}

#[test]
fn test_has_input_type_transformation() {
    let name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type.clone())];
    let output_type = HoliumPackPlaceHolder::Type1;
    let outputs = vec![Io::new(String::from("output0"), output_type.clone())];

    let transformation = Transformation::new(name, inputs, outputs);

    assert_eq!(false, transformation.has_input_type(output_type.clone()));
    assert_eq!(true, transformation.has_input_type(input_type.clone()));
}

#[test]
fn test_has_output_type_transformation() {
    let name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type.clone())];
    let output_type = HoliumPackPlaceHolder::Type1;
    let outputs = vec![Io::new(String::from("output0"), output_type.clone())];

    let transformation = Transformation::new(name, inputs, outputs);

    assert_eq!(false, transformation.has_output_type(input_type.clone()));
    assert_eq!(true, transformation.has_output_type(output_type.clone()));
}

#[test]
fn test_inputs_with_type_transformation() {
    let name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type.clone())];
    let output_type = HoliumPackPlaceHolder::Type1;
    let outputs = vec![Io::new(String::from("output0"), output_type.clone())];

    let transformation = Transformation::new(name, inputs.clone(), outputs);

    let empty_io_vec: Vec<Io> = vec![];
    assert_eq!(empty_io_vec, transformation.inputs_with_type(output_type.clone()));
    assert_eq!(inputs, transformation.inputs_with_type(input_type.clone()));
}

#[test]
fn test_outputs_with_type_transformation() {
    let name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type.clone())];
    let output_type = HoliumPackPlaceHolder::Type1;
    let outputs = vec![Io::new(String::from("output0"), output_type.clone())];

    let transformation = Transformation::new(name, inputs, outputs.clone());

    let empty_io_vec: Vec<Io> = vec![];
    assert_eq!(empty_io_vec, transformation.outputs_with_type(input_type.clone()));
    assert_eq!(outputs, transformation.outputs_with_type(output_type.clone()));
}

/*************************************************************
 * Package testing
 *************************************************************/

#[test]
fn test_new_package() {
    // Prepare transformations
    let transformation_name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type)];
    let output_type = HoliumPackPlaceHolder::Type1;
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
    assert_eq!(transformations_vec, package.handles());
}

#[test]
fn test_tag_package() {
    // Prepare transformations
    let transformation_name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type)];
    let output_type = HoliumPackPlaceHolder::Type1;
    let outputs = vec![Io::new(String::from("output0"), output_type)];

    let transformation = Transformation::new(transformation_name, inputs, outputs);
    let transformations_vec = vec![transformation];

    // Prepare package metadata
    let name = String::from("name");
    let bytecode: Vec<u8> = vec![];

    let mut package = Package::new(name.clone(), bytecode.clone(), transformations_vec.clone());

    let version = String::from("0.1.0");
    package.tag(version.clone());

    assert_eq!(version, package.version);
}

#[test]
fn test_document_package() {
    // Prepare transformations
    let transformation_name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type)];
    let output_type = HoliumPackPlaceHolder::Type1;
    let outputs = vec![Io::new(String::from("output0"), output_type)];

    let transformation = Transformation::new(transformation_name, inputs, outputs);
    let transformations_vec = vec![transformation];

    // Prepare package metadata
    let name = String::from("name");
    let bytecode: Vec<u8> = vec![];

    let mut package = Package::new(name.clone(), bytecode.clone(), transformations_vec.clone());

    let documentation = String::from("documentation");
    package.document(documentation.clone());

    assert_eq!(documentation, package.documentation);
}

#[test]
fn test_update_package() {
    // Prepare transformations
    let transformation_name = String::from("name");
    let input_type = HoliumPackPlaceHolder::Type0;
    let inputs = vec![Io::new(String::from("input0"), input_type.clone())];
    let output_type = HoliumPackPlaceHolder::Type1;
    let outputs = vec![Io::new(String::from("output0"), output_type.clone())];

    let transformation = Transformation::new(transformation_name.clone(), inputs.clone(), outputs.clone());
    let transformations_vec = vec![transformation];

    // Prepare package metadata
    let name = String::from("name");
    let bytecode: Vec<u8> = vec![];

    let mut package = Package::new(name.clone(), bytecode.clone(), transformations_vec.clone());

    // Prepare new bytecode & transformation information
    let bytecode: Vec<u8> = vec![0];
    let transformation_name = String::from("new_name");

    let transformation = Transformation::new(transformation_name.clone(), inputs.clone(), outputs.clone());
    let transformations_vec = vec![transformation];

    // Update package
    package.update(bytecode.clone(), transformations_vec.clone());

    assert_eq!(&bytecode, package.bytecode());
    assert_eq!(transformations_vec, package.handles());
}