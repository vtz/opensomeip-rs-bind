// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! Serializer and Deserializer — safe RAII wrappers.
//!
//! @implements REQ_RUST_002, REQ_RUST_003, REQ_RUST_004, REQ_RUST_007

use core::ptr;
use opensomeip_sys::*;

use crate::error::{check, Result};

// ──────────────────────────────────────────────────────────────────────────
// Serializer
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_serializer_t*`.
///
/// @implements REQ_RUST_002
pub struct Serializer {
    ptr: *mut opensomeip_serializer_t,
}

// SAFETY: Serializer handles are independent, can be moved across threads.
unsafe impl Send for Serializer {}

impl Serializer {
    /// Create a new serializer.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new() -> Result<Self> {
        let mut ptr: *mut opensomeip_serializer_t = ptr::null_mut();
        // SAFETY: FFI call with valid out-pointer.
        unsafe { check(opensomeip_serializer_create(&mut ptr))? };
        Ok(Self { ptr })
    }

    /// Reset the serializer, discarding all written data.
    pub fn reset(&mut self) -> Result<()> {
        // SAFETY: self.ptr is valid.
        unsafe { check(opensomeip_serializer_reset(self.ptr)) }
    }

    /// Write a `u8` value.
    ///
    /// @implements REQ_RUST_004
    pub fn write_u8(&mut self, val: u8) -> Result<()> {
        unsafe { check(opensomeip_serializer_write_uint8(self.ptr, val)) }
    }

    /// Write a `u16` value (big-endian on wire).
    pub fn write_u16(&mut self, val: u16) -> Result<()> {
        unsafe { check(opensomeip_serializer_write_uint16(self.ptr, val)) }
    }

    /// Write a `u32` value.
    pub fn write_u32(&mut self, val: u32) -> Result<()> {
        unsafe { check(opensomeip_serializer_write_uint32(self.ptr, val)) }
    }

    /// Write a `u64` value.
    pub fn write_u64(&mut self, val: u64) -> Result<()> {
        unsafe { check(opensomeip_serializer_write_uint64(self.ptr, val)) }
    }

    /// Write raw bytes.
    pub fn write_bytes(&mut self, data: &[u8]) -> Result<()> {
        unsafe {
            check(opensomeip_serializer_write_bytes(
                self.ptr,
                data.as_ptr(),
                data.len(),
            ))
        }
    }

    /// Get the total size of serialized data.
    pub fn size(&self) -> Result<usize> {
        let mut len = 0usize;
        unsafe { check(opensomeip_serializer_get_size(self.ptr, &mut len))? };
        Ok(len)
    }

    /// Copy serialized data into the provided buffer.
    /// Returns the actual number of bytes written.
    pub fn get_data(&self, buf: &mut [u8]) -> Result<usize> {
        let mut len = buf.len();
        unsafe {
            check(opensomeip_serializer_get_data(
                self.ptr,
                buf.as_mut_ptr(),
                &mut len,
            ))?;
        }
        Ok(len)
    }

    /// Get serialized data as a new `Vec<u8>`.
    #[cfg(feature = "std")]
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let size = self.size()?;
        let mut buf = vec![0u8; size];
        let actual = self.get_data(&mut buf)?;
        buf.truncate(actual);
        Ok(buf)
    }
}

/// @implements REQ_RUST_002
impl Drop for Serializer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: ptr was obtained from opensomeip_serializer_create.
            unsafe {
                let _ = opensomeip_serializer_destroy(self.ptr);
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Deserializer
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_deserializer_t*`.
///
/// @implements REQ_RUST_002
pub struct Deserializer {
    ptr: *mut opensomeip_deserializer_t,
}

// SAFETY: Deserializer handles are independent.
unsafe impl Send for Deserializer {}

impl Deserializer {
    /// Create a new deserializer from input data.
    ///
    /// The data is copied into the C object.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new(data: &[u8]) -> Result<Self> {
        let mut ptr: *mut opensomeip_deserializer_t = ptr::null_mut();
        // SAFETY: FFI call with valid data pointer and out-pointer.
        unsafe {
            check(opensomeip_deserializer_create(
                &mut ptr,
                data.as_ptr(),
                data.len(),
            ))?;
        }
        Ok(Self { ptr })
    }

    /// Read a `u8` value.
    ///
    /// @implements REQ_RUST_004
    pub fn read_u8(&mut self) -> Result<u8> {
        let mut val = 0u8;
        unsafe { check(opensomeip_deserializer_read_uint8(self.ptr, &mut val))? };
        Ok(val)
    }

    /// Read a `u16` value.
    pub fn read_u16(&mut self) -> Result<u16> {
        let mut val = 0u16;
        unsafe { check(opensomeip_deserializer_read_uint16(self.ptr, &mut val))? };
        Ok(val)
    }

    /// Read a `u32` value.
    pub fn read_u32(&mut self) -> Result<u32> {
        let mut val = 0u32;
        unsafe { check(opensomeip_deserializer_read_uint32(self.ptr, &mut val))? };
        Ok(val)
    }

    /// Read a `u64` value.
    pub fn read_u64(&mut self) -> Result<u64> {
        let mut val = 0u64;
        unsafe { check(opensomeip_deserializer_read_uint64(self.ptr, &mut val))? };
        Ok(val)
    }

    /// Read raw bytes into the provided buffer.
    /// Returns the actual number of bytes read.
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut len = buf.len();
        unsafe {
            check(opensomeip_deserializer_read_bytes(
                self.ptr,
                buf.as_mut_ptr(),
                &mut len,
            ))?;
        }
        Ok(len)
    }

    /// Get the number of remaining bytes.
    pub fn remaining(&self) -> Result<usize> {
        let mut out = 0usize;
        unsafe { check(opensomeip_deserializer_get_remaining(self.ptr, &mut out))? };
        Ok(out)
    }
}

/// @implements REQ_RUST_002
impl Drop for Deserializer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: ptr was obtained from opensomeip_deserializer_create.
            unsafe {
                let _ = opensomeip_deserializer_destroy(self.ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @tests REQ_RUST_002
    #[test]
    fn serializer_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Serializer>();
    }

    /// @tests REQ_RUST_002
    #[test]
    fn deserializer_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Deserializer>();
    }
}
