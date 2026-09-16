// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! RPC Client and Server — safe wrappers with callback handling.
//!
//! @implements REQ_RUST_002, REQ_RUST_003, REQ_RUST_004, REQ_RUST_006, REQ_RUST_007

use core::ptr;
use opensomeip_sys::*;

#[cfg(feature = "std")]
use core::ffi::c_void;

#[cfg(feature = "std")]
use crate::error::SomeIpError;
use crate::error::{check, Result};

/// Type alias for the boxed RPC async callback closure.
#[cfg(feature = "std")]
type RpcCallbackBox = Box<dyn FnOnce(core::result::Result<&[u8], SomeIpError>) + Send>;

/// Type alias for the boxed RPC method handler closure.
#[cfg(feature = "std")]
type MethodHandlerBox = Box<
    dyn Fn(u16, u16, &[u8]) -> core::result::Result<std::vec::Vec<u8>, SomeIpError> + Send + Sync,
>;

// ──────────────────────────────────────────────────────────────────────────
// RPC Client
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_rpc_client_t*`.
///
/// @implements REQ_RUST_002, REQ_RUST_006
pub struct RpcClient {
    ptr: *mut opensomeip_rpc_client_t,
}

// SAFETY: The C API uses internal locking for RPC operations.
unsafe impl Send for RpcClient {}
unsafe impl Sync for RpcClient {}

impl RpcClient {
    /// Create a new RPC client with the given client ID.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new(client_id: u16) -> Result<Self> {
        let mut ptr: *mut opensomeip_rpc_client_t = ptr::null_mut();
        // SAFETY: FFI call with valid out-pointer.
        unsafe { check(opensomeip_rpc_client_create(&mut ptr, client_id))? };
        Ok(Self { ptr })
    }

    /// Initialize the client.
    pub fn initialize(&mut self) -> Result<()> {
        unsafe { check(opensomeip_rpc_client_initialize(self.ptr)) }
    }

    /// Shut down the client.
    pub fn shutdown(&mut self) -> Result<()> {
        unsafe { check(opensomeip_rpc_client_shutdown(self.ptr)) }
    }

    /// Synchronous RPC call.
    ///
    /// Sends `input` to `service_id/method_id` and waits up to `timeout_ms`
    /// for a response. Returns the response payload.
    ///
    /// @implements REQ_RUST_004
    #[cfg(feature = "std")]
    pub fn call_sync(
        &mut self,
        service_id: u16,
        method_id: u16,
        input: &[u8],
        timeout_ms: u32,
    ) -> Result<Vec<u8>> {
        let mut output = vec![0u8; OPENSOMEIP_RPC_METHOD_MAX_RESPONSE];
        let mut output_len = output.len();
        // SAFETY: All pointers are valid for the duration of the call.
        unsafe {
            check(opensomeip_rpc_client_call_sync(
                self.ptr,
                service_id,
                method_id,
                input.as_ptr(),
                input.len(),
                output.as_mut_ptr(),
                &mut output_len,
                timeout_ms,
            ))?;
        }
        output.truncate(output_len);
        Ok(output)
    }

    /// Synchronous RPC call into a caller-provided buffer.
    ///
    /// Returns the actual number of response bytes written.
    pub fn call_sync_into(
        &mut self,
        service_id: u16,
        method_id: u16,
        input: &[u8],
        output: &mut [u8],
        timeout_ms: u32,
    ) -> Result<usize> {
        let mut output_len = output.len();
        unsafe {
            check(opensomeip_rpc_client_call_sync(
                self.ptr,
                service_id,
                method_id,
                input.as_ptr(),
                input.len(),
                output.as_mut_ptr(),
                &mut output_len,
                timeout_ms,
            ))?;
        }
        Ok(output_len)
    }

    /// Asynchronous RPC call.
    ///
    /// The `callback` receives the result, response data, and response length.
    /// Returns a handle that can be used to cancel the call.
    ///
    /// @implements REQ_RUST_004
    #[cfg(feature = "std")]
    pub fn call_async<F>(
        &mut self,
        service_id: u16,
        method_id: u16,
        input: &[u8],
        timeout_ms: u32,
        callback: F,
    ) -> Result<u32>
    where
        F: FnOnce(core::result::Result<&[u8], SomeIpError>) + Send + 'static,
    {
        // Box the closure and pass as userdata.
        let boxed: Box<RpcCallbackBox> = Box::new(Box::new(callback));
        let user_data = Box::into_raw(boxed) as *mut c_void;

        let mut handle = 0u32;
        // SAFETY: trampoline and user_data are valid. The C API will call
        // the trampoline exactly once with the provided user_data.
        let result = unsafe {
            opensomeip_rpc_client_call_async(
                self.ptr,
                service_id,
                method_id,
                input.as_ptr(),
                input.len(),
                Some(rpc_callback_trampoline),
                user_data,
                timeout_ms,
                &mut handle,
            )
        };

        match check(result) {
            Ok(()) => Ok(handle),
            Err(e) => {
                // Reclaim the Box to avoid a leak.
                // SAFETY: user_data was just created from Box::into_raw above.
                let _ = unsafe { Box::from_raw(user_data as *mut RpcCallbackBox) };
                Err(e)
            }
        }
    }

    /// Cancel a pending asynchronous call.
    pub fn cancel(&mut self, handle: u32) -> Result<()> {
        unsafe { check(opensomeip_rpc_client_cancel(self.ptr, handle)) }
    }
}

/// Trampoline function for async RPC callbacks.
///
/// This is an `extern "C"` function that the C API calls. It reconstructs
/// the boxed Rust closure from `user_data` and invokes it. Must not panic
/// (in panic=abort mode, a panic here aborts the process).
///
/// @implements REQ_RUST_004
#[cfg(feature = "std")]
unsafe extern "C" fn rpc_callback_trampoline(
    result: opensomeip_result_t,
    return_data: *const u8,
    return_data_len: usize,
    user_data: *mut c_void,
) {
    // SAFETY: user_data was created by call_async from Box::into_raw.
    let cb = unsafe { Box::from_raw(user_data as *mut RpcCallbackBox) };

    let outcome = if result == opensomeip_result_t::OPENSOMEIP_RESULT_SUCCESS {
        // SAFETY: On success, return_data is valid for return_data_len bytes.
        let data = if return_data.is_null() || return_data_len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(return_data, return_data_len) }
        };
        Ok(data)
    } else {
        Err(SomeIpError::from_result(result).unwrap_err())
    };

    cb(outcome);
}

/// @implements REQ_RUST_002
impl Drop for RpcClient {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: ptr was obtained from opensomeip_rpc_client_create.
            unsafe {
                let _ = opensomeip_rpc_client_destroy(self.ptr);
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// RPC Server
// ──────────────────────────────────────────────────────────────────────────

/// RAII wrapper around `opensomeip_rpc_server_t*`.
///
/// @implements REQ_RUST_002, REQ_RUST_006
pub struct RpcServer {
    ptr: *mut opensomeip_rpc_server_t,
    /// Stored handler boxes to prevent them from being dropped while registered.
    #[cfg(feature = "std")]
    _handlers: std::vec::Vec<*mut c_void>,
}

// SAFETY: The C API uses internal locking for RPC server operations.
unsafe impl Send for RpcServer {}
unsafe impl Sync for RpcServer {}

impl RpcServer {
    /// Create a new RPC server for the given service ID.
    ///
    /// @implements REQ_RUST_002, REQ_RUST_003
    pub fn new(service_id: u16) -> Result<Self> {
        let mut ptr: *mut opensomeip_rpc_server_t = ptr::null_mut();
        // SAFETY: FFI call with valid out-pointer.
        unsafe { check(opensomeip_rpc_server_create(&mut ptr, service_id))? };
        Ok(Self {
            ptr,
            #[cfg(feature = "std")]
            _handlers: std::vec::Vec::new(),
        })
    }

    /// Initialize the server.
    pub fn initialize(&mut self) -> Result<()> {
        unsafe { check(opensomeip_rpc_server_initialize(self.ptr)) }
    }

    /// Shut down the server.
    pub fn shutdown(&mut self) -> Result<()> {
        unsafe { check(opensomeip_rpc_server_shutdown(self.ptr)) }
    }

    /// Register a method handler.
    ///
    /// The handler receives `(client_id, session_id, input_data)` and
    /// returns `Result<Vec<u8>>` with the response payload.
    ///
    /// @implements REQ_RUST_004
    #[cfg(feature = "std")]
    pub fn register_method<F>(&mut self, method_id: u16, handler: F) -> Result<()>
    where
        F: Fn(u16, u16, &[u8]) -> core::result::Result<std::vec::Vec<u8>, SomeIpError>
            + Send
            + Sync
            + 'static,
    {
        let boxed: Box<MethodHandlerBox> = Box::new(Box::new(handler));
        let user_data = Box::into_raw(boxed) as *mut c_void;

        // SAFETY: trampoline and user_data are valid for the lifetime of the registration.
        let result = unsafe {
            opensomeip_rpc_server_register_method(
                self.ptr,
                method_id,
                Some(method_handler_trampoline),
                user_data,
            )
        };

        match check(result) {
            Ok(()) => {
                self._handlers.push(user_data);
                Ok(())
            }
            Err(e) => {
                // SAFETY: user_data was just created above.
                let _ = unsafe { Box::from_raw(user_data as *mut MethodHandlerBox) };
                Err(e)
            }
        }
    }

    /// Unregister a method handler.
    pub fn unregister_method(&mut self, method_id: u16) -> Result<()> {
        unsafe { check(opensomeip_rpc_server_unregister_method(self.ptr, method_id)) }
    }
}

/// Trampoline for method handlers. Must not panic.
///
/// @implements REQ_RUST_004
#[cfg(feature = "std")]
unsafe extern "C" fn method_handler_trampoline(
    client_id: u16,
    session_id: u16,
    input_data: *const u8,
    input_len: usize,
    output_data: *mut u8,
    output_len: *mut usize,
    user_data: *mut c_void,
) -> opensomeip_result_t {
    // SAFETY: user_data is a leaked Box that remains valid while registered.
    let handler = unsafe { &*(user_data as *const MethodHandlerBox) };

    // SAFETY: input_data is valid for input_len bytes.
    let input = if input_data.is_null() || input_len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(input_data, input_len) }
    };

    match handler(client_id, session_id, input) {
        Ok(response) => {
            // SAFETY: output_data and output_len are valid pointers from the C bridge.
            let capacity = unsafe { *output_len };
            if response.len() > capacity {
                return opensomeip_result_t::OPENSOMEIP_RESULT_BUFFER_OVERFLOW;
            }
            unsafe {
                core::ptr::copy_nonoverlapping(response.as_ptr(), output_data, response.len());
                *output_len = response.len();
            }
            opensomeip_result_t::OPENSOMEIP_RESULT_SUCCESS
        }
        Err(e) => {
            // Map the error back to a result code.
            match e {
                SomeIpError::InvalidArgument => {
                    opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_ARGUMENT
                }
                SomeIpError::InternalError => opensomeip_result_t::OPENSOMEIP_RESULT_INTERNAL_ERROR,
                SomeIpError::NotImplemented => {
                    opensomeip_result_t::OPENSOMEIP_RESULT_NOT_IMPLEMENTED
                }
                _ => opensomeip_result_t::OPENSOMEIP_RESULT_INTERNAL_ERROR,
            }
        }
    }
}

/// @implements REQ_RUST_002
impl Drop for RpcServer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // SAFETY: ptr was obtained from opensomeip_rpc_server_create.
            unsafe {
                let _ = opensomeip_rpc_server_destroy(self.ptr);
            }
        }
        // Note: handler Box pointers in _handlers are intentionally leaked
        // to match the C API lifetime. They are freed when the C library
        // destroys the server and stops invoking the callbacks.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @tests REQ_RUST_006
    #[test]
    fn rpc_client_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RpcClient>();
    }

    /// @tests REQ_RUST_006
    #[test]
    fn rpc_server_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RpcServer>();
    }
}
