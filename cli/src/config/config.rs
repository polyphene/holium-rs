//! Holds structures used for projects complete configurations.

use serde_derive::{Deserialize, Serialize};

use crate::config::sparse_config::SparseConfig;

#[derive(Serialize, Deserialize, Clone, Default)]
/// Structure holding configuration data for a Holium local project.
pub(crate) struct Config {
    pub(crate) core: ConfigCore,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub(crate) struct ConfigCore {
    pub(crate) no_scm: bool,
    pub(crate) no_dvc: bool,
}

impl Config {
    /// The standard configuration holds default values for all configuration fields.
    pub(crate) fn standard() -> Self {
        Config {
            core: ConfigCore {
                no_scm: false,
                no_dvc: false,
            },
        }
    }

    /// Shadow merges a sparse configuration into a complete one.
    /// Current implementation is hard to maintain and may benefit from the use of macros.
    pub(crate) fn merge(&mut self, other: &SparseConfig) {
        // core
        other.core.as_ref().and_then(|core| Some({
            core.no_scm.and_then(|no_scm| {
                Some(self.core.no_scm = no_scm)
            });
            core.no_dvc.and_then(|no_dvc| {
                Some(self.core.no_dvc = no_dvc)
            });
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_merge_config_and_sparse_config() {
        // Prepare original configurations
        let std_config = Config::standard();
        let mut config = std_config.clone();
        let mut sparse_config = SparseConfig::default();
        sparse_config
            .set("core.no_scm", toml::Value::Boolean(!config.core.no_scm))
            .unwrap();
        // Try to merge
        config.merge(&sparse_config);
        // Test
        assert_eq!(config.core.no_scm, !std_config.core.no_scm);
        assert_eq!(config.core.no_dvc, std_config.core.no_dvc);
    }
}
