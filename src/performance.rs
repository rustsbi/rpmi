//! RPMI 1.0 PERFORMANCE service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-performance.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x000a;

/// `PERF_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `PERF_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `PERF_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `PERF_GET_NUM_DOMAINS`.
pub const GET_NUM_DOMAINS: u8 = 0x02;

/// Payload for `PERF_GET_NUM_DOMAINS` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetNumDomainsResponse {
    pub status: i32,
    pub num_domains: u32,
}

/// `PERF_GET_ATTRIBUTES`.
pub const GET_ATTRIBUTES: u8 = 0x03;

/// Payload for `PERF_GET_ATTRIBUTES` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesRequest {
    pub domain_id: u32,
}

/// Payload for `PERF_GET_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesResponse {
    pub status: i32,
    pub flags: u32,
    pub num_levels: u32,
    pub transition_latency: u32,
    pub domain_name: [u8; 16],
}

/// `PERF_GET_SUPPORTED_LEVELS`.
pub const GET_SUPPORTED_LEVELS: u8 = 0x04;

/// Payload for `PERF_GET_SUPPORTED_LEVELS` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSupportedLevelsRequest {
    pub domain_id: u32,
    pub perf_level_index: u32,
}

/// Payload for `PERF_GET_SUPPORTED_LEVELS` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSupportedLevelsResponse {
    pub status: i32,
    pub flags: u32,
    pub remaining: u32,
    pub returned: u32,
    pub level: [[u32; 4]; 0],
}

/// `PERF_GET_LEVEL`.
pub const GET_LEVEL: u8 = 0x05;

/// Payload for `PERF_GET_LEVEL` (request).
pub type GetLevelRequest = GetAttributesRequest;

/// Payload for `PERF_GET_LEVEL` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetLevelResponse {
    pub status: i32,
    pub level: u32,
}

/// `PERF_SET_LEVEL`.
pub const SET_LEVEL: u8 = 0x06;

/// Payload for `PERF_SET_LEVEL` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SetLevelRequest {
    pub domain_id: u32,
    pub level: u32,
}

/// Payload for `PERF_SET_LEVEL` (response).
pub type SetLevelResponse = crate::message::StatusResponse;

/// `PERF_GET_LIMIT`.
pub const GET_LIMIT: u8 = 0x07;

/// Payload for `PERF_GET_LIMIT` (request).
pub type GetLimitRequest = GetAttributesRequest;

/// Payload for `PERF_GET_LIMIT` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetLimitResponse {
    pub status: i32,
    pub max_perf_level: u32,
    pub min_perf_level: u32,
}

/// `PERF_SET_LIMIT`.
pub const SET_LIMIT: u8 = 0x08;

/// Payload for `PERF_SET_LIMIT` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SetLimitRequest {
    pub domain_id: u32,
    pub max_perf_level: u32,
    pub min_perf_level: u32,
}

/// Payload for `PERF_SET_LIMIT` (response).
pub type SetLimitResponse = crate::message::StatusResponse;

/// `PERF_GET_FAST_CHANNEL_REGION`.
pub const GET_FAST_CHANNEL_REGION: u8 = 0x09;

/// Payload for `PERF_GET_FAST_CHANNEL_REGION` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetFastChannelRegionResponse {
    pub status: i32,
    pub region_phys_addr_low: u32,
    pub region_phys_addr_high: u32,
    pub region_size_low: u32,
    pub region_size_high: u32,
}

/// `PERF_GET_FAST_CHANNEL_ATTRIBUTES`.
pub const GET_FAST_CHANNEL_ATTRIBUTES: u8 = 0x0a;

/// Payload for `PERF_GET_FAST_CHANNEL_ATTRIBUTES` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetFastChannelAttributesRequest {
    pub domain_id: u32,
    pub service_id: u32,
}

/// Payload for `PERF_GET_FAST_CHANNEL_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetFastChannelAttributesResponse {
    pub status: i32,
    pub flags: u32,
    pub fastchan_offset_low: u32,
    pub fastchan_offset_high: u32,
    pub fastchan_size: u32,
    pub db_addr_low: u32,
    pub db_addr_high: u32,
    pub db_write_value: u32,
}

/// PERF_POWER_CHANGE event data; power is in microwatts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PowerChangeEvent {
    pub domain_id: u32,
    pub power: u32,
}

/// Performance level entry; frequency in kHz, power in microwatts, latency in microseconds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct LevelAttribute {
    pub index: u32,
    pub clock_freq: u32,
    pub power_cost: u32,
    pub transition_latency: u32,
}

/// PERF_GET_LEVEL / PERF_SET_LEVEL fast-channel payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct FastLevel {
    pub level: u32,
}

/// PERF_GET_LIMIT / PERF_SET_LIMIT fast-channel payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct FastLimit {
    pub max_perf_level: u32,
    pub min_perf_level: u32,
}

/// PERFORMANCE notification event IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum EventId {
    PowerChange = 1,
    LimitChange = 2,
    LevelChange = 3,
}

/// PERF_LIMIT_CHANGE event data.
pub type LimitChangeEvent = SetLimitRequest;

/// PERF_LEVEL_CHANGE event data.
pub type LevelChangeEvent = SetLevelRequest;
