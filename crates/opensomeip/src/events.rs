// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! Event Publisher and Subscriber — safe wrappers.
//!
//! @implements REQ_RUST_002, REQ_RUST_003, REQ_RUST_004, REQ_RUST_006, REQ_RUST_007

use core::ptr;
use opensomeip_sys::*;

#[cfg(feature = "std")]
use core::ffi::c_void;

use crate::error::{check, Result};

#[cfg(feature = "std")]
type EventCallbackBox = Box<dyn Fn(u16, u16, u16, &[u8]) + Send + Sync>;

// ──────────────────────────────────────────────────────────────────────────
// Event Publisher
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_event_publisher_t*`.
///
/// @implements REQ_RUST_002, REQ_RUST_006
pub struct EventPublisher {
    ptr: *mut opensomeip_event_publisher_t,
}

// SAFETY: The C API uses internal locking for event operations.
unsafe impl Send for EventPublisher {}
unsafe impl Sync for EventPublisher {}

impl EventPublisher {
    /// Create a new event publisher for the given service/instance.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new(service_id: u16, instance_id: u16) -> Result<Self> {
        let mut ptr: *mut opensomeip_event_publisher_t = ptr::null_mut();
        unsafe {
            check(opensomeip_event_publisher_create(
                &mut ptr,
                service_id,
                instance_id,
            ))?;
        }
        Ok(Self { ptr })
    }

    /// Initialize the publisher.
    pub fn initialize(&mut self) -> Result<()> {
        unsafe { check(opensomeip_event_publisher_initialize(self.ptr)) }
    }

    /// Shut down the publisher.
    pub fn shutdown(&mut self) -> Result<()> {
        unsafe { check(opensomeip_event_publisher_shutdown(self.ptr)) }
    }

    /// Register an event in an event group.
    ///
    /// @implements REQ_RUST_004
    pub fn register_event(&mut self, event_id: u16, eventgroup_id: u16) -> Result<()> {
        unsafe {
            check(opensomeip_event_publisher_register(
                self.ptr,
                event_id,
                eventgroup_id,
            ))
        }
    }

    /// Unregister an event.
    pub fn unregister_event(&mut self, event_id: u16) -> Result<()> {
        unsafe { check(opensomeip_event_publisher_unregister(self.ptr, event_id)) }
    }

    /// Publish (notify) an event with the given payload.
    pub fn notify(&self, event_id: u16, data: &[u8]) -> Result<()> {
        unsafe {
            check(opensomeip_event_publisher_notify(
                self.ptr as *mut _,
                event_id,
                data.as_ptr(),
                data.len(),
            ))
        }
    }
}

/// @implements REQ_RUST_002
impl Drop for EventPublisher {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = opensomeip_event_publisher_destroy(self.ptr);
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Event Subscriber
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_event_subscriber_t*`.
///
/// @implements REQ_RUST_002, REQ_RUST_006
pub struct EventSubscriber {
    ptr: *mut opensomeip_event_subscriber_t,
}

// SAFETY: The C API uses internal locking for event operations.
unsafe impl Send for EventSubscriber {}
unsafe impl Sync for EventSubscriber {}

impl EventSubscriber {
    /// Create a new event subscriber with the given client ID.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new(client_id: u16) -> Result<Self> {
        let mut ptr: *mut opensomeip_event_subscriber_t = ptr::null_mut();
        unsafe { check(opensomeip_event_subscriber_create(&mut ptr, client_id))? };
        Ok(Self { ptr })
    }

    /// Initialize the subscriber.
    pub fn initialize(&mut self) -> Result<()> {
        unsafe { check(opensomeip_event_subscriber_initialize(self.ptr)) }
    }

    /// Shut down the subscriber.
    pub fn shutdown(&mut self) -> Result<()> {
        unsafe { check(opensomeip_event_subscriber_shutdown(self.ptr)) }
    }

    /// Subscribe to events in an event group.
    ///
    /// The callback receives `(service_id, instance_id, event_id, data)`.
    ///
    /// @implements REQ_RUST_004
    #[cfg(feature = "std")]
    pub fn subscribe<F>(
        &mut self,
        service_id: u16,
        instance_id: u16,
        eventgroup_id: u16,
        callback: F,
    ) -> Result<()>
    where
        F: Fn(u16, u16, u16, &[u8]) + Send + Sync + 'static,
    {
        let boxed: Box<EventCallbackBox> = Box::new(Box::new(callback));
        let user_data = Box::into_raw(boxed) as *mut c_void;

        let result = unsafe {
            opensomeip_event_subscriber_subscribe(
                self.ptr,
                service_id,
                instance_id,
                eventgroup_id,
                Some(event_callback_trampoline),
                user_data,
            )
        };

        match check(result) {
            Ok(()) => Ok(()),
            Err(e) => {
                let _ = unsafe { Box::from_raw(user_data as *mut EventCallbackBox) };
                Err(e)
            }
        }
    }

    /// Unsubscribe from an event group.
    pub fn unsubscribe(
        &mut self,
        service_id: u16,
        instance_id: u16,
        eventgroup_id: u16,
    ) -> Result<()> {
        unsafe {
            check(opensomeip_event_subscriber_unsubscribe(
                self.ptr,
                service_id,
                instance_id,
                eventgroup_id,
            ))
        }
    }
}

/// Trampoline for event notification callback. Must not panic.
#[cfg(feature = "std")]
unsafe extern "C" fn event_callback_trampoline(
    service_id: u16,
    instance_id: u16,
    event_id: u16,
    data: *const u8,
    data_len: usize,
    user_data: *mut c_void,
) {
    // SAFETY: user_data is a leaked Box that remains valid while subscribed.
    let cb = unsafe { &*(user_data as *const EventCallbackBox) };
    let payload = if data.is_null() || data_len == 0 {
        &[]
    } else {
        // SAFETY: data is valid for data_len bytes during the callback.
        unsafe { core::slice::from_raw_parts(data, data_len) }
    };
    cb(service_id, instance_id, event_id, payload);
}

/// @implements REQ_RUST_002
impl Drop for EventSubscriber {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = opensomeip_event_subscriber_destroy(self.ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @tests REQ_RUST_006
    #[test]
    fn event_publisher_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<EventPublisher>();
    }

    /// @tests REQ_RUST_006
    #[test]
    fn event_subscriber_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<EventSubscriber>();
    }
}
