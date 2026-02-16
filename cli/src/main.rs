use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compiles a SynQ source file
    Compile {
        /// The path to the SynQ source file
        #[arg(short, long)]
        path: PathBuf,
    },
    /// Runs a compiled SynQ bytecode file
    Run {
        /// The path to the SynQ bytecode file
        #[arg(short, long)]
        path: PathBuf,
    },
    /// Verifies deterministic bytecode generation from source
    Verify {
        /// The path to the SynQ source file
        #[arg(short = 's', long)]
        source: PathBuf,
        /// The path to the compiled SynQ bytecode file
        #[arg(short = 'b', long)]
        bytecode: PathBuf,
        /// Execute the verified bytecode after match validation
        #[arg(long, default_value_t = false)]
        run: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Compile { path } => compile(path),
        Commands::Run { path } => run(path),
        Commands::Verify {
            source,
            bytecode,
            run,
        } => verify(source, bytecode, *run),
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}

fn compile(path: &Path) -> Result<(), String> {
    println!("Compiling: {}", path.display());
    let (bytecode, solidity_code) = compile_source(path)?;

    // Generate .synq bytecode
    let (synq_path, sol_path) = derive_output_paths(path);
    fs::write(&synq_path, &bytecode).map_err(|e| format!("Failed to write .synq file: {e}"))?;
    println!("✓ Generated: {}", synq_path.display());

    // Generate .sol Solidity code
    fs::write(&sol_path, &solidity_code).map_err(|e| format!("Failed to write .sol file: {e}"))?;
    println!("✓ Generated: {}", sol_path.display());

    println!("Compilation complete!");
    Ok(())
}

fn run(path: &Path) -> Result<(), String> {
    println!("Running: {}", path.display());
    let bytecode = fs::read(path).map_err(|e| format!("Failed to read bytecode file: {e}"))?;
    execute_bytecode(&bytecode)?;
    println!("Execution finished successfully");
    Ok(())
}

fn verify(source_path: &Path, bytecode_path: &Path, run_after_verify: bool) -> Result<(), String> {
    println!(
        "Verifying bytecode determinism:\n  source: {}\n  bytecode: {}",
        source_path.display(),
        bytecode_path.display()
    );

    let (generated, _solidity) = compile_source(source_path)?;
    let provided = fs::read(bytecode_path).map_err(|e| {
        format!(
            "Failed to read bytecode file {}: {e}",
            bytecode_path.display()
        )
    })?;

    if generated != provided {
        let generated_hash = sha256_hex(&generated);
        let provided_hash = sha256_hex(&provided);
        let diff_index = first_diff_index(&generated, &provided)
            .map(|idx| idx.to_string())
            .unwrap_or_else(|| "length mismatch".to_string());

        return Err(format!(
            "Bytecode mismatch (first difference: {diff_index}). generated_sha256={generated_hash} provided_sha256={provided_hash}"
        ));
    }

    println!(
        "✓ Verification succeeded: source and bytecode match (sha256={})",
        sha256_hex(&provided)
    );

    if run_after_verify {
        execute_bytecode(&provided)?;
        println!("Execution finished successfully");
    }

    Ok(())
}

fn compile_source(path: &Path) -> Result<(Vec<u8>, String), String> {
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read source file {}: {e}", path.display()))?;
    let (_version_req, ast) = compiler::parse(&source)
        .map_err(|e| format!("Failed to parse source file {}: {e}", path.display()))?;
    compiler::analyze(&ast).map_err(|errors| format_semantic_errors(path, &errors))?;

    let codegen = compiler::CodeGenerator::new();
    let bytecode = codegen
        .generate(&ast)
        .map_err(|e| format!("Failed to generate bytecode: {e}"))?;

    let sol_gen = compiler::SolidityGenerator::new();
    let solidity_code = sol_gen
        .generate(&ast)
        .map_err(|e| format!("Failed to generate Solidity code: {e}"))?;

    Ok((bytecode, solidity_code))
}

fn derive_output_paths(path: &Path) -> (PathBuf, PathBuf) {
    let synq_path = match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("synq") => path.with_extension("compiled.synq"),
        _ => path.with_extension("synq"),
    };
    let sol_path = path.with_extension("sol");
    (synq_path, sol_path)
}

fn execute_bytecode(bytecode: &[u8]) -> Result<(), String> {
    let mut vm = quantumvm::QuantumVM::new();
    vm.load_bytecode(&bytecode)
        .map_err(|e| format!("Failed to load bytecode: {e}"))?;
    vm.execute()
        .map_err(|e| format!("VM execution failed: {e}"))?;
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn first_diff_index(a: &[u8], b: &[u8]) -> Option<usize> {
    let limit = a.len().min(b.len());
    for i in 0..limit {
        if a[i] != b[i] {
            return Some(i);
        }
    }
    if a.len() != b.len() {
        return Some(limit);
    }
    None
}

fn format_semantic_errors(path: &Path, errors: &[compiler::ast::SemanticError]) -> String {
    let mut out = format!("Semantic analysis failed for {}:", path.display());
    for (idx, error) in errors.iter().enumerate() {
        let location = match (error.line, error.column) {
            (Some(line), Some(column)) => format!(" [line {line}, col {column}]"),
            (Some(line), None) => format!(" [line {line}]"),
            _ => String::new(),
        };
        out.push_str(&format!("\n  {}. {}{}", idx + 1, error.message, location));
    }
    out
}
