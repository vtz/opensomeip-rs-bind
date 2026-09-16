// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! SOME/IP message — RAII wrapper with builder pattern.
//!
//! @implements REQ_RUST_002, REQ_RUST_003, REQ_RUST_004, REQ_RUST_007

use core::ptr;
use opensomeip_sys::*;

use crate::error::{check, Result};
use crate::types::{MessageType, ReturnCode};

/// RAII wrapper around `opensomeip_message_t*`.
///
/// Automatically calls `opensomeip_message_destroy` on drop.
/// Not `Clone` — each handle is unique.
///
/// @implements REQ_RUST_002, REQ_RUST_006
pub struct SomeIpMessage {
    ptr: *mut opensomeip_message_t,
}

// SAFETY: The C API documents that each message handle is independent
// and can be safely moved across threads.
unsafe impl Send for SomeIpMessage {}

impl SomeIpMessage {
    /// Create a new empty SOME/IP message.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new() -> Result<Self> {
        let mut ptr: *mut opensomeip_message_t = ptr::null_mut();
        // SAFETY: FFI call with valid out-pointer. On success, ptr is a valid handle.
        unsafe { check(opensomeip_message_create(&mut ptr))? };
        Ok(Self { ptr })
    }

    /// Access the raw pointer (for use by other wrapper types).
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const opensomeip_message_t {
        self.ptr
    }

    /// Access the raw mutable pointer (for use by other wrapper types).
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut opensomeip_message_t {
        self.ptr
    }

    /// Take ownership of a raw pointer returned from the C API.
    ///
    /// # Safety
    /// `ptr` must be a valid handle returned by the C API that the caller
    /// is transferring ownership of.
    pub(crate) unsafe fn from_raw(ptr: *mut opensomeip_message_t) -> Self {
        Self { ptr }
    }

    // ── Header field accessors ──────────────────────────────────────────

    /// @implements REQ_RUST_004
    pub fn set_service_id(&mut self, id: u16) -> Result<()> {
        // SAFETY: self.ptr is valid for the lifetime of self.
        unsafe { check(opensomeip_message_set_service_id(self.ptr, id)) }
    }

    pub fn service_id(&self) -> Result<u16> {
        let mut out = 0u16;
        // SAFETY: self.ptr is valid, out is a valid local.
        unsafe { check(opensomeip_message_get_service_id(self.ptr, &mut out))? };
        Ok(out)
    }

    pub fn set_method_id(&mut self, id: u16) -> Result<()> {
        unsafe { check(opensomeip_message_set_method_id(self.ptr, id)) }
    }

    pub fn method_id(&self) -> Result<u16> {
        let mut out = 0u16;
        unsafe { check(opensomeip_message_get_method_id(self.ptr, &mut out))? };
        Ok(out)
    }

    pub fn set_client_id(&mut self, id: u16) -> Result<()> {
        unsafe { check(opensomeip_message_set_client_id(self.ptr, id)) }
    }

    pub fn client_id(&self) -> Result<u16> {
        let mut out = 0u16;
        unsafe { check(opensomeip_message_get_client_id(self.ptr, &mut out))? };
        Ok(out)
    }

    pub fn set_session_id(&mut self, id: u16) -> Result<()> {
        unsafe { check(opensomeip_message_set_session_id(self.ptr, id)) }
    }

    pub fn session_id(&self) -> Result<u16> {
        let mut out = 0u16;
        unsafe { check(opensomeip_message_get_session_id(self.ptr, &mut out))? };
        Ok(out)
    }

    pub fn set_protocol_version(&mut self, version: u8) -> Result<()> {
        unsafe { check(opensomeip_message_set_protocol_version(self.ptr, version)) }
    }

    pub fn protocol_version(&self) -> Result<u8> {
        let mut out = 0u8;
        unsafe { check(opensomeip_message_get_protocol_version(self.ptr, &mut out))? };
        Ok(out)
    }

    pub fn set_interface_version(&mut self, version: u8) -> Result<()> {
        unsafe { check(opensomeip_message_set_interface_version(self.ptr, version)) }
    }

    pub fn interface_version(&self) -> Result<u8> {
        let mut out = 0u8;
        unsafe { check(opensomeip_message_get_interface_version(self.ptr, &mut out))? };
        Ok(out)
    }

    pub fn set_message_type(&mut self, msg_type: MessageType) -> Result<()> {
        unsafe {
            check(opensomeip_message_set_message_type(
                self.ptr,
                msg_type.into(),
            ))
        }
    }

    pub fn message_type(&self) -> Result<MessageType> {
        let mut out = opensomeip_message_type_t::OPENSOMEIP_MSG_REQUEST;
        unsafe { check(opensomeip_message_get_message_type(self.ptr, &mut out))? };
        Ok(MessageType::from(out))
    }

    pub fn set_return_code(&mut self, code: ReturnCode) -> Result<()> {
        unsafe { check(opensomeip_message_set_return_code(self.ptr, code.into())) }
    }

    pub fn return_code(&self) -> Result<ReturnCode> {
        let mut out = opensomeip_return_code_t::OPENSOMEIP_RC_E_OK;
        unsafe { check(opensomeip_message_get_return_code(self.ptr, &mut out))? };
        Ok(ReturnCode::from(out))
    }

    // ── Payload ─────────────────────────────────────────────────────────

    /// Set message payload.
    ///
    /// @implements REQ_RUST_004
    pub fn set_payload(&mut self, data: &[u8]) -> Result<()> {
        // SAFETY: data pointer and len are valid for the slice lifetime.
        unsafe {
            check(opensomeip_message_set_payload(
                self.ptr,
                data.as_ptr(),
                data.len(),
            ))
        }
    }

    /// Get message payload into the provided buffer.
    /// Returns the actual number of bytes written.
    pub fn get_payload(&self, buf: &mut [u8]) -> Result<usize> {
        let mut len = buf.len();
        // SAFETY: buf and len are valid. len is in/out.
        unsafe {
            check(opensomeip_message_get_payload(
                self.ptr,
                buf.as_mut_ptr(),
                &mut len,
            ))?;
        }
        Ok(len)
    }

    /// Get the payload length without copying.
    pub fn payload_length(&self) -> Result<usize> {
        let mut len = 0usize;
        unsafe { check(opensomeip_message_get_payload_length(self.ptr, &mut len))? };
        Ok(len)
    }

    /// Get payload as a new `Vec<u8>` (requires `std` feature).
    #[cfg(feature = "std")]
    pub fn payload(&self) -> Result<Vec<u8>> {
        let len = self.payload_length()?;
        let mut buf = vec![0u8; len];
        let actual = self.get_payload(&mut buf)?;
        buf.truncate(actual);
        Ok(buf)
    }

    // ── Serialization ───────────────────────────────────────────────────

    /// Serialize message to wire format into the provided buffer.
    /// Returns actual number of bytes written.
    pub fn serialize_into(&self, buf: &mut [u8]) -> Result<usize> {
        let mut len = buf.len();
        unsafe {
            check(opensomeip_message_serialize(
                self.ptr,
                buf.as_mut_ptr(),
                &mut len,
            ))?;
        }
        Ok(len)
    }

    /// Serialize message to wire format as a new `Vec<u8>`.
    #[cfg(feature = "std")]
    pub fn serialize(&self) -> Result<Vec<u8>> {
        // Start with a reasonable buffer; the SOME/IP header is 8 bytes minimum
        let mut buf = vec![0u8; 4096];
        let len = self.serialize_into(&mut buf)?;
        buf.truncate(len);
        Ok(buf)
    }

    /// Deserialize wire data into this message, replacing current contents.
    pub fn deserialize(&mut self, data: &[u8]) -> Result<()> {
        unsafe {
            check(opensomeip_message_deserialize(
                self.ptr,
                data.as_ptr(),
                data.len(),
            ))
        }
    }
}

