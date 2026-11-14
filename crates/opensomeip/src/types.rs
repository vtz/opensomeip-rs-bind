// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! Safe Rust types mirroring the C API enums and structs.
//!
//! @implements REQ_RUST_003, REQ_RUST_004, REQ_RUST_007

use opensomeip_sys::*;

// ──────────────────────────────────────────────────────────────────────────
// MessageType
// ──────────────────────────────────────────────────────────────────────────

/// SOME/IP message type.
///
/// @implements REQ_RUST_004
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Request,
    RequestNoReturn,
    Notification,
    RequestAck,
    Response,
    Error,
    ResponseAck,
    ErrorAck,
    TpRequest,
    TpRequestNoReturn,
    TpNotification,
}

impl From<opensomeip_message_type_t> for MessageType {
    fn from(raw: opensomeip_message_type_t) -> Self {
        match raw {
            opensomeip_message_type_t::OPENSOMEIP_MSG_REQUEST => Self::Request,
            opensomeip_message_type_t::OPENSOMEIP_MSG_REQUEST_NO_RETURN => Self::RequestNoReturn,
            opensomeip_message_type_t::OPENSOMEIP_MSG_NOTIFICATION => Self::Notification,
            opensomeip_message_type_t::OPENSOMEIP_MSG_REQUEST_ACK => Self::RequestAck,
            opensomeip_message_type_t::OPENSOMEIP_MSG_RESPONSE => Self::Response,
            opensomeip_message_type_t::OPENSOMEIP_MSG_ERROR => Self::Error,
            opensomeip_message_type_t::OPENSOMEIP_MSG_RESPONSE_ACK => Self::ResponseAck,
            opensomeip_message_type_t::OPENSOMEIP_MSG_ERROR_ACK => Self::ErrorAck,
            opensomeip_message_type_t::OPENSOMEIP_MSG_TP_REQUEST => Self::TpRequest,
            opensomeip_message_type_t::OPENSOMEIP_MSG_TP_REQUEST_NO_RETURN => {
                Self::TpRequestNoReturn
            }
            opensomeip_message_type_t::OPENSOMEIP_MSG_TP_NOTIFICATION => Self::TpNotification,
        }
    }
}

