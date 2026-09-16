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
        println!("cargo:rustc-link-lib=opensomeip");
        println!("cargo:include={dir}/include");
        return;
    }

    // Strategy 1b: Separate lib/include dirs
    if let Ok(lib_dir) = std::env::var("OPENSOMEIP_LIB_DIR") {
        println!("cargo:rustc-link-search=native={lib_dir}");
        println!("cargo:rustc-link-lib=opensomeip");
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
    // The linker will search default paths. If the library is not
    // installed, linking will fail with a clear error message.
    println!("cargo:rustc-link-lib=opensomeip");
}
