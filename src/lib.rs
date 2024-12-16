#![cfg_attr(not(test), no_std)]

use const_decoder2::decode_base64 as b64;

#[cfg(any(feature="x509cert", feature="x509-certificate"))]
extern crate alloc;
#[cfg(any(feature="x509cert", feature="x509-certificate"))]
pub(crate) use alloc::vec::Vec;

pub mod der;
pub use der::*;

#[cfg(feature="pem")]
pub mod pem;
#[cfg(feature="pem")]
pub use pem::*;

#[cfg(feature="x509cert")]
pub mod x509cert;
#[cfg(feature="x509cert")]
pub use x509cert::*;

#[cfg(feature="x509-certificate")]
pub mod x509_certificate;
#[cfg(feature="x509-certificate")]
pub use x509_certificate::*;

#[cfg(test)]
mod test;

