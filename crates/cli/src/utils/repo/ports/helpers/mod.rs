use anyhow::Result;
use crate::utils::repo::models::portation::Portation;
use crate::utils::local::context::helpers::{NodeType, PortationDirectionType, build_portation_id, build_node_typed_name};
use crate::utils::repo::context::RepositoryContext;
use crate::utils::errors::Error::NoObjectForGivenKey;

/// Check from a repository context object if a specific portation object can be found.
pub fn get_portation(
    repo_context: &RepositoryContext,
    node_type: &NodeType,
    node_name: &str,
    direction: &PortationDirectionType,
) -> Option<Portation> {
    let id = build_portation_id(direction, build_node_typed_name(node_type, node_name).as_str());
    repo_context.portations
        .get(&id.to_string())
        .map(|portation| portation.clone())
}