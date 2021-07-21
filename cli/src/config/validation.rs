//! Holds structures used for (de)serialization

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// Configuration structure used for data validation.
pub struct ConfigTemplate {
    core: Option<CoreTemplate>,
}

#[derive(Serialize, Deserialize)]
pub struct CoreTemplate {
    no_scm: Option<bool>,
    no_dvc: Option<bool>,
}
