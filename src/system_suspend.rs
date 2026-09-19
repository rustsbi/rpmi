//! RPMI 1.0 SYSTEM_SUSPEND service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-system-suspend.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x0004;

/// `SYSSUSP_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `SYSSUSP_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `SYSSUSP_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `SYSSUSP_GET_ATTRIBUTES`.
pub const GET_ATTRIBUTES: u8 = 0x02;

/// Payload for `SYSSUSP_GET_ATTRIBUTES` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesRequest {
    pub suspend_type: u32,
}

/// Payload for `SYSSUSP_GET_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesResponse {
    pub status: i32,
    pub flags: u32,
}

/// `SYSSUSP_SUSPEND`.
pub const SUSPEND: u8 = 0x03;

/// Payload for `SYSSUSP_SUSPEND` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SuspendRequest {
    pub hart_id: u32,
    pub suspend_type: u32,
    pub resume_addr_low: u32,
    pub resume_addr_high: u32,
}

/// Payload for `SYSSUSP_SUSPEND` (response).
pub type SuspendResponse = crate::message::StatusResponse;
