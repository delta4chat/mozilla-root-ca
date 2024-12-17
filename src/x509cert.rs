use crate::*;

extern crate x509cert;
use x509cert::X509Certificate;

include!("x509_common.rs");

pub fn x509cert_list() -> &'static [X509Certificate; PEM_LIST_LEN] {
    __x509_common_impl()
}

