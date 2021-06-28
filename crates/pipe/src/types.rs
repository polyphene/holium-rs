use crate::error::PipeError;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Connector {
    // TODO once fixed change the type of cid
    pub cid: String,
    pub outputs_index: Vec<String>,
    pub inputs_index: Vec<String>,
}

impl Connector {
    pub fn new(
        cid: String,
        outputs_index: Vec<String>,
        inputs_index: Vec<String>,
    ) -> Result<Self, PipeError> {
        if outputs_index.len() != inputs_index.len() {
            return Err(PipeError::InvalidMappingError);
        }

        if outputs_index
            .iter()
            .any(|o| !is_correct_index_string(o.as_str()))
            || inputs_index
                .iter()
                .any(|o| !is_correct_index_string(o.as_str()))
        {
            return Err(PipeError::InvalidMappingFormat);
        }

        Ok(Connector {
            cid,
            outputs_index,
            inputs_index,
        })
    }

    /*************************************************************
     * Setter
     *************************************************************/

    pub fn add_mapping(&mut self, output: String, input: String) -> Result<&mut Self, PipeError> {
        if !is_correct_index_string(output.as_str()) || !is_correct_index_string(input.as_str()) {
            return Err(PipeError::InvalidMappingFormat);
        }

        self.outputs_index.push(output);
        self.inputs_index.push(input);

        sort_io_mappings(&mut self.outputs_index, &mut self.inputs_index);

        Ok(self)
    }

    /*************************************************************
     * Utils
     *************************************************************/

    pub fn parse(string: &str) -> Result<Connector, PipeError> {
        let parts: Vec<&str> = string.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(PipeError::InvalidMappingError);
        }

        if !is_correct_mapping_string(parts[1]) || !is_correct_mapping_string(parts[2]) {
            return Err(PipeError::InvalidMappingError);
        }

        let cid = String::from(parts[0]);
        let outputs = parse_connector_mapping(parts[1])?;
        let inputs = parse_connector_mapping(parts[2])?;

        if outputs.len() != inputs.len() {
            return Err(PipeError::InvalidMappingError);
        }

        Ok(Connector::new(cid, outputs, inputs).unwrap())
    }

    pub fn serialize(&mut self) -> Result<String, PipeError> {
        if self.outputs_index.len() != self.inputs_index.len() {
            return Err(PipeError::InvalidMappingError);
        }

        sort_io_mappings(&mut self.outputs_index, &mut self.inputs_index);

        let outputs_string = serialize_connector_mapping(self.outputs_index.clone());
        let inputs_string = serialize_connector_mapping(self.inputs_index.clone());

        Ok(format!("{} {} {}", self.cid, outputs_string, inputs_string))
    }
}

struct Pipe {
    // TODO Should all of our CID be a struct containing utils functions ?
    bytecode_cid: String,
    transformation_handle: String,
    pub connectors: Vec<Connector>,
}

impl Pipe {
    pub fn new(
        bytecode_cid: String,
        transformation_handle: String,
        connectors: Vec<Connector>,
    ) -> Self {
        Pipe {
            bytecode_cid,
            transformation_handle,
            connectors,
        }
    }
}

/*************************************************************
 * Utils
 *************************************************************/
// TODO more explicit error handling
fn parse_connector_mapping(mapping_string: &str) -> Result<Vec<String>, PipeError> {
    let mut mapping_parsed: Vec<String> = vec![];

    let mapping_parts: Vec<&str> = mapping_string.split(',').collect();

    for part in mapping_parts {
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            let dot_count = range[0].matches('.').count();

            if range.len() > 2 || dot_count != range[1].matches('.').count() {
                return Err(PipeError::InvalidMappingError);
            }

            let mut start_range = range[0];
            let mut end_range = range[1];

            let mut prefix: String = String::new();

            if dot_count > 0 {
                let (start_prefix, tmp_start_range) = start_range.rsplit_once('.').unwrap();
                let (end_prefix, tmp_end_range) = end_range.rsplit_once('.').unwrap();

                if start_prefix != end_prefix {
                    return Err(PipeError::InvalidMappingError);
                }

                start_range = tmp_start_range;
                end_range = tmp_end_range;
                prefix = format!("{}.", start_prefix);
            }

            for occurrence in
                start_range.parse::<u32>().unwrap()..end_range.parse::<u32>().unwrap() + 1
            {
                mapping_parsed.push(String::from(format!("{}{}", &prefix, occurrence)));
            }
        } else {
            mapping_parsed.push(String::from(part));
        }
    }

    Ok(mapping_parsed)
}

