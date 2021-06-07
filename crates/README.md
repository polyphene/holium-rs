## Crates sub directory

This directory is meant to contain all secondary crates that could be needed to build our tool.
Crates can be either internal or external depending on their usages.

A sub crate directory should be name by its usage (e.g.: `cli`) and its cargo name should be 
`holium-runtime-<CRATE_DIRECTORY_NAME>`.

A sub crate should have the same layout as our main one, `holium-runtime`