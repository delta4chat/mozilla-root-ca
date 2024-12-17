use crate::*;

use rustls_pki_types::CertificateDer;

pub const RUSTLS_CERTIFICATE_DER_LIST: [CertificateDer<'static>; DER_LIST_LEN] = {
    let mut list = [const { CertificateDer::from_slice(b"") }; DER_LIST_LEN];

    let mut i = 0;
    while i < DER_LIST_LEN {
        list[i] = CertificateDer::from_slice(DER_LIST[i]);
        i += 1;
    }

    list
};

