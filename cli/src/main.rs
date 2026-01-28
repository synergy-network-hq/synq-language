use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compile { path } => {
            compile(path);
        }
        Commands::Run { path } => {
            run(path);
        }
    }
}

fn compile(path: &PathBuf) {
    println!("Compiling: {}", path.display());
    let source = fs::read_to_string(path).expect("Failed to read source file");
    let (_version_req, ast) = compiler::parse(&source).expect("Failed to parse source file");
    
    // Generate .synq bytecode
    let codegen = compiler::CodeGenerator::new();
    let bytecode = codegen.generate(&ast).expect("Failed to generate bytecode");
    let synq_path = path.with_extension("synq");
    fs::write(&synq_path, &bytecode).expect("Failed to write .synq file");
    println!("✓ Generated: {}", synq_path.display());

    // Generate .sol Solidity code
    let sol_gen = compiler::SolidityGenerator::new();
    let solidity_code = sol_gen.generate(&ast).expect("Failed to generate Solidity code");
    let sol_path = path.with_extension("sol");
    fs::write(&sol_path, &solidity_code).expect("Failed to write .sol file");
    println!("✓ Generated: {}", sol_path.display());
    
    println!("Compilation complete!");
}

fn run(path: &PathBuf) {
    println!("Running: {}", path.display());
    let bytecode = fs::read(path).expect("Failed to read bytecode file");
    let mut vm = quantumvm::QuantumVM::new();
    vm.load_bytecode(&bytecode).expect("Failed to load bytecode");
    vm.execute().expect("VM execution failed");
    println!("Execution finished successfully");
}
