# MobileCoin: Common panic handling behavior for SGX enclaves

[![Project Chat][chat-image]][chat-link]<!--
-->![License][license-image]<!--
-->![Target][target-image]<!--
-->[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]

Panic handler for use in SGX enclaves

The panic handler will redirect to the SGX SDK `abort()` method to mark the
enclave as crashed.

## Features

- `log`: Log panic messages during panic handling. The panic messages will be
directed to the host via
[mc-sgx-io::stderr_write_all](https://docs.rs/mc-sgx-io/latest/mc_sgx_io/fn.stderr_write_all.html).

[chat-image]: https://img.shields.io/discord/844353360348971068?style=flat-square
[chat-link]: https://mobilecoin.chat
[license-image]: https://img.shields.io/crates/l/mc-sgx-panic?style=flat-square
[target-image]: https://img.shields.io/badge/target-sgx-red?style=flat-square
[crate-image]: https://img.shields.io/crates/v/mc-sgx-panic.svg?style=flat-square
[crate-link]: https://crates.io/crates/mc-sgx-panic
[docs-image]: https://img.shields.io/docsrs/mc-sgx-panic?style=flat-square
[docs-link]: https://docs.rs/crate/mc-sgx-panic
[deps-image]: https://deps.rs/crate/mc-sgx-panic/0.1.0/status.svg?style=flat-square
[deps-link]: https://deps.rs/crate/mc-sgx-panic/0.1.0
