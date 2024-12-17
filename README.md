# mozilla-root-ca
Mozilla Root CA list from https://curl.se/ca/cacert.pem

The `webpki-roots` crate does not provide the complete certificate content, merely a subset. Furthermore, the `webpki-root-certs` only provides a rustls-compatible format, not a general-purpose format that supports a multitude of crates. Consequently, this crate has been created to address these deficiencies. Any project requiring the full certificate format from Mozilla's Root Certificate Authority Store may utilize this crate.
