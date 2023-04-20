# MobileCoin: Rust libstd functionality for SGX enclaves

[![Project Chat][chat-image]][chat-link]<!--
-->![License][license-image]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->[![CodeCov Status][codecov-image]][codecov-link]<!--
-->[![GitHub Workflow Status][gha-image]][gha-link]<!--
-->[![Contributor Covenant][conduct-image]][conduct-link]

This repository contains MobileCoin's implementations of functionality that exists in Rust's `libstd`. It is not organized as a single crate, but rather as a collection of crates that provide some required/obvious functionality, in particular the requisite language items

| Crate                    | Purpose                                           |
|--------------------------|---------------------------------------------------|

[chat-image]: https://img.shields.io/discord/844353360348971068?style=flat-square
[chat-link]: https://discord.gg/mobilecoin
[license-image]: https://img.shields.io/crates/l/mc-sgx-alloc?style=flat-square
[deps-image]: https://deps.rs/repo/github/mobilecoinfoundation/sgx-std/status.svg?style=flat-square
[deps-link]: https://deps.rs/repo/github/mobilecoinfoundation/sgx-std
[codecov-image]: https://img.shields.io/codecov/c/github/mobilecoinfoundation/sgx-std/main?style=flat-square
[codecov-link]: https://codecov.io/gh/mobilecoinfoundation/sgx-std
[gha-image]: https://img.shields.io/github/actions/workflow/status/mobilecoinfoundation/sgx-std/ci.yaml?branch=main&style=flat-square
[gha-link]: https://github.com/mobilecoinfoundation/sgx-std/actions/workflows/ci.yaml?query=branch%3Amain
[conduct-link]: CODE_OF_CONDUCT.md
[conduct-image]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=flat-square
