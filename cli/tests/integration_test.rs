use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::{tempdir, NamedTempFile};

#[test]
fn test_compile_and_run() {
    let contract = r#"
        contract MyContract {
            function my_function() {}
        }
    "#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", contract).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("cli"));
    cmd.arg("compile").arg("--path").arg(file.path());

    cmd.assert().success();

    let bytecode_path = file.path().with_extension("synq");
    assert!(bytecode_path.exists());

    let mut run_cmd = Command::new(assert_cmd::cargo::cargo_bin!("cli"));
    run_cmd.arg("run").arg("--path").arg(&bytecode_path);

    run_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Execution finished successfully"));
}

#[test]
fn test_verify_accepts_matching_bytecode_and_executes() {
    let contract = r#"
        contract VerifyContract {
            function noop() {}
        }
    "#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", contract).unwrap();

    let mut compile_cmd = Command::new(assert_cmd::cargo::cargo_bin!("cli"));
    compile_cmd.arg("compile").arg("--path").arg(file.path());
    compile_cmd.assert().success();

    let bytecode_path = file.path().with_extension("synq");
    assert!(bytecode_path.exists());

    let mut verify_cmd = Command::new(assert_cmd::cargo::cargo_bin!("cli"));
    verify_cmd
        .arg("verify")
        .arg("--source")
        .arg(file.path())
        .arg("--bytecode")
        .arg(&bytecode_path)
        .arg("--run");

    verify_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Verification succeeded"))
        .stdout(predicate::str::contains("Execution finished successfully"));
}

#[test]
fn test_verify_rejects_mismatched_bytecode() {
    let contract = r#"
        contract VerifyContract {
            function noop() {}
        }
    "#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", contract).unwrap();

    let mut compile_cmd = Command::new(assert_cmd::cargo::cargo_bin!("cli"));
    compile_cmd.arg("compile").arg("--path").arg(file.path());
    compile_cmd.assert().success();

    let bytecode_path = file.path().with_extension("synq");
    assert!(bytecode_path.exists());

    let mut tampered = fs::read(&bytecode_path).unwrap();
    tampered[0] ^= 0xFF;
    fs::write(&bytecode_path, tampered).unwrap();

    let mut verify_cmd = Command::new(assert_cmd::cargo::cargo_bin!("cli"));
    verify_cmd
        .arg("verify")
        .arg("--source")
        .arg(file.path())
        .arg("--bytecode")
        .arg(&bytecode_path);

    verify_cmd
        .assert()
        .failure()
        .stderr(predicate::str::contains("Bytecode mismatch"));
}

#[test]
fn test_compile_does_not_overwrite_synq_source_extension() {
    let contract = r#"
        contract NoOverwrite {
            function noop() {}
        }
    "#;

    let dir = tempdir().unwrap();
    let source_path: PathBuf = dir.path().join("contract.synq");
    fs::write(&source_path, contract).unwrap();

    let mut compile_cmd = Command::new(assert_cmd::cargo::cargo_bin!("cli"));
    compile_cmd.arg("compile").arg("--path").arg(&source_path);
    compile_cmd.assert().success();

    let compiled_path = source_path.with_extension("compiled.synq");
    assert!(compiled_path.exists());
    assert!(source_path.exists());
    assert_eq!(fs::read_to_string(&source_path).unwrap(), contract);
}

#[test]
fn test_compile_all_documented_examples() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let example_paths = [
        "docs/examples/1-ERC20-Token.synq",
        "docs/examples/2-MultiSig-Wallet.synq",
        "docs/examples/3-DAO-Voting.synq",
        "docs/examples/4-NFT-Contract.synq",
        "docs/examples/5-Escrow-Contract.synq",
        "docs/examples/6-Staking-Contract.synq",
    ];

    for relative_path in example_paths {
        let source_path = repo_root.join(relative_path);
        assert!(
            source_path.exists(),
            "Expected example source at {}",
            source_path.display()
        );

        let mut compile_cmd = Command::new(assert_cmd::cargo::cargo_bin!("cli"));
        compile_cmd.arg("compile").arg("--path").arg(&source_path);
        compile_cmd.assert().success();

        let compiled_path = source_path.with_extension("compiled.synq");
        let solidity_path = source_path.with_extension("sol");
        assert!(
            compiled_path.exists(),
            "Expected compiled artifact at {}",
            compiled_path.display()
        );
        assert!(
            solidity_path.exists(),
            "Expected Solidity artifact at {}",
            solidity_path.display()
        );
    }
}

#[test]
fn test_compile_fails_on_semantic_errors() {
    let invalid_contract = r#"
        contract InvalidSemantic {
            function break_it() {
                undefined_symbol = 42;
            }
        }
    "#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", invalid_contract).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("cli"));
    cmd.arg("compile").arg("--path").arg(file.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Semantic analysis failed"));
}
