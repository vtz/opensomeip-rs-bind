# opensomeip-rs-bind

Ferrocene-subset Rust bindings for [OpenSOME/IP](https://github.com/vtz/opensomeip).

## Overview

This repository provides idiomatic Rust bindings to the opensomeip C API,
structured for compatibility with the [Ferrocene](https://ferrocene.dev/)
qualified Rust toolchain.

| Crate | Purpose |
|-------|---------|
| `opensomeip-sys` | Raw `extern "C"` FFI declarations matching `include/capi/opensomeip.h` |
| `opensomeip` | Safe, idiomatic Rust wrapper with `Send`/`Sync` where the C++ stack is thread-safe |

## Ferrocene Subset

v1 is written so an integrator can compile under Ferrocene without rewriting
the API. See [`docs/FERROCENE_SUBSET.md`](docs/FERROCENE_SUBSET.md) for
constraints. CI uses upstream `rustc` with Ferrocene-aligned flags; the
Ferrocene compiler is commercial and not required for development.

**v1 does not claim ISO 26262 or Ferrocene qualification.**

## Building

```bash
# Requires opensomeip C API (BUILD_CAPI=ON) installed or available via submodule
cargo build --workspace
cargo test --workspace
```

## Project Structure

```
crates/
  opensomeip-sys/   # Raw FFI bindings (no_std, hand-written)
  opensomeip/       # Safe API (std by default, no_std optional)
docs/
  FERROCENE_SUBSET.md
```

## License

Apache-2.0. See [LICENSE](LICENSE).
