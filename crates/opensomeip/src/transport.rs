// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! UDP and TCP transport wrappers.
//!
//! @implements REQ_RUST_002, REQ_RUST_004, REQ_RUST_006, REQ_RUST_007

use core::ptr;
use opensomeip_sys::*;

use crate::error::{check, Result};
use crate::message::SomeIpMessage;
use crate::types::Endpoint;

// ──────────────────────────────────────────────────────────────────────────
// UDP Transport
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_udp_transport_t*`.
///
/// @implements REQ_RUST_002, REQ_RUST_006
pub struct UdpTransport {
    ptr: *mut opensomeip_udp_transport_t,
}

// SAFETY: The C API documents internal locking for transport types.
unsafe impl Send for UdpTransport {}
unsafe impl Sync for UdpTransport {}

impl UdpTransport {
    /// Create a new UDP transport bound to the given local endpoint.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new(local_endpoint: &Endpoint) -> Result<Self> {
        let raw_ep = local_endpoint.to_raw();
        let mut ptr: *mut opensomeip_udp_transport_t = ptr::null_mut();
        // SAFETY: raw_ep is a valid stack-local, ptr is a valid out-pointer.
        unsafe { check(opensomeip_udp_transport_create(&mut ptr, &raw_ep))? };
        Ok(Self { ptr })
    }

    /// Start the transport (begin listening).
    ///
    /// @implements REQ_RUST_004
    pub fn start(&mut self) -> Result<()> {
        unsafe { check(opensomeip_udp_transport_start(self.ptr)) }
    }

    /// Stop the transport.
    pub fn stop(&mut self) -> Result<()> {
        unsafe { check(opensomeip_udp_transport_stop(self.ptr)) }
    }

    /// Send a message to the given destination endpoint.
    pub fn send(&self, msg: &SomeIpMessage, dest: &Endpoint) -> Result<()> {
        let raw_ep = dest.to_raw();
        // SAFETY: self.ptr is valid, msg.as_ptr() is valid, raw_ep is stack-local.
        unsafe {
            check(opensomeip_udp_transport_send(
                self.ptr as *mut _,
                msg.as_ptr(),
                &raw_ep,
            ))
        }
    }

    /// Receive a message (blocking). Returns the message and sender endpoint.
    pub fn receive(&mut self) -> Result<(SomeIpMessage, Endpoint)> {
        let mut msg_ptr: *mut opensomeip_message_t = ptr::null_mut();
        let mut raw_ep = opensomeip_endpoint_t {
            address: [0u8; 64],
            port: 0,
            protocol: opensomeip_transport_protocol_t::OPENSOMEIP_TRANSPORT_UDP,
        };
        // SAFETY: FFI call with valid out-pointers.
        unsafe {
            check(opensomeip_udp_transport_receive(
                self.ptr,
                &mut msg_ptr,
                &mut raw_ep,
            ))?;
        }
        // SAFETY: On success, msg_ptr is a valid handle we now own.
        let msg = unsafe { SomeIpMessage::from_raw(msg_ptr) };
        Ok((msg, Endpoint::from_raw(&raw_ep)))
    }
}

/// @implements REQ_RUST_002
impl Drop for UdpTransport {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: ptr was obtained from opensomeip_udp_transport_create.
            unsafe {
                let _ = opensomeip_udp_transport_destroy(self.ptr);
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// TCP Transport
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_tcp_transport_t*`.
///
/// @implements REQ_RUST_002, REQ_RUST_006
pub struct TcpTransport {
    ptr: *mut opensomeip_tcp_transport_t,
}

// SAFETY: The C API documents internal locking for transport types.
unsafe impl Send for TcpTransport {}
unsafe impl Sync for TcpTransport {}

impl TcpTransport {
    /// Create a new TCP transport.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new() -> Result<Self> {
        let mut ptr: *mut opensomeip_tcp_transport_t = ptr::null_mut();
        // SAFETY: FFI call with valid out-pointer.
        unsafe { check(opensomeip_tcp_transport_create(&mut ptr))? };
        Ok(Self { ptr })
    }

    /// Initialize with a local endpoint.
    pub fn initialize(&mut self, local_endpoint: &Endpoint) -> Result<()> {
        let raw_ep = local_endpoint.to_raw();
        unsafe { check(opensomeip_tcp_transport_initialize(self.ptr, &raw_ep)) }
    }

    /// Start the transport.
    ///
    /// @implements REQ_RUST_004
    pub fn start(&mut self) -> Result<()> {
        unsafe { check(opensomeip_tcp_transport_start(self.ptr)) }
    }

    /// Stop the transport.
    pub fn stop(&mut self) -> Result<()> {
        unsafe { check(opensomeip_tcp_transport_stop(self.ptr)) }
    }

    /// Connect to a remote endpoint.
    pub fn connect(&mut self, remote_endpoint: &Endpoint) -> Result<()> {
        let raw_ep = remote_endpoint.to_raw();
        unsafe { check(opensomeip_tcp_transport_connect(self.ptr, &raw_ep)) }
    }

    /// Disconnect from the remote endpoint.
    pub fn disconnect(&mut self) -> Result<()> {
        unsafe { check(opensomeip_tcp_transport_disconnect(self.ptr)) }
    }

    /// Send a message to the given destination endpoint.
    pub fn send(&self, msg: &SomeIpMessage, dest: &Endpoint) -> Result<()> {
        let raw_ep = dest.to_raw();
        // SAFETY: self.ptr is valid, msg.as_ptr() is valid.
        unsafe {
            check(opensomeip_tcp_transport_send(
                self.ptr as *mut _,
                msg.as_ptr(),
                &raw_ep,
            ))
        }
    }
}

/// @implements REQ_RUST_002
impl Drop for TcpTransport {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: ptr was obtained from opensomeip_tcp_transport_create.
            unsafe {
                let _ = opensomeip_tcp_transport_destroy(self.ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @tests REQ_RUST_006
    #[test]
    fn udp_transport_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<UdpTransport>();
    }

    /// @tests REQ_RUST_006
    #[test]
    fn tcp_transport_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TcpTransport>();
    }
}
