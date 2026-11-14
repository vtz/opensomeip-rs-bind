// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! End-to-End (E2E) protection wrapper.
//!
//! @implements REQ_RUST_002, REQ_RUST_003, REQ_RUST_004, REQ_RUST_007

use core::ptr;
use opensomeip_sys::*;

use crate::error::{check, Result};
use crate::message::SomeIpMessage;

/// RAII wrapper around `opensomeip_e2e_t*`.
///
/// @implements REQ_RUST_002
pub struct E2EProtection {
    ptr: *mut opensomeip_e2e_t,
}

// SAFETY: E2E protection uses internal state per handle; safe to move.
unsafe impl Send for E2EProtection {}

impl E2EProtection {
    /// Create a new E2E protection instance.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new() -> Result<Self> {
        let mut ptr: *mut opensomeip_e2e_t = ptr::null_mut();
        unsafe { check(opensomeip_e2e_create(&mut ptr))? };
        Ok(Self { ptr })
    }

    /// Protect a message by adding an E2E header.
    ///
    /// @implements REQ_RUST_004
    pub fn protect(&mut self, msg: &mut SomeIpMessage, data_id: u16, counter: u32) -> Result<()> {
        unsafe {
            check(opensomeip_e2e_protect(
                self.ptr,
                msg.as_mut_ptr(),
                data_id,
                counter,
            ))
        }
    }

    /// Check the E2E protection of a received message.
    pub fn check_protection(&mut self, msg: &SomeIpMessage, data_id: u16) -> Result<()> {
        unsafe { check(opensomeip_e2e_check(self.ptr, msg.as_ptr(), data_id)) }
    }
}

/// @implements REQ_RUST_002
impl Drop for E2EProtection {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = opensomeip_e2e_destroy(self.ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @tests REQ_RUST_006
    #[test]
    fn e2e_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<E2EProtection>();
    }
}
