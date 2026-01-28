# SynQ

SynQ is a domain-specific language (DSL) designed for writing **quantum-resistant smart contracts** using NIST-standardized post-quantum cryptographic (PQC) algorithms such as **ML-DSA** (Module-Lattice-Based Digital Signature Algorithm), **FN-DSA** (FFT over NTRU-Lattice-Based Digital Signature Algorithm), **ML-KEM** (Module-Lattice-Based Key-Encapsulation Mechanism), and **SLH-DSA** (Stateless Hash-Based Digital Signature Algorithm). It enables the development of secure decentralized applications (dApps) that remain resilient in the face of future quantum computing threats.

---

## 🔒 Post-Quantum Cryptography Support

SynQ natively supports:

| Algorithm | Purpose | Type(s) |
|----------|---------|---------|
| ML-DSA | Digital Signatures | `MLDSAKeyPair`, `MLDSASignature` |
| FN-DSA | Compact Signatures | `FNDSAKeyPair`, `FNDSASignature` |
| ML-KEM | Key Encapsulation | `MLKEMKeyPair`, `MLKEMCiphertext` |
| SLH-DSA | Stateless Hash Signatures | `SLHDSAKeyPair`, `SLHDSASignature` |

> Security levels are specified by variant (e.g., ML-DSA-65 for Level 3, ML-KEM-768 for Level 3).

---

## 📦 Features

### ✅ First-Class Cryptographic Types

- Strong type enforcement prevents security mismatches
- Parameterized types by security level
- Composite authentication via `PQAuth`

### ⚙️ Explicit Gas Accounting

- `@gas_cost(base, per_op)` decorator for every PQC operation
- Gas costs based on compute, input size, and key strength
- `@optimize_gas` and batch ops supported

### 🔐 Signature Enforcement

- `require_pqc { ... }` block enforces PQC verifications
- `authenticated_pqc` modifier for secure execution paths

### 🧠 VM Integration

- Uses precompiled contracts for PQC ops
- Tracks PQ-Gas separately from standard gas
- Optional support for hardware acceleration (HSM, TPM, etc.)

---

## 🧰 Core Syntax

### 🔧 Types

```synq
type MLDSAKeyPair
type FNDSASignature
type MLKEMCiphertext
type SLHDSASignature
```

### 🔑 Composite Authentication

```synq
type PQAuth = {
    mldsa_key: MLDSAKeyPair,
    fndsa_key: FNDSAKeyPair,
    backup_key: MLDSAKeyPair
}
```

### 🧪 Signature Verification

```synq
require_pqc {
    verify_mldsa(admin_key, msg, sig);
} or revert("Invalid sig");
```

### 💸 Gas Budgeting

```synq
@gas_cost(base: 75000, mldsa_verify: 35000)
function submit_proposal(...) { ... }
```

---

## 🏛 Example: PQC-Verified DAO

SynQ includes a full-featured DAO contract example with:

- Admin control via ML-DSA-65 (Level 3)
- Voting via encrypted FN-DSA + ML-KEM
- Proposal submission, encrypted vote casting, batched tally
- Governance key rotation with `verify_mldsa`

> See: `Quantum Dao Contract`

---

## ⚙️ Development Tools

### 🛠 CLI Compiler

```bash
qsc compile SynQDAO.synq --target mlkem-768
qsc deploy --contract SynQDAO --gas-overhead 15000
qsc estimate --function cast_vote --args ...
```

### 🧪 Simulation Tools

- `qsc simulate` — test gas use and verify PQ-Gas capping
- `qsc trace` — debug `require_pqc` branches

---

## 🔐 Security Model

- All critical contract paths gated by post-quantum signatures
- No use of classical (ECDSA, Ed25519) keys
- Addresses and contracts use Bech32m encoding
- Gas overuse trapped via VM-level `PQGasTracker`
- Signature domain prefixing (`"VOTE:"`, `"PROPOSAL:"`) is mandatory

---

## 🔮 Future Features

- zk-ML-DSA and zk-ML-KEM proof verification
- Optional PQC signature aggregations
- Module import system (`use pqc::fndsa`)
- Interoperability with classical and quantum-native chains
- Proof-based cold wallet recovery

---

## 📚 Files

| File | Description |
|------|-------------|
| `SynQ-User-Manual.md` | Complete user guide with examples |
| `SynQ-Language-Specification.md` | Core language syntax and types |
| `Gas-Model.md` | Full resource and cost economics |
| `SynQDAO_Example.md` | Reference DAO with full PQC controls |
| `SynQ-VM-Specification.md` | VM runtime architecture and opcodes |
| `Version-Pragma.md` | Version pragma documentation |
| `Examples-Index.md` | Index of all example contracts |

---

## 🤝 Contributing

To contribute:

1. Fork this repo
2. Clone and run `qsc` locally
3. Modify one of the source documents
4. Submit a PR with `[SynQ]` prefix

### 📜 Coding Guidelines

- All PQC types follow NIST naming standards (ML-DSA, FN-DSA, ML-KEM, SLH-DSA)
- Signature and encryption messages must be ABI-encoded and prefixed
- All public functions must declare `@gas_cost`

---

## 👨‍🚀 Maintainers

SynQ is maintained by the Synergy Network Core R&D team.

---

## 🧠 License

SynQ is released under the MIT License.

---

End of README.md
