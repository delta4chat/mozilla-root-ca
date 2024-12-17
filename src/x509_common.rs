fn __x509_common_impl() -> &'static [X509Certificate; PEM_LIST_LEN] {
    #[cfg(not(feature="std"))]
    {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref LIST: [X509Certificate; PEM_LIST_LEN] = gen_cert_list();
        }

        &*LIST
    }

    #[cfg(feature="std")]
    {
        use once_cell::sync::Lazy;

        static LIST: Lazy<[X509Certificate; PEM_LIST_LEN]> = Lazy::new(gen_cert_list);

        &*LIST
    }
}

fn gen_cert_list() -> [X509Certificate; PEM_LIST_LEN] {
    core::array::from_fn(|i| {
        X509Certificate::from_pem(PEM_LIST[i]).unwrap()
    })
}

