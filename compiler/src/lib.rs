#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod parser;
pub mod codegen;
pub mod solidity_gen;
pub mod pqc_integration;
pub mod version;

pub use codegen::CodeGenerator;
pub use solidity_gen::SolidityGenerator;
pub use parser::parse;
pub use pqc_integration::PqcIntegration;
pub use version::{Version, VersionRequirement, get_compiler_version};
