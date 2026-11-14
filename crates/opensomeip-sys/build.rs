// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! Build script for opensomeip-sys.
//!
//! Locates the opensomeip C library using (in priority order):
//! 1. `OPENSOMEIP_DIR` environment variable (points to install prefix)
//! 2. `pkg-config opensomeip`
//! 3. System default library paths
//!
//! @implements REQ_RUST_001

fn main() {
    println!("cargo:rerun-if-env-changed=OPENSOMEIP_DIR");
    println!("cargo:rerun-if-env-changed=OPENSOMEIP_LIB_DIR");
    println!("cargo:rerun-if-env-changed=OPENSOMEIP_INCLUDE_DIR");

    // Strategy 1: Explicit OPENSOMEIP_DIR env var
    if let Ok(dir) = std::env::var("OPENSOMEIP_DIR") {
        println!("cargo:rustc-link-search=native={dir}/lib");
        link_opensomeip_static();
        println!("cargo:include={dir}/include");
        return;
    }

    // Strategy 1b: Separate lib/include dirs
    if let Ok(lib_dir) = std::env::var("OPENSOMEIP_LIB_DIR") {
        println!("cargo:rustc-link-search=native={lib_dir}");
        link_opensomeip_static();
        if let Ok(inc_dir) = std::env::var("OPENSOMEIP_INCLUDE_DIR") {
            println!("cargo:include={inc_dir}");
        }
        return;
    }

    // Strategy 2: pkg-config
    #[cfg(feature = "pkg-config")]
    {
        if pkg_config::probe_library("opensomeip").is_ok() {
            return;
        }
    }

    // Strategy 3: System default — just emit the link directive.
    // The linker will search default paths and `--as-needed` will
    // skip the library when no symbols are referenced (e.g. when
    // building no_std without the C library installed).
    println!("cargo:rustc-link-lib=opensomeip_capi");
    println!("cargo:rustc-link-lib=opensomeip");
}

/// Emit link directives for static archives plus C++ stdlib.
///
/// The C FFI functions (opensomeip_message_create, etc.) live in
/// `libopensomeip_capi`, which in turn depends on the core
/// `libopensomeip` C++ library.  Static linking avoids visibility
/// issues that arise with the shared library's hidden C++ symbols.
fn link_opensomeip_static() {
    println!("cargo:rustc-link-lib=static=opensomeip_capi");
    println!("cargo:rustc-link-lib=static=opensomeip");

    // The core library is C++; link the C++ standard library so that
    // static archives resolve their runtime symbols.
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    } else if cfg!(target_os = "windows") {
        // MSVC links the C++ runtime automatically.
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
