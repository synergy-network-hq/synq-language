//! # PQSynQ - Post-Quantum Cryptography for SynQ
//!
//! This crate provides a unified interface for Post-Quantum Cryptography (PQC) algorithms
//! specifically designed for use within the SynQ quantum computing framework.
//!
//! ## Supported Algorithms
//!
//! ### Key Encapsulation Mechanisms (KEM)
//! - **ML-KEM** (Module-Lattice-based Key Encapsulation Mechanism) - NIST Standard
//! - **HQC-KEM** (Hamming Quasi-Cyclic Key Encapsulation Mechanism) - NIST Alternative
//! - **CMCE-KEM** (Classic McEliece Key Encapsulation Mechanism) - NIST Alternative
//!
//! ### Digital Signature Schemes
//! - **ML-DSA** (Module-Lattice-based Digital Signature Algorithm) - NIST Standard
//! - **FN-DSA** (FN-DSA Digital Signature Algorithm) - NIST Standard
//! - **SLH-DSA** (Stateless Hash-based Digital Signature Algorithm) - NIST Standard
//!
//! ## Usage
//!
//! ```rust
//! use pqsynq::{DigitalSignature, Kem, KeyEncapsulation, Sign, PqcError};
//! 
//! # fn main() -> Result<(), PqcError> {
//! // KEM operations
//! let kem = Kem::mlkem768();
//! let (pk, sk) = kem.keygen()?;
//! let (ct, ss) = kem.encapsulate(&pk)?;
//! let recovered_ss = kem.decapsulate(&ct, &sk)?;
//! 
//! // Signature operations
//! let signer = Sign::mldsa65();
//! let (pk, sk) = signer.keygen()?;
//! let message: &[u8] = b"hello";
//! let sig = signer.sign(message, &sk)?;
//! let valid = signer.verify(message, &sig, &pk)?;
//! # Ok(()) }
//! ```

#![no_std]
#![allow(clippy::len_without_is_empty)]

// For no-std vectors
extern crate alloc;

// For tests
#[cfg(feature = "std")]
extern crate std;

pub mod error;
pub mod kem;
pub mod sign;
pub mod traits;
pub mod utils;

// Re-export main types
pub use error::PqcError;
pub use kem::{Kem, KemAlgorithm};
pub use sign::{Sign, SignAlgorithm};
pub use traits::{KeyEncapsulation, DigitalSignature};

// Re-export specific algorithms for direct access
#[cfg(feature = "mlkem")]
pub use kem::{Mlkem1024, Mlkem512, Mlkem768};

#[cfg(feature = "mldsa")]
pub use sign::{Mldsa44, Mldsa65, Mldsa87};
#[cfg(feature = "fndsa")]
pub use sign::{Fndsa1024, Fndsa512};

/// Result type for PQC operations
pub type Result<T> = core::result::Result<T, PqcError>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get the version string
pub fn version() -> &'static str {
    VERSION
}
