#![cfg_attr(all(not(test), not(feature="std")), no_std)]

#![forbid(unsafe_code)]

#[cfg(not(any(feature="der", feature="pem")))]
compile_error!("it is necessary to enable at least one of the required features (der or pem); otherwise, this crate will be devoid of content.");

#[cfg(feature="der")]
use const_decoder2::decode_base64 as b64;

#[cfg(feature="der")]
pub mod der;
#[cfg(feature="der")]
pub use der::*;
#[cfg(feature="der")]
pub const DER_LIST_LEN: usize = DER_LIST.len();

#[cfg(feature="rustls")]
pub mod rustls;
#[cfg(feature="rustls")]
pub use rustls::*;

#[cfg(feature="pem")]
pub mod pem;
#[cfg(feature="pem")]
pub use pem::*;
#[cfg(feature="pem")]
pub const PEM_LIST_LEN: usize = PEM_LIST.len();

#[cfg(feature="x509cert")]
pub mod x509cert;
#[cfg(feature="x509cert")]
pub use x509cert::*;

#[cfg(feature="x509-certificate")]
pub mod x509_certificate;
#[cfg(feature="x509-certificate")]
pub use x509_certificate::*;

#[cfg(feature="native-tls")]
pub mod native_tls;
#[cfg(feature="native-tls")]
pub use native_tls::*;

#[cfg(test)]
mod test;

