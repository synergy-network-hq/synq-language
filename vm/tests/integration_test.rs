use quantumvm::{Assembler, OpCode, QuantumVM};
use pqsynq::{DigitalSignature, KeyEncapsulation, Kem, Sign};

#[test]
fn test_basic_arithmetic() {
    let mut assembler = Assembler::new();
    assembler.emit_op(OpCode::Push);
    assembler.emit_i32(10);
    assembler.emit_op(OpCode::Push);
    assembler.emit_i32(20);
    assembler.emit_op(OpCode::Add);
    assembler.emit_op(OpCode::Halt);

    let bytecode = assembler.build();
    let mut vm = QuantumVM::new();
    vm.load_bytecode(&bytecode).unwrap();
    vm.execute().unwrap();

    let result = vm.stack.pop().unwrap().as_i32().unwrap();
    assert_eq!(result, 30);
}

#[test]
fn test_mldsa_verify() {
    let mut assembler = Assembler::new();
    
    // Generate real keypair and signature using pqsynq
    let signer = Sign::mldsa65();
    let (pk, sk) = signer.keygen().unwrap();
    let message = b"Hello, quantum world!";
    let signature = signer.sign(message, &sk).unwrap();

    // The VM pops arguments as: public_key, message, signature.
    // So we must push: signature, message, public_key.

    // Push signature
    assembler.emit_op(OpCode::LoadImm);
    assembler.emit_bytes(&signature);

    // Push message
    assembler.emit_op(OpCode::LoadImm);
    assembler.emit_bytes(message);

    // Push public key
    assembler.emit_op(OpCode::LoadImm);
    assembler.emit_bytes(&pk);

    // Verify signature
    assembler.emit_op(OpCode::MLDSAVerify);
    assembler.emit_op(OpCode::Halt);

    let bytecode = assembler.build();
    let mut vm = QuantumVM::new();
    vm.load_bytecode(&bytecode).unwrap();
    vm.execute().unwrap();

    let result = vm.stack.pop().unwrap().as_bool().unwrap();
    assert_eq!(result, true);
}

#[test]
fn test_mlkem_decaps() {
    let mut assembler = Assembler::new();
    
    // Generate real keypair and encapsulate using pqsynq
    let kem = Kem::mlkem768();
    let (pk, sk) = kem.keygen().unwrap();
    let (ct, _ss1) = kem.encapsulate(&pk).unwrap();

    // The VM pops arguments as: private_key, ciphertext.
    // So we must push: ciphertext, private_key.

    // Push ciphertext
    assembler.emit_op(OpCode::LoadImm);
    assembler.emit_bytes(&ct);

    // Push private key
    assembler.emit_op(OpCode::LoadImm);
    assembler.emit_bytes(&sk);

    // Decapsulate
    assembler.emit_op(OpCode::MLKEMKeyExchange);
    assembler.emit_op(OpCode::Halt);

    let bytecode = assembler.build();
    let mut vm = QuantumVM::new();
    vm.load_bytecode(&bytecode).unwrap();
    vm.execute().unwrap();

    let shared_secret = vm.stack.pop().unwrap().as_bytes().unwrap().to_vec();
    assert_eq!(shared_secret.len(), 32); // ML-KEM-768 shared secret is 32 bytes
}
