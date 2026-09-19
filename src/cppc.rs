//! RPMI 1.0 CPPC service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-cppc.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x0006;

/// `CPPC_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `CPPC_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `CPPC_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `CPPC_PROBE_REG`.
pub const PROBE_REG: u8 = 0x02;

/// Payload for `CPPC_PROBE_REG` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ProbeRegRequest {
    pub reg_id: u32,
    pub hart_id: u32,
}

/// Payload for `CPPC_PROBE_REG` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ProbeRegResponse {
    pub status: i32,
    pub reg_length: u32,
}

/// `CPPC_READ_REG`.
pub const READ_REG: u8 = 0x03;

/// Payload for `CPPC_READ_REG` (request).
pub type ReadRegRequest = ProbeRegRequest;

/// Payload for `CPPC_READ_REG` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ReadRegResponse {
    pub status: i32,
    pub data_low: u32,
    pub data_high: u32,
}

/// `CPPC_WRITE_REG`.
pub const WRITE_REG: u8 = 0x04;

/// Payload for `CPPC_WRITE_REG` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct WriteRegRequest {
    pub reg_id: u32,
    pub hart_id: u32,
    pub data_low: u32,
    pub data_high: u32,
}

/// Payload for `CPPC_WRITE_REG` (response).
pub type WriteRegResponse = crate::message::StatusResponse;

/// `CPPC_GET_FAST_CHANNEL_REGION`.
pub const GET_FAST_CHANNEL_REGION: u8 = 0x05;

/// Payload for `CPPC_GET_FAST_CHANNEL_REGION` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetFastChannelRegionResponse {
    pub status: i32,
    pub flags: u32,
    pub region_addr_low: u32,
    pub region_addr_high: u32,
    pub region_size_low: u32,
    pub region_size_high: u32,
    pub db_addr_low: u32,
    pub db_addr_high: u32,
    pub db_write_value: u32,
}

/// `CPPC_GET_FAST_CHANNEL_OFFSET`.
pub const GET_FAST_CHANNEL_OFFSET: u8 = 0x06;

/// Payload for `CPPC_GET_FAST_CHANNEL_OFFSET` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetFastChannelOffsetRequest {
    pub hart_id: u32,
}

/// Payload for `CPPC_GET_FAST_CHANNEL_OFFSET` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetFastChannelOffsetResponse {
    pub status: i32,
    pub perf_request_offset_low: u32,
    pub perf_request_offset_high: u32,
    pub perf_feedback_offset_low: u32,
    pub perf_feedback_offset_high: u32,
}

/// `CPPC_GET_HART_LIST`.
pub const GET_HART_LIST: u8 = 0x07;

/// Payload for `CPPC_GET_HART_LIST` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetHartListRequest {
    pub start_index: u32,
}

/// Payload for `CPPC_GET_HART_LIST` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetHartListResponse {
    pub status: i32,
    pub remaining: u32,
    pub returned: u32,
    pub hart_id: [u32; 0],
}

/// Normal-mode fast-channel; address must be 8-byte aligned, reserved must be zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PerformanceRequest {
    pub desired_performance: u32,
    pub reserved: u32,
}

/// Autonomous-mode fast-channel; address must be 8-byte aligned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct AutonomousPerformanceRequest {
    pub minimum_performance: u32,
    pub maximum_performance: u32,
}

/// Feedback fast-channel in Hz; address must be 8-byte aligned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PerformanceFeedback {
    pub frequency_low: u32,
    pub frequency_high: u32,
}
