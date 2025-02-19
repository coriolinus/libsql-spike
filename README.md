# `libsql` Spike

We are looking for a unified database solution which

- is async
- builds to wasm
- works via uniffi

[`libsql`] looks like it may meet our criteria, so let's prove that concept one way or the other.

[`libsql`]: https://turso.tech/libsql

## Strategy

1. Build a simple pure-rust library using this DB
2. Build a simple CLI so we can prove it works locally
3. Build a -ffi adaptor crate
4. Add `uniffi` bindings to the ffi adaptor crate.
5. Verify that some other language i.e. python can access the things uniffi exports
6. Add `wasm` bindings to the ffi adaptor crate.
7. Verify that TS in bun can access the things wasm exports.

The FFI stuff is complicated, but we can streamline things by essentially copying config from core-crypto.
The point of all this is to demonstrate the capability on a project with a (much!) smaller surface area
than core-crypto itself.

## Outcome

Steps 1-6 work properly. Additionally, we managed to demonstrate a technique for unifying implementations between wasm and uniffi.

Unfortunately, step 7 is impossible without adjusting `libsql`:

```sh
$ cargo build -p ffi --features wasm --target wasm32-unknown-unknown
   Compiling libsql-ffi v0.5.0
   Compiling libsql-sys v0.8.0
error[E0433]: failed to resolve: could not find `unix` in `os`
   --> ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libsql-sys-0.8.0/src/connection.rs:276:26
    |
276 |             use std::os::unix::ffi::OsStrExt;
    |                          ^^^^ could not find `unix` in `os`
    |
note: found an item that was configured out
   --> ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/os/mod.rs:36:9
    |
36  | pub mod unix {}
    |         ^^^^
note: the item is gated here
   --> ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/os/mod.rs:28:1
    |
28  | / #[cfg(all(
29  | |     doc,
30  | |     any(
31  | |         all(target_arch = "wasm32", not(target_os = "wasi")),
32  | |         all(target_vendor = "fortanix", target_env = "sgx")
33  | |     )
34  | | ))]
    | |___^
note: found an item that was configured out
   --> ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/os/mod.rs:85:9
    |
85  | pub mod unix;
    |         ^^^^
note: the item is gated here
   --> ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/os/mod.rs:84:1
    |
84  | #[cfg(all(not(target_os = "hermit"), any(unix, doc)))]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `as_bytes` found for reference `&OsStr` in the current scope
   --> ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libsql-sys-0.8.0/src/connection.rs:277:73
    |
277 |             let path = std::ffi::CString::new(path.as_ref().as_os_str().as_bytes())
    |                                                                         ^^^^^^^^
    |
help: there is a method `as_encoded_bytes` with a similar name
    |
277 |             let path = std::ffi::CString::new(path.as_ref().as_os_str().as_encoded_bytes())
    |                                                                         ~~~~~~~~~~~~~~~~

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `libsql-sys` (lib) due to 2 previous errors
```
