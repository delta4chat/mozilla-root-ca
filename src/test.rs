use crate::*;

use bytes::Bytes;

#[cfg(feature="der")]
#[test]
fn der() {
    for der_cert in DER_LIST {
        eprintln!("{:?}", Bytes::from(*der_cert));
    }
}

#[cfg(feature="pem")]
#[test]
fn pem() {
    for pem_cert in PEM_LIST {
        eprintln!("{pem_cert:?}");
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

#[cfg(feature="rustls")]
#[test]
fn rustls() {
    for cert in RUSTLS_CERTIFICATE_DER_LIST.iter() {
        eprintln!("{cert:?}");
    }
}

#[cfg(feature="native-tls")]
#[test]
fn native_tls() {
    for cert in native_tls_certificate_list() {
        eprintln!("{:?}", cert.to_der());
    }
}

#[cfg(all(feature="security-framework", target_vendor="apple"))]
#[test]
fn security_framework() {
    dbg!(security_framework_sec_certificate_list());
}

#[cfg(all(feature="schannel", target_os="windows"))]
#[test]
fn schannel() {
    dbg!(schannel_cert_context_list());
}

