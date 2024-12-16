use crate::*;

use bytes::Bytes;

#[test]
fn der() {
    for der_cert in DER_LIST {
        dbg!(Bytes::from(*der_cert));
    }
}

#[cfg(feature="pem")]
#[test]
fn pem() {
    for pem_cert in PEM_LIST {
        dbg!(pem_cert);
    }
}

#[cfg(feature="x509cert")]
#[test]
fn x509cert() {
    dbg!(x509cert_list());
}

#[cfg(feature="x509-certificate")]
#[test]
fn x509_certificate() {
    dbg!(x509_certificate_list());
}
