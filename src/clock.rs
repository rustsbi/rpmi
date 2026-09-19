//! RPMI 1.0 CLOCK service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-clock.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x0008;

/// `CLK_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `CLK_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `CLK_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `CLK_GET_NUM_CLOCKS`.
pub const GET_NUM_CLOCKS: u8 = 0x02;

/// Payload for `CLK_GET_NUM_CLOCKS` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetNumClocksResponse {
    pub status: i32,
    pub num_clocks: u32,
}

/// `CLK_GET_ATTRIBUTES`.
pub const GET_ATTRIBUTES: u8 = 0x03;

/// Payload for `CLK_GET_ATTRIBUTES` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesRequest {
    pub clock_id: u32,
}

/// Payload for `CLK_GET_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesResponse {
    pub status: i32,
    pub flags: u32,
    pub num_rates: u32,
    pub transition_latency: u32,
    pub clock_name: [u8; 16],
}

/// `CLK_GET_SUPPORTED_RATES`.
pub const GET_SUPPORTED_RATES: u8 = 0x04;

/// Payload for `CLK_GET_SUPPORTED_RATES` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSupportedRatesRequest {
    pub clock_id: u32,
    pub clock_rate_index: u32,
}

/// Payload for `CLK_GET_SUPPORTED_RATES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSupportedRatesResponse {
    pub status: i32,
    pub flags: u32,
    pub remaining: u32,
    pub returned: u32,
    pub clock_rate: [[u32; 2]; 0],
}

/// `CLK_SET_CONFIG`.
pub const SET_CONFIG: u8 = 0x05;

/// Payload for `CLK_SET_CONFIG` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SetConfigRequest {
    pub clock_id: u32,
    pub config: u32,
}

/// Payload for `CLK_SET_CONFIG` (response).
pub type SetConfigResponse = crate::message::StatusResponse;

/// `CLK_GET_CONFIG`.
pub const GET_CONFIG: u8 = 0x06;

/// Payload for `CLK_GET_CONFIG` (request).
pub type GetConfigRequest = GetAttributesRequest;

/// Payload for `CLK_GET_CONFIG` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetConfigResponse {
    pub status: i32,
    pub config: u32,
}

/// `CLK_SET_RATE`.
pub const SET_RATE: u8 = 0x07;

/// Payload for `CLK_SET_RATE` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SetRateRequest {
    pub clock_id: u32,
    pub flags: u32,
    pub clock_rate_low: u32,
    pub clock_rate_high: u32,
}

/// Payload for `CLK_SET_RATE` (response).
pub type SetRateResponse = crate::message::StatusResponse;

/// `CLK_GET_RATE`.
pub const GET_RATE: u8 = 0x08;

/// Payload for `CLK_GET_RATE` (request).
pub type GetRateRequest = GetAttributesRequest;

/// Payload for `CLK_GET_RATE` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetRateResponse {
    pub status: i32,
    pub clock_rate_low: u32,
    pub clock_rate_high: u32,
}

/// `CLK_SET_RATE` rounding mode in `flags[1:0]`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum RoundingMode {
    /// Round down.
    Down = 0,
    /// Round up.
    Up = 1,
    /// Let the platform choose the closest supported rate.
    Auto = 2,
}
