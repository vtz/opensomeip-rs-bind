//! Raw FFI bindings to the opensomeip C API.
//!
//! This crate provides `extern "C"` function declarations matching
//! `include/capi/opensomeip.h` from the opensomeip C++ project.
//! Bindings are hand-written (no build-time bindgen) for Ferrocene compatibility.
//!
//! @implements REQ_RUST_001, REQ_RUST_005

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]

use core::ffi::c_void;

// ──────────────────────────────────────────────────────────────────────────
// Version constants
// ──────────────────────────────────────────────────────────────────────────

/// @implements REQ_RUST_001
pub const OPENSOMEIP_CAPI_VERSION_MAJOR: u32 = 0;
pub const OPENSOMEIP_CAPI_VERSION_MINOR: u32 = 1;
pub const OPENSOMEIP_CAPI_VERSION_PATCH: u32 = 0;

/// Maximum response payload size for RPC method handlers.
pub const OPENSOMEIP_RPC_METHOD_MAX_RESPONSE: usize = 4096;

// ──────────────────────────────────────────────────────────────────────────
// Result codes
// ──────────────────────────────────────────────────────────────────────────

/// @implements REQ_RUST_001
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum opensomeip_result_t {
    OPENSOMEIP_RESULT_SUCCESS = 0x00,

    OPENSOMEIP_RESULT_NETWORK_ERROR = 0x01,
    OPENSOMEIP_RESULT_NOT_CONNECTED = 0x02,
    OPENSOMEIP_RESULT_CONNECTION_LOST = 0x03,
    OPENSOMEIP_RESULT_CONNECTION_REFUSED = 0x04,
    OPENSOMEIP_RESULT_TIMEOUT = 0x05,
    OPENSOMEIP_RESULT_INVALID_ENDPOINT = 0x06,

    OPENSOMEIP_RESULT_INVALID_MESSAGE = 0x10,
    OPENSOMEIP_RESULT_INVALID_MESSAGE_TYPE = 0x11,
    OPENSOMEIP_RESULT_INVALID_SERVICE_ID = 0x12,
    OPENSOMEIP_RESULT_INVALID_METHOD_ID = 0x13,
    OPENSOMEIP_RESULT_INVALID_PROTOCOL_VERSION = 0x14,
    OPENSOMEIP_RESULT_INVALID_INTERFACE_VERSION = 0x15,
    OPENSOMEIP_RESULT_MALFORMED_MESSAGE = 0x16,

    OPENSOMEIP_RESULT_INVALID_SESSION_ID = 0x20,
    OPENSOMEIP_RESULT_SESSION_EXPIRED = 0x21,
    OPENSOMEIP_RESULT_SESSION_NOT_FOUND = 0x22,

    OPENSOMEIP_RESULT_OUT_OF_MEMORY = 0x30,
    OPENSOMEIP_RESULT_BUFFER_OVERFLOW = 0x31,
    OPENSOMEIP_RESULT_RESOURCE_EXHAUSTED = 0x32,

    OPENSOMEIP_RESULT_SERVICE_NOT_FOUND = 0x40,
    OPENSOMEIP_RESULT_SERVICE_UNAVAILABLE = 0x41,
    OPENSOMEIP_RESULT_SUBSCRIPTION_FAILED = 0x42,

    OPENSOMEIP_RESULT_SAFETY_VIOLATION = 0x50,
    OPENSOMEIP_RESULT_FAULT_DETECTED = 0x51,
    OPENSOMEIP_RESULT_RECOVERY_FAILED = 0x52,

    OPENSOMEIP_RESULT_NOT_IMPLEMENTED = 0x60,
    OPENSOMEIP_RESULT_INVALID_ARGUMENT = 0x61,
    OPENSOMEIP_RESULT_PERMISSION_DENIED = 0x62,
    OPENSOMEIP_RESULT_INTERNAL_ERROR = 0x63,
    OPENSOMEIP_RESULT_NOT_INITIALIZED = 0x64,
    OPENSOMEIP_RESULT_INVALID_STATE = 0x65,

    OPENSOMEIP_RESULT_UNKNOWN_ERROR = 0xFF,
}

// ──────────────────────────────────────────────────────────────────────────
// SOME/IP message types
// ──────────────────────────────────────────────────────────────────────────

