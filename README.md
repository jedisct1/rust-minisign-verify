# minisign-verify

A small, zero-dependencies Rust crate to verify [Minisign](https://jedisct1.github.io/minisign/) signatures.

[API documentation](https://docs.rs/minisign-verify)

Example:

```rust
let public_key =
    PublicKey::from_base64("RWQf6LRCGA9i53mlYecO4IzT51TGPpvWucNSCh1CBM0QTaLn73Y7GFO3")
        .expect("Unable to decode the public key");

let signature = Signature::decode(
    "untrusted comment: signature from minisign secret key
RWQf6LRCGA9i59SLOFxz6NxvASXDJeRtuZykwQepbDEGt87ig1BNpWaVWuNrm73YiIiJbq71Wi+dP9eKL8OC351vwIasSSbXxwA=
trusted comment: timestamp:1555779966\tfile:test
QtKMXWyYcwdpZAlPF7tE2ENJkRd1ujvKjlj1m9RtHTBnZPa5WKU5uWRs5GoP5M/VqE81QFuMKI5k/SfNQUaOAA==",
    ).expect("Unable to decode the signature");

let bin = b"test";
public_key.verify(&bin[..], &signature, false).expect("Signature didn't verify");
```
