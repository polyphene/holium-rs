use holium_pipe::error::PipeError;
use holium_pipe::types::{Connector, Pipe};

/*************************************************************
 * Test Connector
 *************************************************************/

#[test]
fn test_new_connector() {
    /*************************************************************
     * Different length in io
     *************************************************************/

    let cid = String::from("cid");
    let outputs_index: Vec<String> = vec![String::from("0")];
    let inputs_index: Vec<String> = vec![];

    let result_new_connector =
        Connector::new(cid.clone(), outputs_index.clone(), inputs_index.clone());

    let supposed_error = PipeError::InvalidMappingError;

    assert_eq!(true, result_new_connector.is_err());
    assert_eq!(supposed_error, result_new_connector.err().unwrap());

    /*************************************************************
     * Wring index in io
     *************************************************************/

    let cid = String::from("cid");
    let outputs_index: Vec<String> = vec![String::from("0")];
    let inputs_index: Vec<String> = vec![String::from("a")];

    let result_new_connector =
        Connector::new(cid.clone(), outputs_index.clone(), inputs_index.clone());

    let supposed_error = PipeError::InvalidMappingFormat;

    assert_eq!(true, result_new_connector.is_err());
    assert_eq!(supposed_error, result_new_connector.err().unwrap());

    /*************************************************************
     * Good behaviour test
     *************************************************************/

    let cid = String::from("cid");
    let outputs_index: Vec<String> = vec![];
    let inputs_index: Vec<String> = vec![];

    let connector: Connector =
        Connector::new(cid.clone(), outputs_index.clone(), inputs_index.clone()).unwrap();

    assert_eq!(cid, connector.cid);
    assert_eq!(outputs_index, connector.outputs_index);
    assert_eq!(inputs_index, connector.inputs_index);
}

#[test]
fn test_parse_from_connector() {
    /*************************************************************
     * Missing part in string
     *************************************************************/

    let missing_part_string = "iamacid 0,1,2";

    let parsed_connector = Connector::parse(missing_part_string);

    assert_eq!(true, parsed_connector.is_err());
    assert_eq!(
        PipeError::InvalidMappingError,
        parsed_connector.err().unwrap()
    );

    /*************************************************************
     * Wrong mapping format
     *************************************************************/

    let wrong_mapping_format_string = "iamacid 0,1,2 0,1,a";

    let parsed_connector = Connector::parse(wrong_mapping_format_string);

    assert_eq!(true, parsed_connector.is_err());
    assert_eq!(
        PipeError::InvalidMappingError,
        parsed_connector.err().unwrap()
    );

    /*************************************************************
     * Different number of outputs / inputs
     *************************************************************/

    let different_io_nbr_string = "iamacid 0,1,2 0,1";

    let parsed_connector = Connector::parse(different_io_nbr_string);

    assert_eq!(true, parsed_connector.is_err());
    assert_eq!(
        PipeError::InvalidMappingError,
        parsed_connector.err().unwrap()
    );

    /*************************************************************
     * Wrong range in mapping
     *************************************************************/

    let wrong_range_string = "iamacid 0,1,2.0.0.0-2.1.0.4 0,1,2-6";

    let parsed_connector = Connector::parse(wrong_range_string);

    assert_eq!(true, parsed_connector.is_err());
    assert_eq!(
        PipeError::InvalidMappingError,
        parsed_connector.err().unwrap()
    );

    /*************************************************************
     * Good behaviour test
     *************************************************************/

    let good_connector_string = "iamacid 0,1,2.0.0.0-2.0.0.4 0,1,2-6";
    let cid = String::from("iamacid");
    let outputs_index: Vec<String> = vec![
        "0", "1", "2.0.0.0", "2.0.0.1", "2.0.0.2", "2.0.0.3", "2.0.0.4",
    ]
    .into_iter()
    .map(|i| String::from(i))
    .collect();
    let inputs_index: Vec<String> = vec!["0", "1", "2", "3", "4", "5", "6"]
        .into_iter()
        .map(|i| String::from(i))
        .collect();

    let should_be_connector = Connector::new(cid, outputs_index, inputs_index).unwrap();
    let parsed_connector = Connector::parse(good_connector_string).unwrap();

    assert_eq!(should_be_connector, parsed_connector);
}

#[test]
fn test_serialize_connector() {
    /*************************************************************
     * Good behaviour test
     *************************************************************/

    let good_connector_string = "iamacid 0-1,2.0.0.0-2.0.0.4 0,3-6,8-9";

    let mut parsed_connector = Connector::parse(good_connector_string.clone()).unwrap();

    let serialized_string = parsed_connector.serialize().unwrap();

    assert_eq!(good_connector_string, serialized_string);
}

#[test]
fn test_add_mapping_connector() {
    let output_index = String::from("0");
    let input_index = String::from("2.4");

    let connector_string = "iamacid 1,2.0.0.0-2.0.0.4,3 0,3-6,8-9";

    let mut parsed_connector = Connector::parse(connector_string).unwrap();

    let _result_add_mapping = parsed_connector.add_mapping(output_index, input_index);

    let should_be_connector =
        Connector::parse("iamacid 0,1,2.0.0.0-2.0.0.4,3 2.4,0,3-6,8-9").unwrap();

    assert_eq!(should_be_connector, parsed_connector);
}

/*************************************************************
 * Test Pipe
 *************************************************************/

#[test]
fn test_new_pipe() {
    let bytecode_cid = String::from("imacid");
    let transformation_handle = String::from("imahandle");
    let connectors: Vec<Connector> = vec![];

    let pipe = Pipe::new(
        bytecode_cid.clone(),
        transformation_handle.clone(),
        connectors.clone(),
    );

    assert_eq!(bytecode_cid, pipe.bytecode_cid());
    assert_eq!(transformation_handle, pipe.transformation_handle());
    assert_eq!(connectors, pipe.connectors);
}
