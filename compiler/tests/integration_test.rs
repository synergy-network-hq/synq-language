use compiler::parser;

const SIMPLE_CONTRACT_WITH_FUNCTION_PARAMS: &str = r#"
contract MyContract {
    function my_function(a: UInt256, b: Bool) {
    }
}
"#;

#[test]
fn test_parse_simple_contract_with_function_params() {
    let result = parser::parse(SIMPLE_CONTRACT_WITH_FUNCTION_PARAMS);
    if let Err(e) = &result {
        println!("Parser error: {}", e);
    }
    assert!(result.is_ok());
    let (_version_req, source_units) = result.unwrap();
    assert_eq!(source_units.len(), 1);
}
