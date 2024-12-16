use crate::*;

extern crate x509cert;
use x509cert::X509Certificate;

include!("x509_common.rs");

pub fn x509cert_list() -> &'static Vec<X509Certificate> {
    __x509_common_impl()
}