/// @implements REQ_RUST_002
impl Drop for SomeIpMessage {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: ptr was obtained from opensomeip_message_create and is valid.
            unsafe {
                let _ = opensomeip_message_destroy(self.ptr);
            }
        }
    }
}

// ── Builder ─────────────────────────────────────────────────────────────

/// Builder for constructing `SomeIpMessage` with a fluent API.
///
/// @implements REQ_RUST_002, REQ_RUST_004
pub struct SomeIpMessageBuilder {
    msg: SomeIpMessage,
}

impl SomeIpMessageBuilder {
    /// Start building a new message.
    pub fn new() -> Result<Self> {
        Ok(Self {
            msg: SomeIpMessage::new()?,
        })
    }

    pub fn service_id(mut self, id: u16) -> Result<Self> {
        self.msg.set_service_id(id)?;
        Ok(self)
    }

    pub fn method_id(mut self, id: u16) -> Result<Self> {
        self.msg.set_method_id(id)?;
        Ok(self)
    }

    pub fn client_id(mut self, id: u16) -> Result<Self> {
        self.msg.set_client_id(id)?;
        Ok(self)
    }

    pub fn session_id(mut self, id: u16) -> Result<Self> {
        self.msg.set_session_id(id)?;
        Ok(self)
    }

    pub fn protocol_version(mut self, v: u8) -> Result<Self> {
        self.msg.set_protocol_version(v)?;
        Ok(self)
    }

    pub fn interface_version(mut self, v: u8) -> Result<Self> {
        self.msg.set_interface_version(v)?;
        Ok(self)
    }

    pub fn message_type(mut self, t: MessageType) -> Result<Self> {
        self.msg.set_message_type(t)?;
        Ok(self)
    }

    pub fn return_code(mut self, c: ReturnCode) -> Result<Self> {
        self.msg.set_return_code(c)?;
        Ok(self)
    }

    pub fn payload(mut self, data: &[u8]) -> Result<Self> {
        self.msg.set_payload(data)?;
        Ok(self)
    }

    /// Consume the builder and return the message.
    pub fn build(self) -> SomeIpMessage {
        self.msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Tests that call C functions require the opensomeip library
    // to be linked. These are compile-time structure tests.

    /// @tests REQ_RUST_002
    #[test]
    fn message_size_is_pointer() {
        // SomeIpMessage holds a single pointer — verify it's pointer-sized.
        assert_eq!(
            core::mem::size_of::<SomeIpMessage>(),
            core::mem::size_of::<*mut u8>(),
        );
    }

    /// @tests REQ_RUST_006
    #[test]
    fn message_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<SomeIpMessage>();
    }
}
