# MobileCoin: Synchronization primitives for SGX enclaves

[![Project Chat][chat-image]][chat-link]<!--
-->![License][license-image]<!--
-->![Architecture: sgx][arch-image]<!--
-->[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]

Synchronization primitives for SGX enclaves.

The available primitives are meant to mimic the behavior
of [std::sync](https://doc.rust-lang.org/std/sync/). Only the primitives
whose behavior can be supported in SGX enclaves are supported.

## Examples

To have code that works with both
[std::sync](https://doc.rust-lang.org/std/sync/) and
[mc-sgx-sync](https://docs.rs/mc-sgx-sync/latest/mc_sgx_sync/).

```rust
#[cfg(feature = "sgx")]
use mc_sgx_sync::Mutex;
#[cfg(not(feature = "sgx"))]
use std::sync::Mutex;

let mutex = Mutex::new(5);

{
    let mut data = lock.lock().unwrap();
    *data += 1;
    assert_eq!(*data, 6);
} // lock is dropped here
```

## Developer Notes

The modules are implemented to mimic the layout of
[std::sync](https://doc.rust-lang.org/std/sync/).

<!-- Direct image link was chosen due to cargo docs not processing mermaid
     sections. Kept the graph in this comment for future maintainers
```mermaid
graph LR
    subgraph mc-sgx-sync
    subgraph Public API
        M(mutex.rs)
        C(condvar.rs)
        R(rwlock.rs)
    end
    subgraph mc-sgx-sync::sys
        M_(mutex.rs)
        C_(condvar.rs)
        R_(rwlock.rs)
    end
    end
    subgraph mc-sgx-tstdc
        M-(mutex.rs)
        C-(condvar.rs)
        R-(rwlock.rs)
    end
    M -> M_ -> M-
    C -> C_ -> C-
    R -> R_ -> R-
```
-->
![module hierarchy](https://mermaid.ink/svg/pako:eNp1ks1qAyEQgF9lmdMWMn0AD4Wy7aHQhWCvwmJ0mixZNfjTJoS8e83akLTEueh8M_Ix6hGU0wQM1l7uNs07F7bJEdKqAKMwrPcYDlb9qyzTahpV87x8K4Vz9K1JkfaPPjxcYdcqZ_WX9H8xb_335NT2Ssnqup2xcAg3puGuaqi4hrqsao0hanVjxLtGrBixbuwbxKc8QVmwwG7OugK7X8jnjBfIERZgyBs56vxgx3OLgLghQwJY3mrptwKEPeW-tNMy0qseo_PAPuUUaAEyRfeR7xJY9IkuTS-jzHObC6T5TF9-xfw5Tj-XYKLq)

- The modules that define the public API are more or less copies from the
  [rust source](https://github.com/rust-lang/rust.git) at commit
  [606c3907](https://github.com/rust-lang/rust/commit/606c3907251397a42e23d3e60de31be9d32525d5)
  with unsupported behavior removed. This ensures that clients can jump back and
  forth between the [std::sync](https://doc.rust-lang.org/std/sync/) types and
  the supported [mc-sgx-sync](https://docs.rs/mc-sgx-sync/latest/mc_sgx_sync/)
  types.

- The `mc-sgx-sync::sys` modules mimic the modules in the
  [rust source](https://github.com/rust-lang/rust.git). The `sys` modules in the
  [rust source](https://github.com/rust-lang/rust.git) are per operating system
  or platform.
  [mc-sgx-sync](https://docs.rs/mc-sgx-sync/latest/mc_sgx_sync/) only supports
  SGX enclaves, but maintaining the `sys` layout reduces modifications to the
  public API modules.

- [mc-sgx-tstdc](https://docs.rs/mc-sgx-tstdc/latest/mc_sgx_tstdc/) provides
  rust wrappers around the C implementation of the synchronization primitives.

[mc-sgx-sync](https://docs.rs/mc-sgx-sync/latest/mc_sgx_sync/) could depend on
[mc-sgx-tstdc-sys](https://docs.rs/mc-sgx-tstdc-sys/latest/mc_sgx_tstdc_sys/)
and call the C implementation directly. This is how many of the `sys` modules in
the [rust source](https://github.com/rust-lang/rust.git) are implemented. The
choice to depend on
[mc-sgx-tstdc](https://docs.rs/mc-sgx-tstdc/latest/mc_sgx_tstdc/) was made to be
consistent with the use of other `mc-sgx-<lib_wrapper>-sys` crates. The
`mc-sgx-<lib_wrapper>` crates provides idiomatic rust interfaces over the C API
and are usually the only crates that directly depend on the
`mc-sgx-<lib_wrapper>-sys` crates.

[chat-image]: https://img.shields.io/discord/844353360348971068?style=flat-square
[chat-link]: https://discord.gg/mobilecoin
[license-image]: https://img.shields.io/crates/l/mc-sgx-sync?style=flat-square
[arch-image]: https://img.shields.io/badge/arch-sgx-red?style=flat-square
[crate-image]: https://img.shields.io/crates/v/mc-sgx-sync.svg?style=flat-square
[crate-link]: https://crates.io/crates/mc-sgx-sync
[docs-image]: https://img.shields.io/docsrs/mc-sgx-sync?style=flat-square
[docs-link]: https://docs.rs/crate/mc-sgx-sync
[deps-image]: https://deps.rs/crate/mc-sgx-sync/0.1.0/status.svg?style=flat-square
[deps-link]: https://deps.rs/crate/mc-sgx-sync/0.1.0
