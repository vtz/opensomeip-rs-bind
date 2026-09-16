// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! Integration tests for opensomeip Rust bindings.
//!
//! These tests require the opensomeip C library to be linked.
//! Set `OPENSOMEIP_DIR` to the install prefix before running.
//!
//! @tests REQ_RUST_001, REQ_RUST_002, REQ_RUST_003, REQ_RUST_004

use opensomeip::*;

/// @tests REQ_RUST_001
#[test]
fn version_query() {
    let (major, minor, patch) = capi_version_tuple();
    assert_eq!(major, 0);
    assert_eq!(minor, 1);
    assert_eq!(patch, 0);
}

/// @tests REQ_RUST_002, REQ_RUST_003
#[test]
fn message_create_and_drop() {
    let msg = SomeIpMessage::new().expect("failed to create message");
    drop(msg);
}

/// @tests REQ_RUST_002, REQ_RUST_004
#[test]
fn message_builder_pattern() {
    let msg = SomeIpMessageBuilder::new()
        .expect("create")
        .service_id(0x1234)
        .expect("service_id")
        .method_id(0x0001)
        .expect("method_id")
        .client_id(0x0010)
        .expect("client_id")
        .session_id(0x0001)
        .expect("session_id")
        .message_type(MessageType::Request)
        .expect("message_type")
        .return_code(ReturnCode::Ok)
        .expect("return_code")
        .payload(&[0x01, 0x02, 0x03])
        .expect("payload")
        .build();

    assert_eq!(msg.service_id().unwrap(), 0x1234);
    assert_eq!(msg.method_id().unwrap(), 0x0001);
    assert_eq!(msg.payload_length().unwrap(), 3);
}

/// @tests REQ_RUST_002, REQ_RUST_003
#[test]
fn message_payload_roundtrip() {
    let mut msg = SomeIpMessage::new().unwrap();
    let payload = b"hello SOME/IP";
    msg.set_payload(payload).unwrap();

    let mut buf = [0u8; 64];
    let len = msg.get_payload(&mut buf).unwrap();
    assert_eq!(&buf[..len], payload);
}

/// @tests REQ_RUST_002
#[test]
fn serializer_basic() {
    let mut ser = Serializer::new().unwrap();
    ser.write_u8(0x42).unwrap();
    ser.write_u16(0x1234).unwrap();
    ser.write_u32(0xDEADBEEF).unwrap();

    let size = ser.size().unwrap();
    assert!(size > 0);

    let data = ser.to_vec().unwrap();
    assert_eq!(data.len(), size);
}

/// @tests REQ_RUST_002
#[test]
fn deserializer_basic() {
    let mut ser = Serializer::new().unwrap();
    ser.write_u8(0x42).unwrap();
    ser.write_u16(0x1234).unwrap();
    let data = ser.to_vec().unwrap();

    let mut de = Deserializer::new(&data).unwrap();
    assert_eq!(de.read_u8().unwrap(), 0x42);
    assert_eq!(de.read_u16().unwrap(), 0x1234);
    assert_eq!(de.remaining().unwrap(), 0);
}

/// @tests REQ_RUST_002, REQ_RUST_004
#[test]
fn udp_loopback() {
    let ep = Endpoint::new("127.0.0.1", 30490, TransportProtocol::Udp);
    let mut transport = UdpTransport::new(&ep).expect("create UDP transport");
    transport.start().expect("start transport");

    let mut msg = SomeIpMessageBuilder::new()
        .unwrap()
        .service_id(0x1234)
        .unwrap()
        .method_id(0x0001)
        .unwrap()
        .message_type(MessageType::Request)
        .unwrap()
        .payload(b"ping")
        .unwrap()
        .build();

    let dest = Endpoint::new("127.0.0.1", 30490, TransportProtocol::Udp);
    transport.send(&msg, &dest).expect("send message");

    // Receive the loopback message
    let (recv_msg, sender) = transport.receive().expect("receive message");
    assert_eq!(recv_msg.service_id().unwrap(), 0x1234);
    assert_eq!(sender.address_str(), "127.0.0.1");

    transport.stop().expect("stop transport");
}