impl From<MessageType> for opensomeip_message_type_t {
    fn from(t: MessageType) -> Self {
        match t {
            MessageType::Request => Self::OPENSOMEIP_MSG_REQUEST,
            MessageType::RequestNoReturn => Self::OPENSOMEIP_MSG_REQUEST_NO_RETURN,
            MessageType::Notification => Self::OPENSOMEIP_MSG_NOTIFICATION,
            MessageType::RequestAck => Self::OPENSOMEIP_MSG_REQUEST_ACK,
            MessageType::Response => Self::OPENSOMEIP_MSG_RESPONSE,
            MessageType::Error => Self::OPENSOMEIP_MSG_ERROR,
            MessageType::ResponseAck => Self::OPENSOMEIP_MSG_RESPONSE_ACK,
            MessageType::ErrorAck => Self::OPENSOMEIP_MSG_ERROR_ACK,
            MessageType::TpRequest => Self::OPENSOMEIP_MSG_TP_REQUEST,
            MessageType::TpRequestNoReturn => Self::OPENSOMEIP_MSG_TP_REQUEST_NO_RETURN,
            MessageType::TpNotification => Self::OPENSOMEIP_MSG_TP_NOTIFICATION,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// ReturnCode
// ──────────────────────────────────────────────────────────────────────────

/// SOME/IP return code.
///
/// @implements REQ_RUST_004
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnCode {
    Ok,
    NotOk,
    UnknownService,
    UnknownMethod,
    NotReady,
    NotReachable,
    Timeout,
    WrongProtocolVersion,
    WrongInterfaceVersion,
    MalformedMessage,
    WrongMessageType,
}

impl From<opensomeip_return_code_t> for ReturnCode {
    fn from(raw: opensomeip_return_code_t) -> Self {
        match raw {
            opensomeip_return_code_t::OPENSOMEIP_RC_E_OK => Self::Ok,
            opensomeip_return_code_t::OPENSOMEIP_RC_E_NOT_OK => Self::NotOk,
            opensomeip_return_code_t::OPENSOMEIP_RC_E_UNKNOWN_SERVICE => Self::UnknownService,
            opensomeip_return_code_t::OPENSOMEIP_RC_E_UNKNOWN_METHOD => Self::UnknownMethod,
            opensomeip_return_code_t::OPENSOMEIP_RC_E_NOT_READY => Self::NotReady,
            opensomeip_return_code_t::OPENSOMEIP_RC_E_NOT_REACHABLE => Self::NotReachable,
            opensomeip_return_code_t::OPENSOMEIP_RC_E_TIMEOUT => Self::Timeout,
            opensomeip_return_code_t::OPENSOMEIP_RC_E_WRONG_PROTOCOL_VERSION => {
                Self::WrongProtocolVersion
            }
            opensomeip_return_code_t::OPENSOMEIP_RC_E_WRONG_INTERFACE_VERSION => {
                Self::WrongInterfaceVersion
            }
            opensomeip_return_code_t::OPENSOMEIP_RC_E_MALFORMED_MESSAGE => Self::MalformedMessage,
            opensomeip_return_code_t::OPENSOMEIP_RC_E_WRONG_MESSAGE_TYPE => Self::WrongMessageType,
        }
    }
}

impl From<ReturnCode> for opensomeip_return_code_t {
    fn from(c: ReturnCode) -> Self {
        match c {
            ReturnCode::Ok => Self::OPENSOMEIP_RC_E_OK,
            ReturnCode::NotOk => Self::OPENSOMEIP_RC_E_NOT_OK,
            ReturnCode::UnknownService => Self::OPENSOMEIP_RC_E_UNKNOWN_SERVICE,
            ReturnCode::UnknownMethod => Self::OPENSOMEIP_RC_E_UNKNOWN_METHOD,
            ReturnCode::NotReady => Self::OPENSOMEIP_RC_E_NOT_READY,
            ReturnCode::NotReachable => Self::OPENSOMEIP_RC_E_NOT_REACHABLE,
            ReturnCode::Timeout => Self::OPENSOMEIP_RC_E_TIMEOUT,
            ReturnCode::WrongProtocolVersion => Self::OPENSOMEIP_RC_E_WRONG_PROTOCOL_VERSION,
            ReturnCode::WrongInterfaceVersion => Self::OPENSOMEIP_RC_E_WRONG_INTERFACE_VERSION,
            ReturnCode::MalformedMessage => Self::OPENSOMEIP_RC_E_MALFORMED_MESSAGE,
            ReturnCode::WrongMessageType => Self::OPENSOMEIP_RC_E_WRONG_MESSAGE_TYPE,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// TransportProtocol
// ──────────────────────────────────────────────────────────────────────────

/// Transport protocol selection.
///
/// @implements REQ_RUST_004
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportProtocol {
    Udp,
    Tcp,
}

impl From<opensomeip_transport_protocol_t> for TransportProtocol {
    fn from(raw: opensomeip_transport_protocol_t) -> Self {
        match raw {
            opensomeip_transport_protocol_t::OPENSOMEIP_TRANSPORT_UDP => Self::Udp,
            opensomeip_transport_protocol_t::OPENSOMEIP_TRANSPORT_TCP => Self::Tcp,
        }
    }
}

impl From<TransportProtocol> for opensomeip_transport_protocol_t {
    fn from(p: TransportProtocol) -> Self {
        match p {
            TransportProtocol::Udp => Self::OPENSOMEIP_TRANSPORT_UDP,
            TransportProtocol::Tcp => Self::OPENSOMEIP_TRANSPORT_TCP,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Endpoint
// ──────────────────────────────────────────────────────────────────────────

/// Network endpoint (address + port + protocol).
///
/// @implements REQ_RUST_004
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub address: [u8; 64],
    pub port: u16,
    pub protocol: TransportProtocol,
}

impl Endpoint {
    /// Create an endpoint from an address string, port, and protocol.
    ///
    /// The address is truncated to 63 bytes (null-terminated in C).
    pub fn new(address: &str, port: u16, protocol: TransportProtocol) -> Self {
        let mut addr_buf = [0u8; 64];
        let bytes = address.as_bytes();
        let copy_len = bytes.len().min(63);
        addr_buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
        Self {
            address: addr_buf,
            port,
            protocol,
        }
    }

    /// Convert to the C API endpoint struct.
    pub(crate) fn to_raw(&self) -> opensomeip_endpoint_t {
        opensomeip_endpoint_t {
            address: self.address,
            port: self.port,
            protocol: self.protocol.into(),
        }
    }

    /// Convert from the C API endpoint struct.
    pub(crate) fn from_raw(raw: &opensomeip_endpoint_t) -> Self {
        Self {
            address: raw.address,
            port: raw.port,
            protocol: TransportProtocol::from(raw.protocol),
        }
    }

    /// Get the address as a string slice (up to the first null byte).
    pub fn address_str(&self) -> &str {
        let end = self.address.iter().position(|&b| b == 0).unwrap_or(64);
        core::str::from_utf8(&self.address[..end]).unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @tests REQ_RUST_004
    #[test]
    fn message_type_roundtrip() {
        let mt = MessageType::Response;
        let raw: opensomeip_message_type_t = mt.into();
        let back = MessageType::from(raw);
        assert_eq!(mt, back);
    }

    /// @tests REQ_RUST_004
    #[test]
    fn return_code_roundtrip() {
        let rc = ReturnCode::Timeout;
        let raw: opensomeip_return_code_t = rc.into();
        let back = ReturnCode::from(raw);
        assert_eq!(rc, back);
    }

    /// @tests REQ_RUST_004
    #[test]
    fn endpoint_construction() {
        let ep = Endpoint::new("192.168.1.1", 30490, TransportProtocol::Udp);
        assert_eq!(ep.address_str(), "192.168.1.1");
        assert_eq!(ep.port, 30490);
        assert_eq!(ep.protocol, TransportProtocol::Udp);
    }

    /// @tests REQ_RUST_004
    #[test]
    fn endpoint_address_truncation() {
        let long_addr = "a]".repeat(64);
        let ep = Endpoint::new(&long_addr, 80, TransportProtocol::Tcp);
        assert!(ep.address_str().len() <= 63);
    }
}
