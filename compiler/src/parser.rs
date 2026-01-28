use pest::Parser;
use pest::iterators::Pair;
use crate::ast::*;

#[derive(Parser)]
#[grammar = "synq.pest"]
pub struct SynQParser;

// VersionRequirement is now in version.rs, keeping a simple one here for parser compatibility
#[derive(Debug, Clone, PartialEq)]
pub struct VersionRequirement {
    pub comparator: String,
    pub version: String,
}

pub fn parse(source: &str) -> Result<(Option<VersionRequirement>, Vec<SourceUnit>), pest::error::Error<Rule>> {
    let pairs = SynQParser::parse(Rule::source_file, source)?;
    let mut ast = vec![];
    let mut version_req: Option<VersionRequirement> = None;

    let source_file = pairs.into_iter().next().unwrap();
    for pair in source_file.into_inner() {
        match pair.as_rule() {
            Rule::pragma_version => {
                version_req = Some(parse_version_requirement(pair));
            }
            Rule::item => {
                let item = pair.into_inner().next().unwrap();
                match item.as_rule() {
                    Rule::struct_definition => {
                        ast.push(SourceUnit::Struct(parse_struct(item)));
                    }
                    Rule::contract_definition => {
                        ast.push(SourceUnit::Contract(parse_contract(item)));
                    }
                    _ => unreachable!(),
                }
            }
            Rule::EOI => (),
            _ => {} // Ignore whitespace and comments
        }
    }

    Ok((version_req, ast))
}

