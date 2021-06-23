use std::borrow::Borrow;

/// A `PackageBytecode` structure is a Rust representation of a Wasm package bytecode and its CID. A
/// bytecode is the compiled source code of a package containing multiple transformations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageBytecode {
    // TODO might become something like [u8;32] ?
    pub cid: String,
    bytecode: Vec<u8>,
}

// TODO Should we generate CID on a new based on bytecode ?
impl PackageBytecode {
    pub fn new(cid: String, bytecode: Vec<u8>) -> Self {
        PackageBytecode { cid, bytecode }
    }

    /*************************************************************
     * Getter
     *************************************************************/

    pub fn bytecode(&self) -> &[u8] {
        &self.bytecode
    }

    /*************************************************************
     * Setter
     *************************************************************/

    pub fn update(&mut self, cid: String, bytecode: Vec<u8>) -> &mut Self {
        self.cid = cid;
        self.bytecode = bytecode;

        self
    }
}

/// A `Package` structure is a Rust representation of a package in the Holium Framework. A `Package`
/// is mainly composed of a wasm bytecode that contains `Transformation`.
#[derive(Debug, Eq, PartialEq)]
pub struct Package {
    pub version: String,
    pub name: String,
    pub documentation: String,
    bytecode: PackageBytecode,
    transformations: Vec<Transformation>,
}

impl Package {
    pub fn new(
        name: String,
        bytecode: PackageBytecode,
        transformations: Vec<Transformation>,
    ) -> Self {
        Package {
            version: String::new(),
            name,
            documentation: String::new(),
            bytecode,
            transformations,
        }
    }

    /*************************************************************
     * Getter
     *************************************************************/

    pub fn bytecode(&self) -> &PackageBytecode {
        &self.bytecode
    }

    pub fn transformations(&self) -> &[Transformation] {
        &self.transformations
    }

    pub fn transformations_with_input_type(
        &self,
        hp_type: &HoliumPackPlaceHolder,
    ) -> Vec<Transformation> {
        self.transformations
            .clone()
            .into_iter()
            .filter(|t| t.has_input_type(hp_type))
            .collect()
    }

    pub fn transformations_with_output_type(
        &self,
        hp_type: &HoliumPackPlaceHolder,
    ) -> Vec<Transformation> {
        self.transformations
            .clone()
            .into_iter()
            .filter(|t| t.has_output_type(hp_type))
            .collect()
    }

    /*************************************************************
     * Setter
     *************************************************************/

    pub fn tag(&mut self, version: String) {
        self.version = version
    }

    pub fn document(&mut self, documentation: String) {
        self.documentation = documentation
    }

    pub fn update(&mut self, bytecode: PackageBytecode, transformations: Vec<Transformation>) {
        self.bytecode = bytecode;
        self.transformations = transformations
    }

    /*************************************************************
     * Utils
     *************************************************************/

    pub fn has_transformation_with_input_type(&self, hp_type: &HoliumPackPlaceHolder) -> bool {
        self.transformations
            .iter()
            .any(|t| t.has_input_type(hp_type))
    }

    pub fn has_transformation_with_output_type(&self, hp_type: &HoliumPackPlaceHolder) -> bool {
        self.transformations
            .iter()
            .any(|t| t.has_output_type(hp_type))
    }
}

/// A `Transformation` is a wasm function that can be accessed in a `Package` bytecode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transformation {
    pub name: String,
    pub documentation: String,
    inputs: Vec<Io>,
    outputs: Vec<Io>,
}

impl Transformation {
    pub fn new(name: String, inputs: Vec<Io>, outputs: Vec<Io>) -> Self {
        Transformation {
            name,
            documentation: String::new(),
            inputs,
            outputs,
        }
    }

    /*************************************************************
     * Getter
     *************************************************************/

    pub fn inputs(&self) -> &[Io] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[Io] {
        &self.outputs
    }

    pub fn inputs_with_type(&self, hp_type: HoliumPackPlaceHolder) -> Vec<Io> {
        filter_on_io_type(self.inputs.clone(), hp_type)
    }

    pub fn outputs_with_type(&self, hp_type: HoliumPackPlaceHolder) -> Vec<Io> {
        filter_on_io_type(self.outputs.clone(), hp_type)
    }

    /*************************************************************
     * Setter
     *************************************************************/

    pub fn document(&mut self, documentation: String) {
        self.documentation = documentation
    }

    /*************************************************************
     * Utils
     *************************************************************/

    pub fn has_input_type(&self, hp_type: &HoliumPackPlaceHolder) -> bool {
        self.inputs
            .iter()
            .any(|io| std::mem::discriminant(&io.hp_type) == std::mem::discriminant(hp_type))
    }

    pub fn has_output_type(&self, hp_type: &HoliumPackPlaceHolder) -> bool {
        self.outputs
            .iter()
            .any(|io| std::mem::discriminant(&io.hp_type) == std::mem::discriminant(hp_type))
    }
}

/// Io is a structure used to represent the different inputs and outputs that can be found in a transformation.
/// An Io has a `name` that should be human readable and a `hp_type` representing its type in the
/// Holium Pack format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Io {
    pub name: String,
    pub documentation: String,
    pub hp_type: HoliumPackPlaceHolder,
}

impl Io {
    pub fn new(name: String, hp_type: HoliumPackPlaceHolder) -> Self {
        Io {
            name,
            documentation: String::new(),
            hp_type,
        }
    }

    /*************************************************************
     * Setter
     *************************************************************/

    pub fn document(&mut self, documentation: String) {
        self.documentation = documentation
    }
}

// TODO delete when using Holium pack enum
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HoliumPackPlaceHolder {
    Type0,
    Type1,
}

/*************************************************************
 * Utils
 *************************************************************/

fn filter_on_io_type(vector: Vec<Io>, hp_type: HoliumPackPlaceHolder) -> Vec<Io> {
    vector
        .into_iter()
        .filter(|i| std::mem::discriminant(&i.hp_type) == std::mem::discriminant(&hp_type))
        .collect()
}
