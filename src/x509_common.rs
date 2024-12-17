fn __x509_common_impl() -> &'static Vec<X509Certificate> {
    #[cfg(not(feature="std"))]
    {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref LIST: Vec<X509Certificate> = gen_cert_list();
        }

        &*LIST
    }

    #[cfg(feature="std")]
    {
        use once_cell::sync::Lazy;

        static LIST: Lazy<Vec<X509Certificate>> = Lazy::new(gen_cert_list);
        &*LIST
    }
}

fn gen_cert_list() -> Vec<X509Certificate> {
    X509Certificate::from_pem_multiple(PEM_BUNDLE).unwrap()
}