fn parse_version_requirement(pair: Pair<Rule>) -> VersionRequirement {
    let mut inner = pair.into_inner();
    
    // Find version_requirement rule
    if let Some(version_req) = inner.find(|p| p.as_rule() == Rule::version_requirement) {
        // Parse first comparator and version (simplified)
        // Full implementation would handle multiple constraints like ">=1.0.0 <2.0.0"
        let mut parts = version_req.into_inner();
        
        let comparator = if let Some(comp_pair) = parts.next() {
            comp_pair.as_str().to_string()
        } else {
            "^".to_string() // Default to caret
        };
        
        let version = if let Some(ver_pair) = parts.next() {
            ver_pair.as_str().to_string()
        } else {
            "1.0.0".to_string()
        };
        
        VersionRequirement { comparator, version }
    } else {
        // Fallback
        VersionRequirement {
            comparator: "^".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

fn parse_struct(pair: Pair<Rule>) -> StructDefinition {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let fields = inner.map(parse_struct_field).collect();
    StructDefinition { name, fields }
}

fn parse_struct_field(pair: Pair<Rule>) -> Parameter {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let ty = parse_type(inner.next().unwrap());
    Parameter { ty, name, is_indexed: false }
}

fn parse_contract(pair: Pair<Rule>) -> ContractDefinition {
    let mut inner = pair.into_inner();
    // Skip annotations for now (would parse them here)
    let name = inner.find(|p| p.as_rule() == Rule::IDENT).unwrap().as_str().to_string();
    let parts: Vec<_> = inner.filter_map(|p| {
        if matches!(p.as_rule(), Rule::state_variable_declaration | Rule::function_definition | Rule::constructor_definition | Rule::event_definition) {
            Some(parse_contract_part(p))
        } else {
            None
        }
    }).collect();
    ContractDefinition { 
        name, 
        parts,
        annotations: vec![], // TODO: Parse annotations
    }
}

fn parse_contract_part(pair: Pair<Rule>) -> ContractPart {
    match pair.as_rule() {
        Rule::state_variable_declaration => {
            ContractPart::StateVariable(parse_state_variable(pair))
        }
        Rule::function_definition => {
            ContractPart::Function(parse_function(pair))
        }
        Rule::constructor_definition => {
            ContractPart::Constructor(parse_constructor(pair))
        }
        Rule::event_definition => {
            ContractPart::Event(parse_event(pair))
        }
        _ => {
            // Fallback - try to parse as function
            ContractPart::Function(parse_function(pair))
        }
    }
}

fn parse_constructor(pair: Pair<Rule>) -> ConstructorDefinition {
    let mut inner = pair.into_inner();
    let params: Vec<_> = inner
        .clone()
        .filter(|p| p.as_rule() == Rule::param)
        .map(parse_param)
        .collect();
    let body = inner.find(|p| p.as_rule() == Rule::block)
        .map(|_| Block { statements: vec![] })
        .unwrap_or_else(|| Block { statements: vec![] });
    ConstructorDefinition {
        params,
        body,
        annotations: vec![], // TODO: Parse annotations
    }
}

fn parse_event(pair: Pair<Rule>) -> EventDefinition {
    let mut inner = pair.into_inner();
    let name = inner.find(|p| p.as_rule() == Rule::IDENT).unwrap().as_str().to_string();
    let params: Vec<_> = inner
        .filter(|p| p.as_rule() == Rule::event_param || p.as_rule() == Rule::param)
        .map(|p| {
            let mut inner = p.into_inner();
            let name = inner.find(|p| p.as_rule() == Rule::IDENT).unwrap().as_str().to_string();
            let ty = parse_type(inner.find(|p| p.as_rule() == Rule::type_decl).unwrap());
            Parameter { ty, name, is_indexed: false }
        })
        .collect();
    EventDefinition {
        name,
        params,
        annotations: vec![], // TODO: Parse annotations
    }
}

fn parse_state_variable(pair: Pair<Rule>) -> StateVariableDeclaration {
    let mut inner = pair.into_inner();
    let name = inner.find(|p| p.as_rule() == Rule::IDENT).unwrap().as_str().to_string();
    let ty = parse_type(inner.find(|p| p.as_rule() == Rule::type_decl).unwrap());
    let is_public = inner.any(|p| p.as_str() == "public");
    StateVariableDeclaration { 
        ty, 
        name, 
        is_public,
        annotations: vec![], // TODO: Parse annotations
    }
}

fn parse_function(pair: Pair<Rule>) -> FunctionDefinition {
    let mut inner = pair.into_inner();
    let name = inner.find(|p| p.as_rule() == Rule::IDENT).unwrap().as_str().to_string();
    let params: Vec<_> = inner
        .clone()
        .filter(|p| p.as_rule() == Rule::param)
        .map(parse_param)
        .collect();
    let _body = inner.find(|p| p.as_rule() == Rule::block); // ignore for now
    FunctionDefinition {
        name,
        params,
        returns: None,
        body: Block { statements: vec![] },
        is_public: false,
        annotations: vec![], // TODO: Parse annotations
    }
}

fn parse_param(pair: Pair<Rule>) -> Parameter {
    println!("Parsing param: {:?}", pair);
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let ty = parse_type(inner.next().unwrap());
    Parameter { ty, name, is_indexed: false }
}


fn parse_type(pair: Pair<Rule>) -> Type {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    match name {
        "Address" => Type::Address,
        "UInt256" => Type::UInt256,
        "Bool" => Type::Bool,
        "Bytes" => Type::Bytes,
        "MLDSAPublicKey" => Type::MLDSAPublicKey,
        "MLDSAKeyPair" => Type::MLDSAKeyPair,
        "MLDSASignature" => Type::MLDSASignature,
        "FNDSAPublicKey" => Type::FNDSAPublicKey,
        "FNDSAKeyPair" => Type::FNDSAKeyPair,
        "FNDSASignature" => Type::FNDSASignature,
        "MLKEMPublicKey" => Type::MLKEMPublicKey,
        "MLKEMKeyPair" => Type::MLKEMKeyPair,
        "MLKEMCiphertext" => Type::MLKEMCiphertext,
        "SLHDSAPublicKey" => Type::SLHDSAPublicKey,
        "SLHDSAKeyPair" => Type::SLHDSAKeyPair,
        "SLHDSASignature" => Type::SLHDSASignature,
        _ => {
            // Unknown type - try to parse as generic or struct
            Type::Struct(name.to_string())
        }
    }
}
