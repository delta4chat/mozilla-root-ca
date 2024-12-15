use crate::*;

extern crate x509cert;
use x509cert::X509Certificate;

pub fn x509cert_list() -> Vec<X509Certificate> {
    X509Certificate::from_pem_multiple(pem::PEM_BUNDLE).unwrap()
}