/// @implements REQ_RUST_001
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum opensomeip_message_type_t {
    OPENSOMEIP_MSG_REQUEST = 0x00,
    OPENSOMEIP_MSG_REQUEST_NO_RETURN = 0x01,
    OPENSOMEIP_MSG_NOTIFICATION = 0x02,
    OPENSOMEIP_MSG_TP_REQUEST = 0x20,
    OPENSOMEIP_MSG_TP_REQUEST_NO_RETURN = 0x21,
    OPENSOMEIP_MSG_TP_NOTIFICATION = 0x22,
    OPENSOMEIP_MSG_REQUEST_ACK = 0x40,
    OPENSOMEIP_MSG_RESPONSE = 0x80,
    OPENSOMEIP_MSG_ERROR = 0x81,
    OPENSOMEIP_MSG_RESPONSE_ACK = 0xC0,
    OPENSOMEIP_MSG_ERROR_ACK = 0xC1,
}

// ──────────────────────────────────────────────────────────────────────────
// SOME/IP return codes
// ──────────────────────────────────────────────────────────────────────────

/// @implements REQ_RUST_001
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum opensomeip_return_code_t {
    OPENSOMEIP_RC_E_OK = 0x00,
    OPENSOMEIP_RC_E_NOT_OK = 0x01,
    OPENSOMEIP_RC_E_UNKNOWN_SERVICE = 0x02,
    OPENSOMEIP_RC_E_UNKNOWN_METHOD = 0x03,
    OPENSOMEIP_RC_E_NOT_READY = 0x04,
    OPENSOMEIP_RC_E_NOT_REACHABLE = 0x05,
    OPENSOMEIP_RC_E_TIMEOUT = 0x06,
    OPENSOMEIP_RC_E_WRONG_PROTOCOL_VERSION = 0x07,
    OPENSOMEIP_RC_E_WRONG_INTERFACE_VERSION = 0x08,
    OPENSOMEIP_RC_E_MALFORMED_MESSAGE = 0x09,
    OPENSOMEIP_RC_E_WRONG_MESSAGE_TYPE = 0x0A,
}

// ──────────────────────────────────────────────────────────────────────────
// Transport protocol
// ──────────────────────────────────────────────────────────────────────────

/// @implements REQ_RUST_001
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum opensomeip_transport_protocol_t {
    OPENSOMEIP_TRANSPORT_UDP = 0,
    OPENSOMEIP_TRANSPORT_TCP = 1,
}

// ──────────────────────────────────────────────────────────────────────────
// Opaque handle types
// ──────────────────────────────────────────────────────────────────────────

/// @implements REQ_RUST_001
#[repr(C)]
pub struct opensomeip_message_s {
    _opaque: [u8; 0],
}
pub type opensomeip_message_t = opensomeip_message_s;

#[repr(C)]
pub struct opensomeip_serializer_s {
    _opaque: [u8; 0],
}
pub type opensomeip_serializer_t = opensomeip_serializer_s;

#[repr(C)]
pub struct opensomeip_deserializer_s {
    _opaque: [u8; 0],
}
pub type opensomeip_deserializer_t = opensomeip_deserializer_s;

#[repr(C)]
pub struct opensomeip_udp_transport_s {
    _opaque: [u8; 0],
}
pub type opensomeip_udp_transport_t = opensomeip_udp_transport_s;

#[repr(C)]
pub struct opensomeip_tcp_transport_s {
    _opaque: [u8; 0],
}
pub type opensomeip_tcp_transport_t = opensomeip_tcp_transport_s;

#[repr(C)]
pub struct opensomeip_rpc_client_s {
    _opaque: [u8; 0],
}
pub type opensomeip_rpc_client_t = opensomeip_rpc_client_s;

#[repr(C)]
pub struct opensomeip_rpc_server_s {
    _opaque: [u8; 0],
}
pub type opensomeip_rpc_server_t = opensomeip_rpc_server_s;

#[repr(C)]
pub struct opensomeip_sd_client_s {
    _opaque: [u8; 0],
}
pub type opensomeip_sd_client_t = opensomeip_sd_client_s;

#[repr(C)]
pub struct opensomeip_sd_server_s {
    _opaque: [u8; 0],
}
pub type opensomeip_sd_server_t = opensomeip_sd_server_s;

#[repr(C)]
pub struct opensomeip_event_publisher_s {
    _opaque: [u8; 0],
}
pub type opensomeip_event_publisher_t = opensomeip_event_publisher_s;

