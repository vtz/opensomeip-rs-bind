// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! SOME/IP Transport Protocol (TP) manager — safe wrapper for segmentation
//! and reassembly of large payloads.
//!
//! @implements REQ_RUST_002, REQ_RUST_003, REQ_RUST_004, REQ_RUST_007

use core::ptr;
use opensomeip_sys::*;

use crate::error::{check, Result};
use crate::message::SomeIpMessage;

/// RAII wrapper around `opensomeip_tp_manager_t*`.
///
/// @implements REQ_RUST_002
pub struct TpManager {
    ptr: *mut opensomeip_tp_manager_t,
}

// SAFETY: TP manager uses internal locking.
unsafe impl Send for TpManager {}
unsafe impl Sync for TpManager {}

impl TpManager {
    /// Create a new TP manager.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new() -> Result<Self> {
        let mut ptr: *mut opensomeip_tp_manager_t = ptr::null_mut();
        unsafe { check(opensomeip_tp_manager_create(&mut ptr))? };
        Ok(Self { ptr })
    }

    /// Initialize the TP manager.
    pub fn initialize(&mut self) -> Result<()> {
        unsafe { check(opensomeip_tp_manager_initialize(self.ptr)) }
    }

    /// Shut down the TP manager.
    pub fn shutdown(&mut self) -> Result<()> {
        unsafe { check(opensomeip_tp_manager_shutdown(self.ptr)) }
    }

    /// Check if a payload needs TP segmentation.
    ///
    /// @implements REQ_RUST_004
    pub fn needs_segmentation(&self, payload: &[u8]) -> Result<bool> {
        let mut needs = 0i32;
        unsafe {
            check(opensomeip_tp_needs_segmentation(
                self.ptr,
                payload.as_ptr(),
                payload.len(),
                &mut needs,
            ))?;
        }
        Ok(needs != 0)
    }

    /// Segment a message payload into TP segments.
    ///
    /// Returns the segments written into `out_buf`. The actual number
    /// of bytes written is returned.
    pub fn segment(&mut self, msg: &SomeIpMessage, out_buf: &mut [u8]) -> Result<usize> {
        let mut out_len = out_buf.len();
        unsafe {
            check(opensomeip_tp_segment(
                self.ptr,
                msg.as_ptr(),
                out_buf.as_mut_ptr(),
                &mut out_len,
            ))?;
        }
        Ok(out_len)
    }

    /// Segment a message and return the result as a `Vec<u8>`.
    #[cfg(feature = "std")]
    pub fn segment_to_vec(&mut self, msg: &SomeIpMessage) -> Result<Vec<u8>> {
        // Allocate a generous buffer; TP segments add overhead.
        let mut buf = vec![0u8; 64 * 1024];
        let len = self.segment(msg, &mut buf)?;
        buf.truncate(len);
        Ok(buf)
    }

    /// Feed a received TP segment for reassembly.
    ///
    /// When reassembly is complete, returns `Some(actual_len)` where
    /// `actual_len` bytes have been written to `out_buf`. When more
    /// segments are needed, returns `None`.
    pub fn reassemble(&mut self, segment_data: &[u8], out_buf: &mut [u8]) -> Result<Option<usize>> {
        let mut out_len = out_buf.len();
        let mut complete = 0i32;
        unsafe {
            check(opensomeip_tp_reassemble(
                self.ptr,
                segment_data.as_ptr(),
                segment_data.len(),
                out_buf.as_mut_ptr(),
                &mut out_len,
                &mut complete,
            ))?;
        }
        if complete != 0 {
            Ok(Some(out_len))
        } else {
            Ok(None)
        }
    }
}

/// @implements REQ_RUST_002
impl Drop for TpManager {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = opensomeip_tp_manager_destroy(self.ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @tests REQ_RUST_006
    #[test]
    fn tp_manager_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TpManager>();
    }
}
