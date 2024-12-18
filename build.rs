fn main() {
    #[cfg(all(feature="security-framework", not(target_vendor="apple")))]
    println!("cargo:warning=the feature security-framework is enabled, but that is NO-OP, because target_vendor is not apple.");

    #[cfg(all(feature="schannel", not(target_os="windows")))]
    println!("cargo:warning=the feature schannel is enabled, but that is NO-OP, because target_os is not windows.");
}

