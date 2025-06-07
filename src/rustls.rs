use crate::*;

use core::mem;

use rustls_pki_types::{CertificateDer, Der};

pub static RUSTLS_DER_LIST: [Der<'static>; DER_LIST_LEN] = {
    let mut list = [const { Der::from_slice(b"") }; DER_LIST_LEN];

    let mut i = 0;
    while i < DER_LIST_LEN {
        mem::forget(mem::replace(&mut list[i], Der::from_slice(DER_LIST[i])));
        i += 1;
    }

    list
};

pub static RUSTLS_CERTIFICATE_DER_LIST: [CertificateDer<'static>; DER_LIST_LEN] = {
    let mut list = [const { CertificateDer::from_slice(b"") }; DER_LIST_LEN];

    let mut i = 0;
    while i < DER_LIST_LEN {
        mem::forget(mem::replace(&mut list[i], CertificateDer::from_slice(DER_LIST[i])));
        i += 1;
    }

    list
};

#[cfg(feature="rustls-trust-anchor")]
use rustls_pki_types::TrustAnchor;

#[cfg(feature="rustls-trust-anchor")]
pub fn rustls_trust_anchor_list() -> &'static [TrustAnchor<'static>; DER_LIST_LEN] {
    #[cfg(not(feature="std"))]
    {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref LIST: [TrustAnchor<'static>; DER_LIST_LEN] = gen_anchor_list();
        }

        &*LIST
    }

    #[cfg(feature="std")]
    {
        use once_cell::sync::Lazy;

        static LIST: Lazy<[TrustAnchor<'static>; DER_LIST_LEN]> = Lazy::new(gen_anchor_list);

        &*LIST
    }
}

#[cfg(feature="rustls-trust-anchor")]
fn gen_anchor_list() -> [TrustAnchor<'static>; DER_LIST_LEN] {
    core::array::from_fn(|i| {
        webpki::anchor_from_trusted_cert(&RUSTLS_CERTIFICATE_DER_LIST[i]).unwrap()
    })
}

