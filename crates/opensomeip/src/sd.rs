// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! Service Discovery Client and Server — safe wrappers.
//!
//! @implements REQ_RUST_002, REQ_RUST_003, REQ_RUST_004, REQ_RUST_006, REQ_RUST_007

use core::ptr;
use opensomeip_sys::*;

#[cfg(feature = "std")]
use core::ffi::c_void;

use crate::error::{check, Result};
use crate::types::Endpoint;

#[cfg(feature = "std")]
type SdFoundBox = Box<dyn FnOnce(u16, u16, &Endpoint) + Send>;

#[cfg(feature = "std")]
type SdAvailabilityBox = Box<dyn Fn(u16, u16, bool) + Send + Sync>;

// ──────────────────────────────────────────────────────────────────────────
// SD Client
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_sd_client_t*`.
///
/// @implements REQ_RUST_002, REQ_RUST_006
pub struct SdClient {
    ptr: *mut opensomeip_sd_client_t,
}

// SAFETY: The C API uses internal locking for SD operations.
unsafe impl Send for SdClient {}
unsafe impl Sync for SdClient {}

impl SdClient {
    /// Create a new SD client.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new() -> Result<Self> {
        let mut ptr: *mut opensomeip_sd_client_t = ptr::null_mut();
        unsafe { check(opensomeip_sd_client_create(&mut ptr))? };
        Ok(Self { ptr })
    }

    /// Initialize the SD client.
    pub fn initialize(&mut self) -> Result<()> {
        unsafe { check(opensomeip_sd_client_initialize(self.ptr)) }
    }

    /// Shut down the SD client.
    pub fn shutdown(&mut self) -> Result<()> {
        unsafe { check(opensomeip_sd_client_shutdown(self.ptr)) }
    }

    /// Find a service by ID. The callback is invoked when found.
    ///
    /// @implements REQ_RUST_004
    #[cfg(feature = "std")]
    pub fn find_service<F>(&mut self, service_id: u16, timeout_ms: u32, callback: F) -> Result<()>
    where
        F: FnOnce(u16, u16, &Endpoint) + Send + 'static,
    {
        let boxed: Box<SdFoundBox> = Box::new(Box::new(callback));
        let user_data = Box::into_raw(boxed) as *mut c_void;

        let result = unsafe {
            opensomeip_sd_client_find_service(
                self.ptr,
                service_id,
                Some(sd_found_trampoline),
                user_data,
                timeout_ms,
            )
        };

        match check(result) {
            Ok(()) => Ok(()),
            Err(e) => {
                // SAFETY: Reclaim the box on error.
                let _ = unsafe { Box::from_raw(user_data as *mut SdFoundBox) };
                Err(e)
            }
        }
    }

    /// Subscribe to service availability changes.
    ///
    /// @implements REQ_RUST_004
    #[cfg(feature = "std")]
    pub fn subscribe_availability<F>(&mut self, service_id: u16, callback: F) -> Result<()>
    where
        F: Fn(u16, u16, bool) + Send + Sync + 'static,
    {
        let boxed: Box<SdAvailabilityBox> = Box::new(Box::new(callback));
        let user_data = Box::into_raw(boxed) as *mut c_void;

        let result = unsafe {
            opensomeip_sd_client_subscribe_availability(
                self.ptr,
                service_id,
                Some(sd_availability_trampoline),
                user_data,
            )
        };

        match check(result) {
            Ok(()) => Ok(()),
            Err(e) => {
                let _ = unsafe { Box::from_raw(user_data as *mut SdAvailabilityBox) };
                Err(e)
            }
        }
    }

    /// Subscribe to an event group.
    pub fn subscribe_eventgroup(
        &mut self,
        service_id: u16,
        instance_id: u16,
        eventgroup_id: u16,
    ) -> Result<()> {
        unsafe {
            check(opensomeip_sd_client_subscribe_eventgroup(
                self.ptr,
                service_id,
                instance_id,
                eventgroup_id,
            ))
        }
    }
}

/// Trampoline for SD found callback. Must not panic.
#[cfg(feature = "std")]
unsafe extern "C" fn sd_found_trampoline(
    service_id: u16,
    instance_id: u16,
    endpoint: *const opensomeip_endpoint_t,
    user_data: *mut c_void,
) {
    // SAFETY: user_data was created by find_service from Box::into_raw.
    let cb = unsafe { Box::from_raw(user_data as *mut SdFoundBox) };
    // SAFETY: endpoint is valid for the duration of the callback.
    let ep = Endpoint::from_raw(unsafe { &*endpoint });
    cb(service_id, instance_id, &ep);
}

/// Trampoline for SD availability callback. Must not panic.
#[cfg(feature = "std")]
unsafe extern "C" fn sd_availability_trampoline(
    service_id: u16,
    instance_id: u16,
    available: i32,
    user_data: *mut c_void,
) {
    // SAFETY: user_data is a leaked Box that remains valid while subscribed.
    let cb = unsafe { &*(user_data as *const SdAvailabilityBox) };
    cb(service_id, instance_id, available != 0);
}

/// @implements REQ_RUST_002
impl Drop for SdClient {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = opensomeip_sd_client_destroy(self.ptr);
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// SD Server
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_sd_server_t*`.
///
/// @implements REQ_RUST_002, REQ_RUST_006
pub struct SdServer {
    ptr: *mut opensomeip_sd_server_t,
}

// SAFETY: The C API uses internal locking for SD operations.
unsafe impl Send for SdServer {}
unsafe impl Sync for SdServer {}

impl SdServer {
    /// Create a new SD server.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new() -> Result<Self> {
        let mut ptr: *mut opensomeip_sd_server_t = ptr::null_mut();
        unsafe { check(opensomeip_sd_server_create(&mut ptr))? };
        Ok(Self { ptr })
    }

    /// Initialize the SD server.
    pub fn initialize(&mut self) -> Result<()> {
        unsafe { check(opensomeip_sd_server_initialize(self.ptr)) }
    }

    /// Shut down the SD server.
    pub fn shutdown(&mut self) -> Result<()> {
        unsafe { check(opensomeip_sd_server_shutdown(self.ptr)) }
    }

    /// Offer a service at the given endpoint.
    ///
    /// @implements REQ_RUST_004
    pub fn offer_service(
        &mut self,
        service_id: u16,
        instance_id: u16,
        endpoint: &Endpoint,
    ) -> Result<()> {
        let raw_ep = endpoint.to_raw();
        unsafe {
            check(opensomeip_sd_server_offer_service(
                self.ptr,
                service_id,
                instance_id,
                &raw_ep,
            ))
        }
    }

    /// Stop offering a service.
    pub fn stop_offer(&mut self, service_id: u16, instance_id: u16) -> Result<()> {
        unsafe {
            check(opensomeip_sd_server_stop_offer(
                self.ptr,
                service_id,
                instance_id,
            ))
        }
    }
}

/// @implements REQ_RUST_002
impl Drop for SdServer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = opensomeip_sd_server_destroy(self.ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @tests REQ_RUST_006
    #[test]
    fn sd_client_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SdClient>();
    }

    /// @tests REQ_RUST_006
    #[test]
    fn sd_server_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SdServer>();
    }
}