#[repr(C)]
pub struct opensomeip_event_subscriber_s {
    _opaque: [u8; 0],
}
pub type opensomeip_event_subscriber_t = opensomeip_event_subscriber_s;

#[repr(C)]
pub struct opensomeip_tp_manager_s {
    _opaque: [u8; 0],
}
pub type opensomeip_tp_manager_t = opensomeip_tp_manager_s;

#[repr(C)]
pub struct opensomeip_e2e_s {
    _opaque: [u8; 0],
}
pub type opensomeip_e2e_t = opensomeip_e2e_s;

// ──────────────────────────────────────────────────────────────────────────
// POD structs
// ──────────────────────────────────────────────────────────────────────────

/// Network endpoint (C-friendly POD).
/// @implements REQ_RUST_001
#[repr(C)]
#[derive(Clone, Copy)]
pub struct opensomeip_endpoint_t {
    pub address: [u8; 64],
    pub port: u16,
    pub protocol: opensomeip_transport_protocol_t,
}

// ──────────────────────────────────────────────────────────────────────────
// Callback types
// ──────────────────────────────────────────────────────────────────────────

/// RPC async completion callback.
/// @implements REQ_RUST_001
pub type opensomeip_rpc_callback_t = Option<
    unsafe extern "C" fn(
        result: opensomeip_result_t,
        return_data: *const u8,
        return_data_len: usize,
        user_data: *mut c_void,
    ),
>;

/// RPC method handler (server side).
/// @implements REQ_RUST_001
pub type opensomeip_method_handler_t = Option<
    unsafe extern "C" fn(
        client_id: u16,
        session_id: u16,
        input_data: *const u8,
        input_len: usize,
        output_data: *mut u8,
        output_len: *mut usize,
        user_data: *mut c_void,
    ) -> opensomeip_result_t,
>;

/// SD service-found callback.
/// @implements REQ_RUST_001
pub type opensomeip_sd_found_callback_t = Option<
    unsafe extern "C" fn(
        service_id: u16,
        instance_id: u16,
        endpoint: *const opensomeip_endpoint_t,
        user_data: *mut c_void,
    ),
>;

/// SD service availability callback.
/// @implements REQ_RUST_001
pub type opensomeip_sd_availability_callback_t = Option<
    unsafe extern "C" fn(service_id: u16, instance_id: u16, available: i32, user_data: *mut c_void),
>;

/// Event notification callback.
/// @implements REQ_RUST_001
pub type opensomeip_event_callback_t = Option<
    unsafe extern "C" fn(
        service_id: u16,
        instance_id: u16,
        event_id: u16,
        data: *const u8,
        data_len: usize,
        user_data: *mut c_void,
    ),
>;

// ──────────────────────────────────────────────────────────────────────────
// Extern "C" function declarations
// ──────────────────────────────────────────────────────────────────────────

