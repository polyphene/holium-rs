//! Define and implement the `UpdatableField` trait.

/// The `UpdatableField` trait is useful to get, set, or unset a field of a complex structure such
/// as the one used for Holium parsed configuration files.
pub(crate) trait UpdatableField {
    /// This method is called to get, set or unset a field.
    /// The last value of the field is always returned.
    /// If `opt_value` is not none, it is used, once parsed, to set the field's newest value.
    /// If `unset` is true, then the field is simply set to `None`.
    fn update(&mut self, opt_value: Option<toml::Value>, unset: bool) -> Option<String>;
}

impl UpdatableField for Option<bool> {
    fn update(&mut self, opt_value: Option<toml::Value>, unset: bool) -> Option<String> {
        // set or unset if necessary
        if unset {
            *self = None;
        } else if let Some(new_toml_value) = opt_value {
            *self = new_toml_value.as_bool();
        }
        // return TOML displayable version
        self.and_then(|v| Some(toml::Value::Boolean(v).to_string()))
    }
}

impl UpdatableField for Option<i64> {
    fn update(&mut self, opt_value: Option<toml::Value>, unset: bool) -> Option<String> {
        // set or unset if necessary
        if unset {
            *self = None;
        } else if let Some(new_toml_value) = opt_value {
            *self = new_toml_value.as_integer();
        }
        // return TOML displayable version
        self.and_then(|v| Some(toml::Value::Integer(v).to_string()))
    }
}

