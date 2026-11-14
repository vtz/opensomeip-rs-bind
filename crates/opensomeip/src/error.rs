// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! Error types for the opensomeip safe wrapper.
//!
//! @implements REQ_RUST_003, REQ_RUST_007

use opensomeip_sys::opensomeip_result_t;

/// Error type mapping all non-success `opensomeip_result_t` variants.
///
/// @implements REQ_RUST_003
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SomeIpError {
    // Network errors
    NetworkError,
    NotConnected,
    ConnectionLost,
    ConnectionRefused,
    Timeout,
    InvalidEndpoint,

    // Message errors
    InvalidMessage,
    InvalidMessageType,
    InvalidServiceId,
    InvalidMethodId,
    InvalidProtocolVersion,
    InvalidInterfaceVersion,
    MalformedMessage,

    // Session errors
    InvalidSessionId,
    SessionExpired,
    SessionNotFound,

    // Resource errors
    OutOfMemory,
    BufferOverflow,
    ResourceExhausted,

    // Service errors
    ServiceNotFound,
    ServiceUnavailable,
    SubscriptionFailed,

    // Safety errors
    SafetyViolation,
    FaultDetected,
    RecoveryFailed,

    // General errors
    NotImplemented,
    InvalidArgument,
    PermissionDenied,
    InternalError,
    NotInitialized,
    InvalidState,
    UnknownError,
}

impl SomeIpError {
    /// Convert a raw `opensomeip_result_t` to `Result<(), SomeIpError>`.
    ///
    /// @implements REQ_RUST_003
    #[inline]
    pub fn from_result(result: opensomeip_result_t) -> core::result::Result<(), Self> {
        match result {
            opensomeip_result_t::OPENSOMEIP_RESULT_SUCCESS => Ok(()),

            opensomeip_result_t::OPENSOMEIP_RESULT_NETWORK_ERROR => Err(Self::NetworkError),
            opensomeip_result_t::OPENSOMEIP_RESULT_NOT_CONNECTED => Err(Self::NotConnected),
            opensomeip_result_t::OPENSOMEIP_RESULT_CONNECTION_LOST => Err(Self::ConnectionLost),
            opensomeip_result_t::OPENSOMEIP_RESULT_CONNECTION_REFUSED => {
                Err(Self::ConnectionRefused)
            }
            opensomeip_result_t::OPENSOMEIP_RESULT_TIMEOUT => Err(Self::Timeout),
            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_ENDPOINT => Err(Self::InvalidEndpoint),

            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_MESSAGE => Err(Self::InvalidMessage),
            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_MESSAGE_TYPE => {
                Err(Self::InvalidMessageType)
            }
            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_SERVICE_ID => {
                Err(Self::InvalidServiceId)
            }
            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_METHOD_ID => Err(Self::InvalidMethodId),
            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_PROTOCOL_VERSION => {
                Err(Self::InvalidProtocolVersion)
            }
            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_INTERFACE_VERSION => {
                Err(Self::InvalidInterfaceVersion)
            }
            opensomeip_result_t::OPENSOMEIP_RESULT_MALFORMED_MESSAGE => Err(Self::MalformedMessage),

            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_SESSION_ID => {
                Err(Self::InvalidSessionId)
            }
            opensomeip_result_t::OPENSOMEIP_RESULT_SESSION_EXPIRED => Err(Self::SessionExpired),
            opensomeip_result_t::OPENSOMEIP_RESULT_SESSION_NOT_FOUND => Err(Self::SessionNotFound),

            opensomeip_result_t::OPENSOMEIP_RESULT_OUT_OF_MEMORY => Err(Self::OutOfMemory),
            opensomeip_result_t::OPENSOMEIP_RESULT_BUFFER_OVERFLOW => Err(Self::BufferOverflow),
            opensomeip_result_t::OPENSOMEIP_RESULT_RESOURCE_EXHAUSTED => {
                Err(Self::ResourceExhausted)
            }

            opensomeip_result_t::OPENSOMEIP_RESULT_SERVICE_NOT_FOUND => Err(Self::ServiceNotFound),
            opensomeip_result_t::OPENSOMEIP_RESULT_SERVICE_UNAVAILABLE => {
                Err(Self::ServiceUnavailable)
            }
            opensomeip_result_t::OPENSOMEIP_RESULT_SUBSCRIPTION_FAILED => {
                Err(Self::SubscriptionFailed)
            }

            opensomeip_result_t::OPENSOMEIP_RESULT_SAFETY_VIOLATION => Err(Self::SafetyViolation),
            opensomeip_result_t::OPENSOMEIP_RESULT_FAULT_DETECTED => Err(Self::FaultDetected),
            opensomeip_result_t::OPENSOMEIP_RESULT_RECOVERY_FAILED => Err(Self::RecoveryFailed),

            opensomeip_result_t::OPENSOMEIP_RESULT_NOT_IMPLEMENTED => Err(Self::NotImplemented),
            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_ARGUMENT => Err(Self::InvalidArgument),
            opensomeip_result_t::OPENSOMEIP_RESULT_PERMISSION_DENIED => Err(Self::PermissionDenied),
            opensomeip_result_t::OPENSOMEIP_RESULT_INTERNAL_ERROR => Err(Self::InternalError),
            opensomeip_result_t::OPENSOMEIP_RESULT_NOT_INITIALIZED => Err(Self::NotInitialized),
            opensomeip_result_t::OPENSOMEIP_RESULT_INVALID_STATE => Err(Self::InvalidState),

            opensomeip_result_t::OPENSOMEIP_RESULT_UNKNOWN_ERROR => Err(Self::UnknownError),
        }
    }
}

