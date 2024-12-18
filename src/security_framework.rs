use crate::*;

extern crate security_framework;
use security_framework::certificate::SecCertificate;

pub fn security_framework_sec_certificate_list() -> &'static [SecCertificate; DER_LIST_LEN] {
    #[cfg(not(feature="std"))]
    {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref LIST: [SecCertificate; DER_LIST_LEN] = gen_cert_list();
        }

        &*LIST
    }

    #[cfg(feature="std")]
    {
        use once_cell::sync::Lazy;

        static LIST: Lazy<[SecCertificate; DER_LIST_LEN]> = Lazy::new(gen_cert_list);

        &*LIST
    }
}

fn gen_cert_list() -> [SecCertificate; DER_LIST_LEN] {
    core::array::from_fn(|i| {
        SecCertificate::from_der(DER_LIST[i]).unwrap()
    })
}

