fn main() {
    #[cfg(all(feature="security-framework", not(target_vendor="apple")))]
    println!("cargo:warning=the feature security-framework is enabled, but that is NO-OP, because target_vendor is not apple.");
}

