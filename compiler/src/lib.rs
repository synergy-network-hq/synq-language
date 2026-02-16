#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod codegen;
pub mod parser;
pub mod pqc_integration;
pub mod semantic;
pub mod solidity_gen;
pub mod version;

pub use codegen::CodeGenerator;
pub use parser::parse;
pub use pqc_integration::PqcIntegration;
pub use semantic::analyze;
pub use solidity_gen::SolidityGenerator;
pub use version::{get_compiler_version, Version, VersionRequirement};
