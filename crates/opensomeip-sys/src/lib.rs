//! Raw FFI bindings to the opensomeip C API.
//!
//! This crate provides `extern "C"` function declarations matching
//! `include/capi/opensomeip.h` from the opensomeip C++ project.
//! Bindings are hand-written (no build-time bindgen) for Ferrocene compatibility.

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

// FFI declarations will be added in OSI-RS-3
