use crate::*;

extern crate schannel;
use schannel::cert_context::CertContext;

pub fn schannel_cert_context_list() -> &'static [CertContext; PEM_LIST_LEN] {
    #[cfg(not(feature="std"))]
    {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref LIST: [CertContext; PEM_LIST_LEN] = gen_cert_list();
        }

        &*LIST
    }

    #[cfg(feature="std")]
    {
        use once_cell::sync::Lazy;

        static LIST: Lazy<[CertContext; PEM_LIST_LEN]> = Lazy::new(gen_cert_list);

        &*LIST
    }
}

fn gen_cert_list() -> [CertContext; PEM_LIST_LEN] {
    core::array::from_fn(|i| {
        CertContext::from_pem(PEM_LIST[i]).unwrap()
    })
}

