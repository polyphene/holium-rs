use holium_pipe::error::PipeError;
use holium_pipe::types::Connector;

#[test]
fn test_new_connector() {
    let cid = String::from("cid");
    let outputs: Vec<String> = vec![];
    let inputs: Vec<String> = vec![];

    let connector: Connector = Connector::new(cid.clone(), outputs.clone(), inputs.clone());

    assert_eq!(cid, connector.cid);
    assert_eq!(outputs, connector.outputs);
    assert_eq!(inputs, connector.inputs);
}

#[test]
fn test_parse_from_connector() {
    /*************************************************************
     * Missing part in string
     *************************************************************/

    let missing_part_string = String::from("iamacid 0,1,2");

    let parsed_connector = Connector::parse(missing_part_string);

    assert_eq!(true, parsed_connector.is_err());
    assert_eq!(
        PipeError::InvalidMappingError,
        parsed_connector.err().unwrap()
    );

    /*************************************************************
     * Wrong mapping format
     *************************************************************/

    let wrong_mapping_format_string = String::from("iamacid 0,1,2 0,1,a");

    let parsed_connector = Connector::parse(wrong_mapping_format_string);

    assert_eq!(true, parsed_connector.is_err());
    assert_eq!(
        PipeError::InvalidMappingError,
        parsed_connector.err().unwrap()
    );

    /*************************************************************
     * Different number of outputs / inputs
     *************************************************************/

    let different_io_nbr_string = String::from("iamacid 0,1,2 0,1");

    let parsed_connector = Connector::parse(different_io_nbr_string);

    assert_eq!(true, parsed_connector.is_err());
    assert_eq!(
        PipeError::InvalidMappingError,
        parsed_connector.err().unwrap()
    );

    /*************************************************************
     * Wrong range in mapping
     *************************************************************/

    let wrong_range_string = String::from("iamacid 0,1,2.0.0.0-2.1.0.4 0,1,2-6");

    let parsed_connector = Connector::parse(wrong_range_string);

    assert_eq!(true, parsed_connector.is_err());
    assert_eq!(
        PipeError::InvalidMappingError,
        parsed_connector.err().unwrap()
    );

    /*************************************************************
     * Good behaviour test
     *************************************************************/

    let good_connector_string = String::from("iamacid 0,1,2.0.0.0-2.0.0.4 0,1,2-6");
    let cid = String::from("iamacid");
    let outputs: Vec<String> = vec![
        "0", "1", "2.0.0.0", "2.0.0.1", "2.0.0.2", "2.0.0.3", "2.0.0.4",
    ]
    .into_iter()
    .map(|i| String::from(i))
    .collect();
    let inputs: Vec<String> = vec!["0", "1", "2", "3", "4", "5", "6"]
        .into_iter()
        .map(|i| String::from(i))
        .collect();

    let should_be_connector = Connector::new(cid, outputs, inputs);
    let parsed_connector = Connector::parse(good_connector_string).unwrap();

    assert_eq!(should_be_connector, parsed_connector);
}

#[test]
fn test_serialize_connector() {
    /*************************************************************
     * Different length of outputs / inputs
     *************************************************************/
    let cid = String::from("iamacid");
    let outputs: Vec<String> = vec!["0", "2.0.0.0", "2.0.0.1", "2.0.0.2", "2.0.0.3", "2.0.0.4"]
        .into_iter()
        .map(|i| String::from(i))
        .collect();
    let inputs: Vec<String> = vec!["0", "1", "2", "3", "4", "5", "6"]
        .into_iter()
        .map(|i| String::from(i))
        .collect();

    let mut different_nbr_io_connector = Connector::new(cid, outputs, inputs);

    let serialized_connector = different_nbr_io_connector.serialize();

    assert_eq!(true, serialized_connector.is_err());
    assert_eq!(
        PipeError::SerializationError,
        serialized_connector.err().unwrap()
    );

    /*************************************************************
     * Good behaviour test
     *************************************************************/

    let good_connector_string = String::from("iamacid 0-1,2.0.0.0-2.0.0.4 0,3-6,8-9");

    let mut parsed_connector = Connector::parse(good_connector_string.clone()).unwrap();

    let serialized_string = parsed_connector.serialize().unwrap();

    assert_eq!(good_connector_string, serialized_string);
}

#[test]
fn test_add_mapping_connector() {
    let output_index = String::from("0");
    let input_index = String::from("2.4");

    let connector_string = String::from("iamacid 1,2.0.0.0-2.0.0.4,3 0,3-6,8-9");

    let mut parsed_connector = Connector::parse(connector_string).unwrap();

    parsed_connector.add_mapping(output_index, input_index);

    let should_be_connector =
        Connector::parse(String::from("iamacid 0,1,2.0.0.0-2.0.0.4,3 2.4,0,3-6,8-9")).unwrap();

    assert_eq!(should_be_connector, parsed_connector);
}
