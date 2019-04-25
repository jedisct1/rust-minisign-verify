//! A small crate to verify Minisign signatures.
//!
//! # Example
//!
//! ```rust
//! use minisign_verify::{PublicKey, Signature};
//!
//! let public_key =
//!     PublicKey::from_base64("RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3")
//!    .expect("Unable to decode the public key");
//!
//! let signature = Signature::decode("untrusted comment: signature from minisign secret key
//! RWQf6LRCGA9i59SLOFxz6NxvASXDJeRtuZykwQepbDEGt87ig1BNpWaVWuNrm73YiIiJbq71Wi+dP9eKL8OC351vwIasSSbXxwA=
//! trusted comment: timestamp:1555779966\tfile:test
//! QtKMXWyYcwdpZAlPF7tE2ENJkRd1ujvKjlj1m9RtHTBnZPa5WKU5uWRs5GoP5M/VqE81QFuMKI5k/SfNQUaOAA==")
//!     .expect("Unable to decode the signature");
//!
//! let bin = b"test";
//! public_key.verify(&bin[..], &signature).expect("Signature didn't verify");
//! ```

extern crate base64;
mod crypto;

use crate::crypto::blake2b::{Blake2b, BLAKE2B_OUTBYTES};
use crate::crypto::ed25519;

use std::path::Path;
use std::{fmt, fs, io};
#[derive(Debug)]
pub enum Error {
    InvalidEncoding,
    InvalidSignature,
    IoError(io::Error),
    UnexpectedAlgorithm,
    UnexpectedKeyId,
    UnsupportedAlgorithm,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidEncoding => "Invalid encoding",
            Error::InvalidSignature => "Invalid signature",
            Error::IoError(_) => "I/O error",
            Error::UnexpectedAlgorithm => "Unexpected algorithm",
            Error::UnexpectedKeyId => "Unexpected key identifier",
            Error::UnsupportedAlgorithm => "Unsupported algorithm",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            Error::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<base64::DecodeError> for Error {
    fn from(_e: base64::DecodeError) -> Error {
        Error::InvalidEncoding
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

/// A Minisign public key
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicKey {
    untrusted_comment: Option<String>,
    signature_algorithm: [u8; 2],
    key_id: [u8; 8],
    key: [u8; 32],
}

/// A Minisign signature
#[derive(Clone)]
pub struct Signature {
    untrusted_comment: String,
    signature_algorithm: [u8; 2],
    key_id: [u8; 8],
    signature: [u8; 64],
    trusted_comment: String,
    global_signature: [u8; 64],
}

impl Signature {
    /// Create a Minisign signature from a string
    pub fn decode(lines_str: &str) -> Result<Self, Error> {
        let mut lines = lines_str.lines();
        let untrusted_comment = lines.next().ok_or(Error::InvalidEncoding)?.to_string();
        let bin1 = base64::decode(lines.next().ok_or(Error::InvalidEncoding)?)?;
        if bin1.len() != 74 {
            Err(Error::InvalidEncoding)?
        }
        let trusted_comment = lines.next().ok_or(Error::InvalidEncoding)?.to_string();
        let bin2 = base64::decode(lines.next().ok_or(Error::InvalidEncoding)?)?;
        if bin2.len() != 64 {
            Err(Error::InvalidEncoding)?
        }
        let mut signature_algorithm = [0u8; 2];
        signature_algorithm.copy_from_slice(&bin1[0..2]);
        let mut key_id = [0u8; 8];
        key_id.copy_from_slice(&bin1[2..10]);
        let mut signature = [0u8; 64];
        signature.copy_from_slice(&bin1[10..74]);
        let mut global_signature = [0u8; 64];
        global_signature.copy_from_slice(&bin2);
        Ok(Signature {
            untrusted_comment,
            signature_algorithm,
            key_id,
            signature,
            trusted_comment,
            global_signature,
        })
    }

    /// Load a Minisign signature from a `.sig` file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bin = fs::read_to_string(path)?;
        Signature::decode(&bin)
    }

    /// Return the trusted comment of the signature
    pub fn trusted_comment(&self) -> &str {
        &self.trusted_comment[17..]
    }

    /// Return the untrusted comment of the signature
    pub fn untrusted_comment(&self) -> &str {
        &self.untrusted_comment
    }
}

impl PublicKey {
    /// Create a Minisign public key from a base64 string
    pub fn from_base64(public_key_b64: &str) -> Result<Self, Error> {
        let bin = base64::decode(&public_key_b64)?;
        if bin.len() != 42 {
            Err(Error::InvalidEncoding)?;
        }
        let mut signature_algorithm = [0u8; 2];
        signature_algorithm.copy_from_slice(&bin[0..2]);
        match (signature_algorithm[0], signature_algorithm[1]) {
            (0x45, 0x64) | (0x45, 0x44) => {}
            _ => Err(Error::UnsupportedAlgorithm)?,
        };
        let mut key_id = [0u8; 8];
        key_id.copy_from_slice(&bin[2..10]);
        let mut key = [0u8; 32];
        key.copy_from_slice(&bin[10..42]);
        Ok(PublicKey {
            untrusted_comment: None,
            signature_algorithm,
            key_id,
            key,
        })
    }

    /// Create a Minisign public key from a string, as in the `minisign.pub` file
    pub fn decode(lines_str: &str) -> Result<Self, Error> {
        let mut lines = lines_str.lines();
        let untrusted_comment = lines.next().ok_or(Error::InvalidEncoding)?;
        let public_key_b64 = lines.next().ok_or(Error::InvalidEncoding)?;
        let mut public_key = PublicKey::from_base64(public_key_b64)?;
        public_key.untrusted_comment = Some(untrusted_comment.to_string());
        Ok(public_key)
    }

    /// Load a Minisign key from a file (such as the `minisign.pub` file)
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bin = fs::read_to_string(path)?;
        PublicKey::decode(&bin)
    }

    /// Return the untrusted comment, if there is one
    pub fn untrusted_comment(&self) -> Option<&str> {
        self.untrusted_comment.as_ref().map(String::as_str)
    }

    /// Verify that `signature` is a valid signature for `bin` using this public key
    pub fn verify(&self, bin: &[u8], signature: &Signature) -> Result<(), Error> {
        let prehashed = match (
            signature.signature_algorithm[0],
            signature.signature_algorithm[1],
        ) {
            (0x45, 0x64) => false,
            (0x45, 0x44) => true,
            _ => Err(Error::UnsupportedAlgorithm)?,
        };
        let mut h;
        let bin = if prehashed {
            h = vec![0u8; BLAKE2B_OUTBYTES];
            Blake2b::blake2b(&mut h, bin);
            &h
        } else {
            bin
        };
        if self.key_id != signature.key_id {
            Err(Error::UnexpectedKeyId)?
        }
        if !signature.trusted_comment.starts_with("trusted comment: ") {
            Err(Error::InvalidEncoding)?
        }
        if !ed25519::verify(bin, &self.key, &signature.signature) {
            Err(Error::InvalidSignature)?
        }
        let trusted_comment_bin = signature.trusted_comment().as_bytes();
        let mut global = Vec::with_capacity(signature.signature.len() + trusted_comment_bin.len());
        global.extend_from_slice(&signature.signature[..]);
        global.extend_from_slice(trusted_comment_bin);
        if !ed25519::verify(&global, &self.key, &signature.global_signature) {
            Err(Error::InvalidSignature)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify() {
        let public_key =
            PublicKey::from_base64("RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3")
                .expect("Unable to decode the public key");
        assert_eq!(public_key.untrusted_comment(), None);
        let signature = Signature::decode(
            "untrusted comment: signature from minisign secret key
RWQf6LRCGA9i59SLOFxz6NxvASXDJeRtuZykwQepbDEGt87ig1BNpWaVWuNrm73YiIiJbq71Wi+dP9eKL8OC351vwIasSSbXxwA=
trusted comment: timestamp:1555779966\tfile:test
QtKMXWyYcwdpZAlPF7tE2ENJkRd1ujvKjlj1m9RtHTBnZPa5WKU5uWRs5GoP5M/VqE81QFuMKI5k/SfNQUaOAA==",
        )
        .expect("Unable to decode the signature");
        assert_eq!(
            signature.untrusted_comment(),
            "untrusted comment: signature from minisign secret key"
        );
        assert_eq!(
            signature.trusted_comment(),
            "timestamp:1555779966\tfile:test"
        );
        let bin = b"test";
        public_key
            .verify(&bin[..], &signature)
            .expect("Signature didn't verify");
        let bin = b"Test";
        match public_key.verify(&bin[..], &signature) {
            Err(Error::InvalidSignature) => {}
            _ => panic!("Invalid signature verified"),
        };

        let public_key2 = PublicKey::decode(
            "untrusted comment: minisign public key E7620F1842B4E81F
RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3",
        )
        .expect("Unable to decode the public key");
        assert_eq!(
            public_key2.untrusted_comment(),
            Some("untrusted comment: minisign public key E7620F1842B4E81F")
        );
        match public_key2.verify(&bin[..], &signature) {
            Err(Error::InvalidSignature) => {}
            _ => panic!("Invalid signature verified"),
        };
    }

    #[test]
    fn verify_prehashed() {
        let public_key =
            PublicKey::from_base64("RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3")
                .expect("Unable to decode the public key");
        assert_eq!(public_key.untrusted_comment(), None);
        let signature = Signature::decode(
            "untrusted comment: signature from minisign secret key
RUQf6LRCGA9i559r3g7V1qNyJDApGip8MfqcadIgT9CuhV3EMhHoN1mGTkUidF/z7SrlQgXdy8ofjb7bNJJylDOocrCo8KLzZwo=
trusted comment: timestamp:1556193335\tfile:test
y/rUw2y8/hOUYjZU71eHp/Wo1KZ40fGy2VJEDl34XMJM+TX48Ss/17u3IvIfbVR1FkZZSNCisQbuQY+bHwhEBg==",
        )
        .expect("Unable to decode the signature");
        assert_eq!(
            signature.untrusted_comment(),
            "untrusted comment: signature from minisign secret key"
        );
        assert_eq!(
            signature.trusted_comment(),
            "timestamp:1556193335\tfile:test"
        );
        let bin = b"test";
        public_key
            .verify(&bin[..], &signature)
            .expect("Signature didn't verify");
    }
}
