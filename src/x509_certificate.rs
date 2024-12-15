use crate::*;

extern crate x509_certificate;
use x509_certificate::X509Certificate;

pub fn x509_certificate_list() -> Vec<X509Certificate> {
    X509Certificate::from_pem_multiple(pem::PEM_BUNDLE).unwrap()
}