extern "C" {
    // ── Version query ────────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_capi_version() -> u32;

    // ── Message API ─────────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_message_create(
        out_msg: *mut *mut opensomeip_message_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_destroy(msg: *mut opensomeip_message_t) -> opensomeip_result_t;

    pub fn opensomeip_message_set_service_id(
        msg: *mut opensomeip_message_t,
        service_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_service_id(
        msg: *const opensomeip_message_t,
        out: *mut u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_set_method_id(
        msg: *mut opensomeip_message_t,
        method_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_method_id(
        msg: *const opensomeip_message_t,
        out: *mut u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_set_client_id(
        msg: *mut opensomeip_message_t,
        client_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_client_id(
        msg: *const opensomeip_message_t,
        out: *mut u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_set_session_id(
        msg: *mut opensomeip_message_t,
        session_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_session_id(
        msg: *const opensomeip_message_t,
        out: *mut u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_set_protocol_version(
        msg: *mut opensomeip_message_t,
        version: u8,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_protocol_version(
        msg: *const opensomeip_message_t,
        out: *mut u8,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_set_interface_version(
        msg: *mut opensomeip_message_t,
        version: u8,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_interface_version(
        msg: *const opensomeip_message_t,
        out: *mut u8,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_set_message_type(
        msg: *mut opensomeip_message_t,
        msg_type: opensomeip_message_type_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_message_type(
        msg: *const opensomeip_message_t,
        out: *mut opensomeip_message_type_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_set_return_code(
        msg: *mut opensomeip_message_t,
        code: opensomeip_return_code_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_return_code(
        msg: *const opensomeip_message_t,
        out: *mut opensomeip_return_code_t,
    ) -> opensomeip_result_t;

    pub fn opensomeip_message_set_payload(
        msg: *mut opensomeip_message_t,
        data: *const u8,
        len: usize,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_payload(
        msg: *const opensomeip_message_t,
        buf: *mut u8,
        out_len: *mut usize,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_get_payload_length(
        msg: *const opensomeip_message_t,
        out_len: *mut usize,
    ) -> opensomeip_result_t;

    pub fn opensomeip_message_serialize(
        msg: *const opensomeip_message_t,
        buf: *mut u8,
        buf_len: *mut usize,
    ) -> opensomeip_result_t;
    pub fn opensomeip_message_deserialize(
        msg: *mut opensomeip_message_t,
        data: *const u8,
        len: usize,
    ) -> opensomeip_result_t;

    // ── Serializer API ──────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_serializer_create(
        out: *mut *mut opensomeip_serializer_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_serializer_destroy(ser: *mut opensomeip_serializer_t) -> opensomeip_result_t;
    pub fn opensomeip_serializer_reset(ser: *mut opensomeip_serializer_t) -> opensomeip_result_t;

    pub fn opensomeip_serializer_write_uint8(
        ser: *mut opensomeip_serializer_t,
        val: u8,
    ) -> opensomeip_result_t;
    pub fn opensomeip_serializer_write_uint16(
        ser: *mut opensomeip_serializer_t,
        val: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_serializer_write_uint32(
        ser: *mut opensomeip_serializer_t,
        val: u32,
    ) -> opensomeip_result_t;
    pub fn opensomeip_serializer_write_uint64(
        ser: *mut opensomeip_serializer_t,
        val: u64,
    ) -> opensomeip_result_t;
    pub fn opensomeip_serializer_write_bytes(
        ser: *mut opensomeip_serializer_t,
        data: *const u8,
        len: usize,
    ) -> opensomeip_result_t;

    pub fn opensomeip_serializer_get_data(
        ser: *const opensomeip_serializer_t,
        buf: *mut u8,
        out_len: *mut usize,
    ) -> opensomeip_result_t;
    pub fn opensomeip_serializer_get_size(
        ser: *const opensomeip_serializer_t,
        out_len: *mut usize,
    ) -> opensomeip_result_t;

    // ── Deserializer API ────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_deserializer_create(
        out: *mut *mut opensomeip_deserializer_t,
        data: *const u8,
        len: usize,
    ) -> opensomeip_result_t;
    pub fn opensomeip_deserializer_destroy(
        de: *mut opensomeip_deserializer_t,
    ) -> opensomeip_result_t;

    pub fn opensomeip_deserializer_read_uint8(
        de: *mut opensomeip_deserializer_t,
        out: *mut u8,
    ) -> opensomeip_result_t;
    pub fn opensomeip_deserializer_read_uint16(
        de: *mut opensomeip_deserializer_t,
        out: *mut u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_deserializer_read_uint32(
        de: *mut opensomeip_deserializer_t,
        out: *mut u32,
    ) -> opensomeip_result_t;
    pub fn opensomeip_deserializer_read_uint64(
        de: *mut opensomeip_deserializer_t,
        out: *mut u64,
    ) -> opensomeip_result_t;
    pub fn opensomeip_deserializer_read_bytes(
        de: *mut opensomeip_deserializer_t,
        buf: *mut u8,
        len: *mut usize,
    ) -> opensomeip_result_t;

    pub fn opensomeip_deserializer_get_remaining(
        de: *const opensomeip_deserializer_t,
        out: *mut usize,
    ) -> opensomeip_result_t;

    // ── UDP Transport API ───────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_udp_transport_create(
        out: *mut *mut opensomeip_udp_transport_t,
        local_ep: *const opensomeip_endpoint_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_udp_transport_destroy(
        t: *mut opensomeip_udp_transport_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_udp_transport_start(
        t: *mut opensomeip_udp_transport_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_udp_transport_stop(t: *mut opensomeip_udp_transport_t)
        -> opensomeip_result_t;
    pub fn opensomeip_udp_transport_send(
        t: *mut opensomeip_udp_transport_t,
        msg: *const opensomeip_message_t,
        dest: *const opensomeip_endpoint_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_udp_transport_receive(
        t: *mut opensomeip_udp_transport_t,
        out_msg: *mut *mut opensomeip_message_t,
        out_sender: *mut opensomeip_endpoint_t,
    ) -> opensomeip_result_t;

    // ── TCP Transport API ───────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_tcp_transport_create(
        out: *mut *mut opensomeip_tcp_transport_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_tcp_transport_destroy(
        t: *mut opensomeip_tcp_transport_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_tcp_transport_initialize(
        t: *mut opensomeip_tcp_transport_t,
        local_ep: *const opensomeip_endpoint_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_tcp_transport_start(
        t: *mut opensomeip_tcp_transport_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_tcp_transport_stop(t: *mut opensomeip_tcp_transport_t)
        -> opensomeip_result_t;
    pub fn opensomeip_tcp_transport_connect(
        t: *mut opensomeip_tcp_transport_t,
        remote_ep: *const opensomeip_endpoint_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_tcp_transport_disconnect(
        t: *mut opensomeip_tcp_transport_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_tcp_transport_send(
        t: *mut opensomeip_tcp_transport_t,
        msg: *const opensomeip_message_t,
        dest: *const opensomeip_endpoint_t,
    ) -> opensomeip_result_t;

    // ── RPC Client API ──────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_rpc_client_create(
        out: *mut *mut opensomeip_rpc_client_t,
        client_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_rpc_client_destroy(c: *mut opensomeip_rpc_client_t) -> opensomeip_result_t;
    pub fn opensomeip_rpc_client_initialize(c: *mut opensomeip_rpc_client_t)
        -> opensomeip_result_t;
    pub fn opensomeip_rpc_client_shutdown(c: *mut opensomeip_rpc_client_t) -> opensomeip_result_t;

    pub fn opensomeip_rpc_client_call_sync(
        c: *mut opensomeip_rpc_client_t,
        service_id: u16,
        method_id: u16,
        input_data: *const u8,
        input_len: usize,
        output_data: *mut u8,
        output_len: *mut usize,
        timeout_ms: u32,
    ) -> opensomeip_result_t;

    pub fn opensomeip_rpc_client_call_async(
        c: *mut opensomeip_rpc_client_t,
        service_id: u16,
        method_id: u16,
        input_data: *const u8,
        input_len: usize,
        callback: opensomeip_rpc_callback_t,
        user_data: *mut c_void,
        timeout_ms: u32,
        out_handle: *mut u32,
    ) -> opensomeip_result_t;

    pub fn opensomeip_rpc_client_cancel(
        c: *mut opensomeip_rpc_client_t,
        handle: u32,
    ) -> opensomeip_result_t;

    // ── RPC Server API ──────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_rpc_server_create(
        out: *mut *mut opensomeip_rpc_server_t,
        service_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_rpc_server_destroy(s: *mut opensomeip_rpc_server_t) -> opensomeip_result_t;
    pub fn opensomeip_rpc_server_initialize(s: *mut opensomeip_rpc_server_t)
        -> opensomeip_result_t;
    pub fn opensomeip_rpc_server_shutdown(s: *mut opensomeip_rpc_server_t) -> opensomeip_result_t;

    pub fn opensomeip_rpc_server_register_method(
        s: *mut opensomeip_rpc_server_t,
        method_id: u16,
        handler: opensomeip_method_handler_t,
        user_data: *mut c_void,
    ) -> opensomeip_result_t;

    pub fn opensomeip_rpc_server_unregister_method(
        s: *mut opensomeip_rpc_server_t,
        method_id: u16,
    ) -> opensomeip_result_t;

    // ── SD Client API ───────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_sd_client_create(
        out: *mut *mut opensomeip_sd_client_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_sd_client_destroy(c: *mut opensomeip_sd_client_t) -> opensomeip_result_t;
    pub fn opensomeip_sd_client_initialize(c: *mut opensomeip_sd_client_t) -> opensomeip_result_t;
    pub fn opensomeip_sd_client_shutdown(c: *mut opensomeip_sd_client_t) -> opensomeip_result_t;

    pub fn opensomeip_sd_client_find_service(
        c: *mut opensomeip_sd_client_t,
        service_id: u16,
        callback: opensomeip_sd_found_callback_t,
        user_data: *mut c_void,
        timeout_ms: u32,
    ) -> opensomeip_result_t;

    pub fn opensomeip_sd_client_subscribe_availability(
        c: *mut opensomeip_sd_client_t,
        service_id: u16,
        callback: opensomeip_sd_availability_callback_t,
        user_data: *mut c_void,
    ) -> opensomeip_result_t;

    pub fn opensomeip_sd_client_subscribe_eventgroup(
        c: *mut opensomeip_sd_client_t,
        service_id: u16,
        instance_id: u16,
        eventgroup_id: u16,
    ) -> opensomeip_result_t;

    // ── SD Server API ───────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_sd_server_create(
        out: *mut *mut opensomeip_sd_server_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_sd_server_destroy(s: *mut opensomeip_sd_server_t) -> opensomeip_result_t;
    pub fn opensomeip_sd_server_initialize(s: *mut opensomeip_sd_server_t) -> opensomeip_result_t;
    pub fn opensomeip_sd_server_shutdown(s: *mut opensomeip_sd_server_t) -> opensomeip_result_t;

    pub fn opensomeip_sd_server_offer_service(
        s: *mut opensomeip_sd_server_t,
        service_id: u16,
        instance_id: u16,
        endpoint: *const opensomeip_endpoint_t,
    ) -> opensomeip_result_t;

    pub fn opensomeip_sd_server_stop_offer(
        s: *mut opensomeip_sd_server_t,
        service_id: u16,
        instance_id: u16,
    ) -> opensomeip_result_t;

    // ── Event Publisher API ─────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_event_publisher_create(
        out: *mut *mut opensomeip_event_publisher_t,
        service_id: u16,
        instance_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_event_publisher_destroy(
        p: *mut opensomeip_event_publisher_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_event_publisher_initialize(
        p: *mut opensomeip_event_publisher_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_event_publisher_shutdown(
        p: *mut opensomeip_event_publisher_t,
    ) -> opensomeip_result_t;

    pub fn opensomeip_event_publisher_register(
        p: *mut opensomeip_event_publisher_t,
        event_id: u16,
        eventgroup_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_event_publisher_unregister(
        p: *mut opensomeip_event_publisher_t,
        event_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_event_publisher_notify(
        p: *mut opensomeip_event_publisher_t,
        event_id: u16,
        data: *const u8,
        len: usize,
    ) -> opensomeip_result_t;

    // ── Event Subscriber API ────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_event_subscriber_create(
        out: *mut *mut opensomeip_event_subscriber_t,
        client_id: u16,
    ) -> opensomeip_result_t;
    pub fn opensomeip_event_subscriber_destroy(
        s: *mut opensomeip_event_subscriber_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_event_subscriber_initialize(
        s: *mut opensomeip_event_subscriber_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_event_subscriber_shutdown(
        s: *mut opensomeip_event_subscriber_t,
    ) -> opensomeip_result_t;

    pub fn opensomeip_event_subscriber_subscribe(
        s: *mut opensomeip_event_subscriber_t,
        service_id: u16,
        instance_id: u16,
        eventgroup_id: u16,
        callback: opensomeip_event_callback_t,
        user_data: *mut c_void,
    ) -> opensomeip_result_t;

    pub fn opensomeip_event_subscriber_unsubscribe(
        s: *mut opensomeip_event_subscriber_t,
        service_id: u16,
        instance_id: u16,
        eventgroup_id: u16,
    ) -> opensomeip_result_t;

    // ── TP Manager API ──────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_tp_manager_create(
        out: *mut *mut opensomeip_tp_manager_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_tp_manager_destroy(tp: *mut opensomeip_tp_manager_t) -> opensomeip_result_t;
    pub fn opensomeip_tp_manager_initialize(
        tp: *mut opensomeip_tp_manager_t,
    ) -> opensomeip_result_t;
    pub fn opensomeip_tp_manager_shutdown(tp: *mut opensomeip_tp_manager_t) -> opensomeip_result_t;

    pub fn opensomeip_tp_needs_segmentation(
        tp: *const opensomeip_tp_manager_t,
        payload: *const u8,
        len: usize,
        out_needs: *mut i32,
    ) -> opensomeip_result_t;

    pub fn opensomeip_tp_segment(
        tp: *mut opensomeip_tp_manager_t,
        msg: *const opensomeip_message_t,
        out_buf: *mut u8,
        out_len: *mut usize,
    ) -> opensomeip_result_t;

    pub fn opensomeip_tp_reassemble(
        tp: *mut opensomeip_tp_manager_t,
        segment_data: *const u8,
        segment_len: usize,
        out_buf: *mut u8,
        out_len: *mut usize,
        complete: *mut i32,
    ) -> opensomeip_result_t;

    // ── E2E API ─────────────────────────────────────────────────────────

    /// @implements REQ_RUST_001
    pub fn opensomeip_e2e_create(out: *mut *mut opensomeip_e2e_t) -> opensomeip_result_t;
    pub fn opensomeip_e2e_destroy(e: *mut opensomeip_e2e_t) -> opensomeip_result_t;

    pub fn opensomeip_e2e_protect(
        e: *mut opensomeip_e2e_t,
        msg: *mut opensomeip_message_t,
        data_id: u16,
        counter: u32,
    ) -> opensomeip_result_t;

    pub fn opensomeip_e2e_check(
        e: *mut opensomeip_e2e_t,
        msg: *const opensomeip_message_t,
        data_id: u16,
    ) -> opensomeip_result_t;
}

// ──────────────────────────────────────────────────────────────────────────
// Compile-time assertions
// ──────────────────────────────────────────────────────────────────────────

/// @tests REQ_RUST_001
#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    #[test]
    fn result_enum_size() {
        // C enum is int (typically 4 bytes)
        assert_eq!(mem::size_of::<opensomeip_result_t>(), 4);
    }

    #[test]
    fn message_type_enum_size() {
        assert_eq!(mem::size_of::<opensomeip_message_type_t>(), 4);
    }

    #[test]
    fn return_code_enum_size() {
        assert_eq!(mem::size_of::<opensomeip_return_code_t>(), 4);
    }

    #[test]
    fn transport_protocol_enum_size() {
        assert_eq!(mem::size_of::<opensomeip_transport_protocol_t>(), 4);
    }

    #[test]
    fn endpoint_layout() {
        // address[64] + u16 port + 2 bytes padding + transport_protocol (4 bytes)
        // On most platforms: 64 + 2 + 2(pad) + 4 = 72
        let size = mem::size_of::<opensomeip_endpoint_t>();
        assert!(size >= 68, "endpoint_t too small: {size}");
        assert!(size <= 72, "endpoint_t too large: {size}");
    }

    #[test]
    fn version_constants_match() {
        assert_eq!(OPENSOMEIP_CAPI_VERSION_MAJOR, 0);
        assert_eq!(OPENSOMEIP_CAPI_VERSION_MINOR, 1);
        assert_eq!(OPENSOMEIP_CAPI_VERSION_PATCH, 0);
    }

    #[test]
    fn result_discriminant_values() {
        assert_eq!(opensomeip_result_t::OPENSOMEIP_RESULT_SUCCESS as i32, 0x00);
        assert_eq!(
            opensomeip_result_t::OPENSOMEIP_RESULT_NETWORK_ERROR as i32,
            0x01
        );
        assert_eq!(opensomeip_result_t::OPENSOMEIP_RESULT_TIMEOUT as i32, 0x05);
        assert_eq!(
            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_MESSAGE as i32,
            0x10
        );
        assert_eq!(
            opensomeip_result_t::OPENSOMEIP_RESULT_OUT_OF_MEMORY as i32,
            0x30
        );
        assert_eq!(
            opensomeip_result_t::OPENSOMEIP_RESULT_UNKNOWN_ERROR as i32,
            0xFF
        );
    }

    #[test]
    fn opaque_types_are_zero_sized() {
        assert_eq!(mem::size_of::<opensomeip_message_t>(), 0);
        assert_eq!(mem::size_of::<opensomeip_serializer_t>(), 0);
        assert_eq!(mem::size_of::<opensomeip_deserializer_t>(), 0);
        assert_eq!(mem::size_of::<opensomeip_udp_transport_t>(), 0);
        assert_eq!(mem::size_of::<opensomeip_tcp_transport_t>(), 0);
    }

    #[test]
    fn callback_types_are_pointer_sized() {
        assert_eq!(
            mem::size_of::<opensomeip_rpc_callback_t>(),
            mem::size_of::<usize>()
        );
        assert_eq!(
            mem::size_of::<opensomeip_method_handler_t>(),
            mem::size_of::<usize>()
        );
        assert_eq!(
            mem::size_of::<opensomeip_sd_found_callback_t>(),
            mem::size_of::<usize>()
        );
        assert_eq!(
            mem::size_of::<opensomeip_event_callback_t>(),
            mem::size_of::<usize>()
        );
    }
}
