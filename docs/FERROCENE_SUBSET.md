# Ferrocene Subset Constraints

This document records the Ferrocene compatibility constraints applied to
`opensomeip-rs-bind`. It is the reference for contributors and for safety
integrators evaluating the crate for use under the Ferrocene qualified
toolchain.

**v1 does not claim Ferrocene qualification or ISO 26262 compliance.** The
crate is structured so that qualification is feasible without an API rewrite.

## Edition

- **Rust 2021** (`--edition 2021`). No unstable features.

## Panic Strategy

- Library crates used with the C ABI are built with `panic = "abort"`.
- The release profile sets `panic = "abort"` and `opt-level = 2` (the
  Ferrocene-recommended optimization level for certified builds).

```toml
[profile.release]
panic = "abort"
opt-level = 2
```

## No Build-Time Code Generation

- **No bindgen, cxx, or autocxx** in the build script. `opensomeip-sys`
  contains hand-written (or vendored/checked-in) `extern "C"` declarations.
- A CI job may optionally diff hand-written bindings against bindgen output,
  but bindgen is never required to build.

## Proc Macros

- **None** in `opensomeip-sys` or in the safety-relevant path of `opensomeip`.
- No `serde` derives on the FFI boundary.
- Convenience derives (e.g., `Debug`, `Clone`) use compiler built-in derives
  only.
- A future `host-macros` feature may enable non-safety proc macros, excluded
  from the Ferrocene profile.

## Standard Library Usage

- `opensomeip-sys` is `#![no_std]`.
- `opensomeip` defaults to `std` (sockets, threads) behind the `std` feature.
- `std`-only APIs are gated so a future `no_std` + certified-core target is
  possible.
- Allowed `std` items (host v1): `std::net`, `std::sync`, `std::io`,
  `std::thread`. This is a **documented deviation** from certified `core`
  subset.

## Unsafe Code

- `unsafe` is confined to `opensomeip-sys` and a thin private module in
  `opensomeip`.
- `#![warn(unsafe_op_in_unsafe_fn)]` is set on all crate roots.
- Every `unsafe` block has a `// SAFETY:` comment.

## Mixed-Language FFI Boundary

- The C ABI (`extern "C"`) is the SM §6.5 qualification boundary.
- No C++ types, exceptions, or unwinding cross the FFI.
- Callbacks from C into Rust use `extern "C"` trampolines that must not
  panic. In the Ferrocene `panic = "abort"` profile, a panic in a callback
  aborts the process. Trampolines document this constraint.

## Forbidden Features

- `proc_macro` crates in the dependency tree of the Ferrocene profile
- `build.rs` that invokes external code generators
- Nightly-only features or `#![feature(...)]`
- `std::panic::catch_unwind` in safety-relevant paths (abort is the policy)

## CI Enforcement

The `ferrocene-subset` CI job builds with:

```
RUSTFLAGS="-C panic=abort" cargo build --workspace --no-default-features
```

It fails if:
- The build requires `std` when `--no-default-features` is set
- Proc-macro dependencies appear in the dependency tree
- `cargo clippy` reports warnings
- `cargo fmt --check` fails