impl core::fmt::Display for SomeIpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NetworkError => write!(f, "network error"),
            Self::NotConnected => write!(f, "not connected"),
            Self::ConnectionLost => write!(f, "connection lost"),
            Self::ConnectionRefused => write!(f, "connection refused"),
            Self::Timeout => write!(f, "timeout"),
            Self::InvalidEndpoint => write!(f, "invalid endpoint"),
            Self::InvalidMessage => write!(f, "invalid message"),
            Self::InvalidMessageType => write!(f, "invalid message type"),
            Self::InvalidServiceId => write!(f, "invalid service ID"),
            Self::InvalidMethodId => write!(f, "invalid method ID"),
            Self::InvalidProtocolVersion => write!(f, "invalid protocol version"),
            Self::InvalidInterfaceVersion => write!(f, "invalid interface version"),
            Self::MalformedMessage => write!(f, "malformed message"),
            Self::InvalidSessionId => write!(f, "invalid session ID"),
            Self::SessionExpired => write!(f, "session expired"),
            Self::SessionNotFound => write!(f, "session not found"),
            Self::OutOfMemory => write!(f, "out of memory"),
            Self::BufferOverflow => write!(f, "buffer overflow"),
            Self::ResourceExhausted => write!(f, "resource exhausted"),
            Self::ServiceNotFound => write!(f, "service not found"),
            Self::ServiceUnavailable => write!(f, "service unavailable"),
            Self::SubscriptionFailed => write!(f, "subscription failed"),
            Self::SafetyViolation => write!(f, "safety violation"),
            Self::FaultDetected => write!(f, "fault detected"),
            Self::RecoveryFailed => write!(f, "recovery failed"),
            Self::NotImplemented => write!(f, "not implemented"),
            Self::InvalidArgument => write!(f, "invalid argument"),
            Self::PermissionDenied => write!(f, "permission denied"),
            Self::InternalError => write!(f, "internal error"),
            Self::NotInitialized => write!(f, "not initialized"),
            Self::InvalidState => write!(f, "invalid state"),
            Self::UnknownError => write!(f, "unknown error"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SomeIpError {}

/// Convenience type alias.
pub type Result<T> = core::result::Result<T, SomeIpError>;

/// Helper: call an FFI function returning `opensomeip_result_t` and convert.
#[inline]
pub(crate) fn check(result: opensomeip_result_t) -> Result<()> {
    SomeIpError::from_result(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @tests REQ_RUST_003
    #[test]
    fn success_maps_to_ok() {
        assert!(SomeIpError::from_result(opensomeip_result_t::OPENSOMEIP_RESULT_SUCCESS).is_ok());
    }

    /// @tests REQ_RUST_003
    #[test]
    fn error_codes_map_to_err() {
        assert_eq!(
            SomeIpError::from_result(opensomeip_result_t::OPENSOMEIP_RESULT_TIMEOUT),
            Err(SomeIpError::Timeout),
        );
        assert_eq!(
            SomeIpError::from_result(opensomeip_result_t::OPENSOMEIP_RESULT_OUT_OF_MEMORY),
            Err(SomeIpError::OutOfMemory),
        );
    }

    /// @tests REQ_RUST_003
    #[test]
    fn display_impl() {
        let err = SomeIpError::Timeout;
        assert_eq!(format!("{err}"), "timeout");
    }
}