fn is_correct_mapping_string(string: &str) -> bool {
    let re =
        regex::Regex::new(r"^([0-9]*([.]?[0-9]+)+)([-]([0-9]*([.]?[0-9]+)+))?([,]([0-9]*([.]?[0-9]+)+)([-]([0-9]*([.]?[0-9]+)+))?)*$").unwrap();
    re.is_match(string)
}

fn is_correct_index_string(string: &str) -> bool {
    let re = regex::Regex::new(r"^([0-9]*([.]?[0-9]+)+)$").unwrap();
    re.is_match(string)
}

// TODO more explicit error handling
fn serialize_connector_mapping(mapping_vec: Vec<String>) -> String {
    let mut mapping_serialized = String::new();

    let mut i = 0;
    while i < mapping_vec.len() - 1 {
        if i == mapping_vec.len() - 1 {
            mapping_serialized = format!("{},{}", mapping_serialized, mapping_vec[i]);
            break;
        }

        // Loop through next index mapping to check if we can serialize them in a range
        let mut j = 1;
        while j < mapping_vec.len() - i {
            // For sub indexes, we need to isolate the deepest index and use it as reference
            if mapping_vec[i].contains('.') {
                let (current_prefix, current_occurrence) = mapping_vec[i].rsplit_once('.').unwrap();

                if mapping_vec[i].matches('.').count() == mapping_vec[i + j].matches('.').count() {
                    let (next_prefix, next_occurrence) =
                        mapping_vec[i + j].rsplit_once('.').unwrap();
                    if current_prefix != next_prefix
                        || current_occurrence.parse::<u32>().unwrap() + j as u32
                            != next_occurrence.parse::<u32>().unwrap()
                    {
                        break;
                    }
                } else {
                    break;
                }
            // For basic indexes there is no special logic to implement
            } else {
                if mapping_vec[i + j].matches('.').count() > 0
                    || mapping_vec[i].parse::<u32>().unwrap() + j as u32
                        != mapping_vec[i + j].parse::<u32>().unwrap()
                {
                    break;
                }
            }
            j += 1;
        }

        // If j=1 it is not a range as we broke in the first iteration
        if j == 1 {
            mapping_serialized = format_append(
                &mapping_serialized,
                (&mapping_vec[i], &String::new()),
                !(i == 0),
            );
        } else {
            mapping_serialized = format_append(
                &mapping_serialized,
                (&mapping_vec[i], &mapping_vec[i + j - 1]),
                !(i == 0),
            );
        }

        i += j;
    }

    mapping_serialized
}

fn format_append(root_string: &String, to_append: (&String, &String), has_comma: bool) -> String {
    let mut formatted_string = root_string.clone();

    let (first_string, second_string) = to_append;

    if has_comma {
        formatted_string = format!("{},", formatted_string);
    }

    if second_string.len() > 0 {
        formatted_string = format!("{}{}-{}", formatted_string, first_string, second_string);
    } else {
        formatted_string = format!("{}{}", formatted_string, first_string);
    }

    formatted_string
}

fn sort_io_mappings(outputs_vec: &mut Vec<String>, inputs_vec: &mut Vec<String>) {
    let tmp_outputs_vec = outputs_vec.to_vec();

    let mut tmp_inputs_vec: Vec<String> = vec![];
    tmp_inputs_vec.resize(inputs_vec.len(), String::new());

    let mut sorting_map: HashMap<&String, usize> = HashMap::new();

    for (i, output) in tmp_outputs_vec.iter().enumerate() {
        sorting_map.insert(output, i);
    }

    (*outputs_vec).sort();

    for (i, output) in (*outputs_vec).iter().enumerate() {
        let previous_index = sorting_map.get(output).unwrap();

        tmp_inputs_vec[i] = inputs_vec[*previous_index].clone();
    }

    *inputs_vec = tmp_inputs_vec;
}
