use crate::*;

extern crate native_tls;
use native_tls::Certificate;

const PEM_LIST_LEN: usize = PEM_LIST.len();

pub fn native_tls_certificate_list() -> &'static [Certificate; PEM_LIST_LEN] {
    #[cfg(not(feature="std"))]
    {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref LIST: [Certificate; PEM_LIST_LEN] = gen_cert_list();
        }

        &*LIST
    }

    #[cfg(feature="std")]
    {
        use once_cell::sync::Lazy;

        static LIST: Lazy<[Certificate; PEM_LIST_LEN]> = Lazy::new(gen_cert_list);
        &*LIST
    }
}

fn gen_cert_list() -> [Certificate; PEM_LIST_LEN] {
    core::array::from_fn(|i| {
        Certificate::from_pem(PEM_LIST[i].as_bytes()).unwrap()
    })
}
