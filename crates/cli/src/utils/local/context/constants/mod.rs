//! Module holding constants related to [ LocalContext ].

/// Separator used in a node typed name (*ie* the `:` in `source:my-source`).
pub static TYPED_NODE_NAME_SEPARATOR: &str = ":";
/// Separator used in a connection ID (*ie* the `→` in `source:my-source→transformation:my-transformation`).
pub static CONNECTION_ID_SEPARATOR: &str = "→";
/// Separator and prefix used to inform on the direction of a portation (*eg* `from:transformation:my-transformation`).
pub static PORTATION_PREFIX_SEPARATOR: &str = ":";
pub static PORTATION_FROM_HOLIUM_PREFIX: &str = "from";
pub static PORTATION_TO_HOLIUM_PREFIX: &str = "to";
