# SynQ User Manual

**Version 1.0**  
**Complete Guide to Post-Quantum Smart Contract Development**

---

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Language Fundamentals](#language-fundamentals)
4. [Post-Quantum Cryptography](#post-quantum-cryptography)
5. [Gas Model and Resource Management](#gas-model-and-resource-management)
6. [Advanced Features](#advanced-features)
7. [Best Practices](#best-practices)
8. [Complete Examples](#complete-examples)
9. [Reference](#reference)

---

## 1. Introduction

### What is SynQ?

SynQ is a smart contract programming language designed for the post-quantum era. It provides first-class support for quantum-resistant cryptographic primitives, specifically the NIST-standardized algorithms:

- **ML-DSA** (Module-Lattice-Based Digital Signature Algorithm): Digital signature scheme with variants ML-DSA-44, ML-DSA-65, and ML-DSA-87
- **FN-DSA** (FFT over NTRU-Lattice-Based Digital Signature Algorithm): Digital signature scheme with variants FN-DSA-512 and FN-DSA-1024
- **ML-KEM** (Module-Lattice-Based Key-Encapsulation Mechanism): Key Encapsulation Mechanism with variants ML-KEM-512, ML-KEM-768, and ML-KEM-1024
- **SLH-DSA** (Stateless Hash-Based Digital Signature Algorithm): Stateless hash-based signatures with multiple variants

### Why SynQ?

Traditional blockchain cryptography (ECDSA, RSA) is vulnerable to quantum computers. SynQ enables developers to build quantum-resistant smart contracts with:

- **Native PQC Support**: Built-in types and functions for post-quantum cryptography
- **Gas-Aware Design**: Accurate cost modeling for computationally intensive PQC operations
- **Type Safety**: Strong typing prevents cryptographic misuse
- **Developer-Friendly**: Familiar syntax with Solidity-like structure

### Key Features

- ✅ Post-quantum signature verification
- ✅ Key encapsulation mechanisms
- ✅ Gas metering and optimization
- ✅ `require_pqc` blocks for secure verification
- ✅ Gas annotations and budgeting
- ✅ Event system
- ✅ Comprehensive error handling

---

## 2. Getting Started

### Installation

```bash
# Clone the SynQ repository
git clone <repository-url>
cd SynQ

# Build the compiler
cargo build --release

# Build the VM
cd vm
cargo build --release
```

### Your First Contract

Create a file `HelloSynQ.synq`:

```synq
pragma synq ^1.0.0;

contract HelloSynQ {
    Address public owner;
    UInt256 public counter;

    constructor() {
        owner = msg.sender;
        counter = 0;
    }

    function increment() public {
        counter = counter + 1;
    }

    function getCounter() public -> UInt256 {
        return counter;
    }
}
```

### Compiling

> **Note:** The `qsc` CLI tool is planned for future implementation. Currently, compilation is done programmatically via the Rust compiler API.

```bash
# Future CLI syntax (not yet implemented)
qsc compile HelloSynQ.synq
```

**Current Usage:**
The compiler is a Rust library. See the compiler crate documentation for API usage.

### Deploying

```bash
# Future CLI syntax (not yet implemented)
qsc deploy --contract HelloSynQ --gas 1000000
```

**Current Status:**
Deployment tools are planned for future implementation.

---

## 3. Language Fundamentals

### 3.1 Version Pragma

Every SynQ contract should begin with a version pragma to specify the compiler version it requires:

```synq
pragma synq ^1.0.0;
```

**Version Pragma Syntax:**

- `pragma synq ^1.0.0;` - Compatible with version >=1.0.0 and <2.0.0 (recommended)
- `pragma synq >=1.0.0;` - Requires version 1.0.0 or higher
- `pragma synq =1.0.0;` - Requires exactly version 1.0.0
- `pragma synq >=1.0.0 <2.0.0;` - Multiple constraints

**Why Use Version Pragma?**

1. **Compatibility Checking**: Prevents compilation with incompatible compiler versions
2. **Feature Availability**: Ensures required language features are available
3. **Breaking Changes**: Protects against breaking changes in future compiler versions
4. **Documentation**: Makes contract requirements explicit

**Best Practices:**

- Always include a version pragma at the top of your contract
- Use caret (`^`) for flexibility while maintaining compatibility
- Update the pragma when using new language features
- Test with multiple compiler versions when possible

**Example:**

```synq
pragma synq ^1.0.0;

contract MyContract {
    // Contract code here
}
```

### 3.2 Data Types

#### Primitive Types

| Type | Description | Size |
|------|-------------|------|
| `Address` | Ethereum-style address | 20 bytes |
| `UInt256` | Unsigned 256-bit integer | 32 bytes |
| `UInt128` | Unsigned 128-bit integer | 16 bytes |
| `UInt64` | Unsigned 64-bit integer | 8 bytes |
| `UInt32` | Unsigned 32-bit integer | 4 bytes |
| `UInt8` | Unsigned 8-bit integer | 1 byte |
| `Int256` | Signed 256-bit integer | 32 bytes |
| `Int128` | Signed 128-bit integer | 16 bytes |
| `Int64` | Signed 64-bit integer | 8 bytes |
| `Int32` | Signed 32-bit integer | 4 bytes |
| `Int8` | Signed 8-bit integer | 1 byte |
| `Bool` | Boolean value | 1 byte |
| `Bytes` | Variable-length byte array | Dynamic |
| `String` | UTF-8 string | Dynamic |

#### Post-Quantum Types

| Type | Description | Typical Size |
|------|-------------|--------------|
| `MLDSAPublicKey` | ML-DSA public key | ~1,952 bytes (ML-DSA-65/Level 3) |
| `MLDSASignature` | ML-DSA signature | ~3,309 bytes (ML-DSA-65/Level 3) |
| `FNDSAPublicKey` | FN-DSA public key | ~897 bytes (FN-DSA-512/Level 1) |
| `FNDSASignature` | FN-DSA signature | ~666 bytes (FN-DSA-512/Level 1) |
| `MLKEMPublicKey` | ML-KEM public key | ~1,184 bytes (ML-KEM-768/Level 3) |
| `MLKEMCiphertext` | ML-KEM ciphertext | ~1,088 bytes (ML-KEM-768/Level 3) |
| `SLHDSAPublicKey` | SLH-DSA public key | Variable (depends on variant) |
| `SLHDSASignature` | SLH-DSA signature | Variable (depends on variant) |

### 3.3 Variables and Storage

#### State Variables

```synq
contract StorageExample {
    // Public state variable (auto-generates getter)
    Address public owner;
    
    // Private state variable
    UInt256 private balance;
    
    // Mapping
    mapping(Address => UInt256) public balances;
    
    // Array
    Address[] public members;
}
```

#### Local Variables

```synq
function example() {
    let localVar: UInt256 = 100;
    let inferred = 200; // Type inferred
}
```

### 3.4 Functions

#### Function Visibility

```synq
contract VisibilityExample {
    // Public function - can be called externally
    function publicFunction() public {
        // ...
    }
    
    // Internal function - only within contract
    function internalFunction() {
        // ...
    }
}
```

#### Function Parameters and Returns

```synq
function add(UInt256 a, UInt256 b) public -> UInt256 {
    return a + b;
}

function process(Bytes data, Address sender) public -> Bool {
    // Process data
    return true;
}
```

### 3.5 Control Flow

#### If-Else

```synq
if (balance > 100) {
    transfer(recipient, amount);
} else {
    revert("Insufficient balance");
}
```

#### Loops

```synq
// For loop with range (range-based iteration)
for (i in 0..10) {
    process(i);
}

// Traditional C-style for loop
for (i = 0; i < length; i++) {
    process(array[i]);
}

// Note: While loops are planned for future implementation
```

### 3.6 Events

```synq
contract EventExample {
    event Transfer(Address indexed from, Address indexed to, UInt256 value);
    
    function transfer(Address to, UInt256 amount) public {
        // Transfer logic
        emit Transfer(msg.sender, to, amount);
    }
}
```

### 3.7 Error Handling

```synq
function withdraw(UInt256 amount) public {
    require(balance >= amount, "Insufficient balance");
    require(amount > 0, "Amount must be positive");
    
    balance = balance - amount;
    // Transfer logic
}

function criticalOperation() public {
    if (condition) {
        revert("Critical operation failed");
    }
}
```

---

## 4. Post-Quantum Cryptography

### 4.1 Signature Verification

#### ML-DSA Verification

```synq
function verifyMLDSA(
    MLDSAPublicKey publicKey,
    Bytes message,
    MLDSASignature signature
) public -> Bool {
    return verifyMLDSASignature(publicKey, message, signature);
}
```

#### FN-DSA Verification

```synq
function verifyFNDSA(
    FNDSAPublicKey publicKey,
    Bytes message,
    FNDSASignature signature
) public -> Bool {
    return verifyFNDSASignature(publicKey, message, signature);
}
```

### 4.2 Key Encapsulation (ML-KEM)

```synq
function mlkemDecapsulate(
    Bytes privateKey,
    MLKEMCiphertext ciphertext
) public -> Bytes {
    // Returns shared secret
    return mlkemDecapsulate(ciphertext, privateKey);
}
```

### 4.3 require_pqc Blocks

The `require_pqc` block ensures all PQC verifications within it succeed:

```synq
function executeWithPQC(
    MLDSAPublicKey key,
    Bytes message,
    MLDSASignature sig
) public {
    require_pqc {
        let isValid = verifyMLDSASignature(key, message, sig);
        require(isValid, "Invalid signature");
    } or revert("PQC verification failed");
    
    // Continue execution only if all PQC checks passed
    executeAction();
}
```

**Key Points:**
- All PQC operations in the block must succeed
- If any fails, the `or` clause executes (revert or return)
- Gas is tracked separately for PQC operations

---

## 5. Gas Model and Resource Management

### 5.1 Understanding Gas Costs

SynQ uses a sophisticated gas model that accounts for:

1. **Base Cost**: Fixed overhead per operation
2. **Data Cost**: Proportional to input size
3. **Compute Cost**: CPU/memory usage

### 5.2 PQC Operation Costs

| Operation | Base | Data/Byte | Compute | Total (Typical) |
|-----------|------|-----------|---------|-----------------|
| ML-DSA Verify (ML-DSA-65/Level 3) | 6,000 | 9 | 20,000 | ~35,000 |
| FN-DSA Verify (FN-DSA-512/Level 1) | 4,000 | 6 | 10,000 | ~20,000 |
| ML-KEM Decapsulate (ML-KEM-768/Level 3) | 5,000 | 6 | 14,000 | ~25,000 |
| SLH-DSA Verify | 5,000 | 7 | 25,000 | ~40,000 |

### 5.3 Gas Annotations

#### Function-Level Gas Cost

```synq
@gas_cost(base: 45000, mldsa_verify: 35000)
function submitProposal(
    Bytes description,
    MLDSASignature sig
) public {
    // Function with explicit gas cost
}
```

#### Gas Limit Enforcement

> **Note:** `@gas_limit` and `with_gas_limit` are planned features. Currently, gas limits are enforced at the VM level per transaction.

```synq
// Future syntax (not yet implemented)
@gas_limit(100000)
function batchOperation() public {
    // Operation limited to 100,000 gas
}
```

### 5.4 Gas Budgeting

> **Note:** `with_gas_limit` blocks are planned for future implementation. Currently, gas tracking is handled automatically by the VM.

```synq
// Future syntax (not yet implemented)
function complexOperation() public {
    with_gas_limit(200000) {
        // Operations within this block
        // Will revert if gas limit exceeded
        processBatch();
    }
}
```

### 5.5 Gas Optimization Tips

1. **Batch Verifications**: Use batch verification when possible
2. **Minimize Storage**: Store only essential data on-chain
3. **Use Events**: Emit events instead of storing data when possible
4. **Optimize Algorithms**: Choose appropriate PQC algorithm for use case
   - FN-DSA: Smaller signatures, faster verification
   - ML-DSA: More established, larger signatures
   - SLH-DSA: Stateless, largest signatures
   - ML-KEM: For key encapsulation needs

---

## 6. Advanced Features

### 6.1 Structs

```synq
struct User {
    Address addr;
    UInt256 balance;
    Bool active;
}

contract UserManagement {
    mapping(Address => User) public users;
    
    function createUser(Address addr) public {
        users[addr] = User({
            addr: addr,
            balance: 0,
            active: true
        });
    }
}
```

### 6.2 Mappings

```synq
contract MappingExample {
    mapping(Address => UInt256) public balances;
    mapping(UInt256 => Address) public owners;
    mapping(Address => mapping(UInt256 => Bool)) public approvals;
    
    function setBalance(Address addr, UInt256 amount) public {
        balances[addr] = amount;
    }
}
```

### 6.3 Arrays

```synq
contract ArrayExample {
    Address[] public members;
    UInt256[10] public fixedArray;
    
    function addMember(Address member) public {
        members.push(member);
    }
    
    function getMemberCount() public -> UInt256 {
        return members.length;
    }
}
```

### 6.4 Modifiers and Annotations

```synq
@gas_cost(base: 50000, mldsa_verify: 35000)
function expensiveOperation() public {
    // Annotated function with gas cost specification
}

// Note: @optimize_gas and @precompile are planned for future implementation
```

### 6.5 Constructor

```synq
contract Initializable {
    Address public owner;
    UInt256 public initialSupply;
    
    constructor(Address _owner, UInt256 _supply) {
        owner = _owner;
        initialSupply = _supply;
    }
}
```

---

## 7. Best Practices

### 7.1 Security

1. **Always Verify Signatures**: Never trust unverified signatures
2. **Use require_pqc**: For critical operations requiring PQC verification
3. **Check Inputs**: Validate all external inputs
4. **Reentrancy Protection**: Use checks-effects-interactions pattern
5. **Access Control**: Implement proper access control

### 7.2 Gas Optimization

1. **Pack Storage**: Use smaller types when possible
2. **Cache Storage Reads**: Read once, use multiple times
3. **Use Events**: Instead of storage for non-critical data
4. **Batch Operations**: Group operations to reduce overhead

### 7.3 Code Organization

1. **Modular Design**: Break contracts into logical components
2. **Clear Naming**: Use descriptive names
3. **Documentation**: Comment complex logic
4. **Error Messages**: Provide clear error messages

### 7.4 Testing

```synq
// Example test structure
contract TestContract {
    function testVerification() {
        // Setup
        let key = generateKey();
        let message = "test";
        let sig = sign(message, key);
        
        // Execute
        let result = verifyMLDSASignature(key, message, sig);
        
        // Assert
        require(result == true, "Verification should succeed");
    }
}
```

---

## 8. Complete Examples

See the [Complete Examples](#complete-examples) section below for full contract implementations.

---

## 9. Reference

### 9.1 Version Pragma

The version pragma must appear at the top of every SynQ source file before any contract definitions:

```synq
pragma synq ^1.0.0;
```

Supported comparators:
- `^` - Compatible within same major version (recommended)
- `>=` - Greater than or equal
- `<=` - Less than or equal  
- `>` - Greater than
- `<` - Less than
- `=` - Exact match

### 9.2 Built-in Functions

#### Cryptographic Functions

- `verifyMLDSASignature(publicKey, message, signature) -> Bool`
- `verifyFNDSASignature(publicKey, message, signature) -> Bool`
- `verifySLHDSASignature(publicKey, message, signature) -> Bool`
- `mlkemDecapsulate(ciphertext, privateKey) -> Bytes`

#### Global Variables

- `msg.sender`: Address of the transaction sender
- `block.timestamp`: Current block timestamp
- `block.number`: Current block number

### 9.3 Operators

| Operator | Description |
|----------|-------------|
| `+`, `-`, `*`, `/`, `%` | Arithmetic (add, subtract, multiply, divide, modulo) |
| `==`, `!=`, `<`, `<=`, `>`, `>=` | Comparison (equal, not equal, less than, less than or equal, greater than, greater than or equal) |
| `&&`, `\|\|` | Logical (AND, OR) |
| `!` | Logical NOT |
| `++`, `--` | Increment/Decrement (prefix and postfix) |
| `<<`, `>>` | Bit shift (left shift, right shift) |

### 9.4 Error Codes

Common VM errors:
- `OutOfGas`: Insufficient gas
- `StackOverflow`: Stack limit exceeded
- `StackUnderflow`: Empty stack operation
- `InvalidAddress`: Invalid memory/bytecode address
- `CryptoError`: Cryptographic operation failed
- `RuntimeError`: General runtime error

---

## Appendix A: Gas Cost Reference

### Standard Operations

| Operation | Gas Cost |
|-----------|----------|
| Stack Push/Pop | 1 |
| Arithmetic (Add/Sub) | 1 |
| Arithmetic (Mul) | 3 |
| Arithmetic (Div) | 5 |
| Memory Load | 3 |
| Memory Store | 5 |
| Jump | 2 |
| Function Call | 10 |
| Return | 3 |

### PQC Operations

See [Section 5.2](#52-pqc-operation-costs) for detailed PQC gas costs.

---

## Appendix B: Migration Guide

### From Solidity to SynQ

1. **Add Version Pragma**: Add `pragma synq ^1.0.0;` at the top
2. **Replace ECDSA with PQC**: Use ML-DSA, FN-DSA, or SLH-DSA instead
3. **Add Gas Annotations**: Specify gas costs for PQC operations
4. **Use require_pqc**: Replace signature checks with `require_pqc` blocks
5. **Update Types**: Use SynQ-specific types (e.g., `MLDSAPublicKey`, `FNDSAPublicKey`)

---

## Support and Resources

- **Documentation**: See `docs/` directory
- **Examples**: See `examples/` directory
- **Specification**: See `docs/SynQ-Language-Specification.md`
- **VM Specification**: See `docs/SynQ-VM-Specification.md`

---

**SynQ User Manual v1.0**  
*Last Updated: 2024*
