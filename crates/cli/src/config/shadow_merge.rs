//! Defines the `SparseShadowMerge` trait for sparse structures.
//! It is useful for the merging of structures linked to sparse configurations.

use crate::config::sparse_config::{SparseConfig, SparseConfigCore};

/// Provides the `SparseShadowMerge` trait for sparse objects that can be merged in shadowing order.
/// A derive macro could easily generate implementation code for structures.
pub(crate) trait SparseShadowMerge {
    fn shadow_merge(&mut self, other: Self);
}

impl<T> SparseShadowMerge for Option<T> {
    fn shadow_merge(&mut self, other: Self) {
        if other.is_some() {
            *self = other;
        }
    }
}

impl SparseShadowMerge for SparseConfig {
    fn shadow_merge(&mut self, other: Self) {
        self.core.shadow_merge(other.core);
    }
}

impl SparseShadowMerge for SparseConfigCore {
    fn shadow_merge(&mut self, other: Self) {
        self.no_scm.shadow_merge(other.no_scm);
        self.no_dvc.shadow_merge(other.no_dvc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Utility function used to scaffold tests on the `core.no_scm` field of a sparse configuration.
    fn test_merge_sparse_config(
        shadowed: Option<bool>,
        shadowing: Option<bool>,
        expected: Option<bool>,
    ) {
        // Prepare original configurations
        let mut config = SparseConfig::default();
        if shadowed.is_some() {
            let mut core = SparseConfigCore::default();
            core.no_scm = shadowed;
            config.core = Some(core);
        }
        let mut shadowing_config = SparseConfig::default();
        if shadowing.is_some() {
            let mut core = SparseConfigCore::default();
            core.no_scm = shadowing;
            shadowing_config.core = Some(core);
        }
        // Merge configurations
        config.shadow_merge(shadowing_config);
        // Test
        if config.core.is_none() {
            assert!(expected.is_none())
        } else {
            assert_eq!(config.core.unwrap().no_scm, expected);
        }
    }

    #[test]
    fn can_merge_sparse_configs_none_none() {
        test_merge_sparse_config(None, None, None)
    }

    #[test]
    fn can_merge_sparse_configs_none_true() {
        test_merge_sparse_config(None, Some(true), Some(true))
    }

    #[test]
    fn can_merge_sparse_configs_true_none() {
        test_merge_sparse_config(Some(true), None, Some(true))
    }

    #[test]
    fn can_merge_sparse_configs_true_false() {
        test_merge_sparse_config(Some(true), Some(false), Some(false))
    }

    #[test]
    fn can_merge_sparse_configs_false_true() {
        test_merge_sparse_config(Some(false), Some(true), Some(true))
    }
}
