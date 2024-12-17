use crate::*;

extern crate x509_certificate;
use x509_certificate::X509Certificate;

include!("x509_common.rs");

pub fn x509_certificate_list() -> &'static [X509Certificate; PEM_LIST_LEN] {
    __x509_common_impl()
}

