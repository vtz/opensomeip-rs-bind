.. Copyright (c) 2025 Vinicius Tadeu Zein
.. SPDX-License-Identifier: Apache-2.0

===================================
Rust Bindings Requirements
===================================

This document defines the requirements for the ``opensomeip-rs-bind``
project — the Rust language bindings to the OpenSOME/IP C API.

Requirements are traceable from this document to implementation via
``@implements`` annotations in source code and ``@tests`` annotations
in test code.

.. contents:: Table of Contents
   :local:

REQ_RUST_001 — Raw FFI Bindings Match C ABI Header
====================================================

**ID:** REQ_RUST_001

**Description:**
The ``opensomeip-sys`` crate SHALL provide hand-written ``extern "C"``
declarations that exactly match the public C ABI header
``include/capi/opensomeip.h`` from the opensomeip project.

**Rationale:**
FFI correctness requires that Rust declarations match the C ABI in
function signatures, type sizes, alignment, and calling convention.
Hand-written bindings (no build-time bindgen) ensure Ferrocene
compatibility and auditability.

**Acceptance Criteria:**

- Every public function, enum, struct, and callback type in
  ``opensomeip.h`` has a corresponding Rust declaration.
- Version constants (``OPENSOMEIP_CAPI_VERSION_*``) are mirrored.
- Unit tests verify type sizes and alignment match C expectations.

REQ_RUST_002 — Safe Wrappers Enforce Ownership via RAII
========================================================

**ID:** REQ_RUST_002

**Description:**
The ``opensomeip`` crate SHALL wrap each opaque C handle in a Rust
struct that implements ``Drop`` to call the corresponding
``_destroy`` function, ensuring deterministic resource cleanup.

**Rationale:**
RAII prevents resource leaks and double-frees at the type level.

**Acceptance Criteria:**

- Every opaque handle type (``opensomeip_message_t*``,
  ``opensomeip_udp_transport_t*``, etc.) has a Rust wrapper with
  ``Drop``.
- Wrapper types are non-copyable (no ``Clone`` unless deep-clone is
  implemented via the C API).
- Builder patterns return ``Result`` on fallible construction.

REQ_RUST_003 — Error Mapping from opensomeip_result_t to Result
================================================================

**ID:** REQ_RUST_003

**Description:**
The ``opensomeip`` crate SHALL define a ``SomeIpError`` enum that maps
every non-success ``opensomeip_result_t`` variant. All fallible C API
calls SHALL be wrapped in functions returning ``Result<T, SomeIpError>``.

**Rationale:**
Idiomatic Rust error handling prevents silent failures and enables
``?`` propagation.

**Acceptance Criteria:**

- ``SomeIpError`` covers all ``OPENSOMEIP_RESULT_*`` error codes.
- A ``from_result_t`` conversion is provided.
- No safe public function returns a raw ``opensomeip_result_t``.

REQ_RUST_004 — No Unsafe in Public API Surface
================================================

**ID:** REQ_RUST_004

**Description:**
The public API of the ``opensomeip`` crate SHALL NOT expose any
``unsafe`` functions or require callers to use ``unsafe`` blocks.

**Rationale:**
Encapsulating unsafety within the crate boundary allows callers to
rely on Rust's safety guarantees.

**Acceptance Criteria:**

- ``unsafe`` is confined to ``opensomeip-sys`` (FFI declarations) and
  private modules in ``opensomeip``.
- Every ``unsafe`` block has a ``// SAFETY:`` comment.
- ``#![warn(unsafe_op_in_unsafe_fn)]`` is set on all crate roots.

REQ_RUST_005 — Ferrocene Subset Compatibility
===============================================

**ID:** REQ_RUST_005

**Description:**
The crate SHALL be compatible with the Ferrocene qualified toolchain
constraints documented in ``docs/FERROCENE_SUBSET.md``. This includes:

- ``#![no_std]`` support behind a feature flag
- ``panic = "abort"`` in release profile
- No proc macros in the safety-relevant dependency path
- No build-time code generation (no bindgen)
- Rust 2021 edition, no unstable features

**Rationale:**
Ferrocene compatibility is required for safety-critical automotive
deployments under ISO 26262.

**Acceptance Criteria:**

- ``opensomeip-sys`` is ``#![no_std]`` unconditionally.
- ``opensomeip`` builds with ``--no-default-features`` (no_std mode).
- ``cargo build`` succeeds with ``RUSTFLAGS="-C panic=abort"``.
- No ``#![feature(...)]`` attributes.

REQ_RUST_006 — Thread Safety
==============================

**ID:** REQ_RUST_006

**Description:**
Wrapper types SHALL implement ``Send`` and/or ``Sync`` where the
underlying C API documents thread safety. Types that are NOT thread-safe
SHALL NOT implement these traits.

**Rationale:**
Rust's ``Send``/``Sync`` system prevents data races at compile time.
Accurate trait implementations preserve this guarantee across the FFI
boundary.

**Acceptance Criteria:**

- Transport types (UDP, TCP) implement ``Send + Sync`` (the C API
  documents internal locking).
- ``SomeIpMessage`` implements ``Send`` (can be moved across threads)
  but documents any Sync limitations.
- Compile-time assertions verify trait implementations.

REQ_RUST_007 — Traceability Annotations
=========================================

**ID:** REQ_RUST_007

**Description:**
Implementation code SHALL contain ``@implements REQ_RUST_*`` annotations
and test code SHALL contain ``@tests REQ_RUST_*`` annotations that trace
back to this requirements document.

**Rationale:**
Traceability supports safety assessment and change-impact analysis.

**Acceptance Criteria:**

- Every public type and function in ``opensomeip`` has at least one
  ``@implements`` annotation.
- Every test function has a ``@tests`` annotation.
- A CI script can extract and report the traceability matrix.
