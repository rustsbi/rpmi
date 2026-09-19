//! RPMI 1.0 VOLTAGE service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-voltage.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x0007;

/// `VOLT_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `VOLT_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `VOLT_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `VOLT_GET_NUM_DOMAINS`.
pub const GET_NUM_DOMAINS: u8 = 0x02;

/// Payload for `VOLT_GET_NUM_DOMAINS` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetNumDomainsResponse {
    pub status: i32,
    pub num_domains: u32,
}

/// `VOLT_GET_ATTRIBUTES`.
pub const GET_ATTRIBUTES: u8 = 0x03;

/// Payload for `VOLT_GET_ATTRIBUTES` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesRequest {
    pub domain_id: u32,
}

/// Payload for `VOLT_GET_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesResponse {
    pub status: i32,
    pub flags: u32,
    pub num_levels: u32,
    pub trans_latency: u32,
    pub domain_name: [u8; 16],
}

/// `VOLT_GET_SUPPORTED_LEVELS`.
pub const GET_SUPPORTED_LEVELS: u8 = 0x04;

/// Payload for `VOLT_GET_SUPPORTED_LEVELS` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSupportedLevelsRequest {
    pub domain_id: u32,
    pub voltage_level_index: u32,
}

/// Payload for `VOLT_GET_SUPPORTED_LEVELS` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSupportedLevelsResponse {
    pub status: i32,
    pub flags: u32,
    pub remaining: u32,
    pub returned: u32,
    pub voltage_level: [u32; 0],
}

/// `VOLT_SET_CONFIG`.
pub const SET_CONFIG: u8 = 0x05;

/// Payload for `VOLT_SET_CONFIG` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SetConfigRequest {
    pub domain_id: u32,
    pub config: u32,
}

/// Payload for `VOLT_SET_CONFIG` (response).
pub type SetConfigResponse = crate::message::StatusResponse;

/// `VOLT_GET_CONFIG`.
pub const GET_CONFIG: u8 = 0x06;

/// Payload for `VOLT_GET_CONFIG` (request).
pub type GetConfigRequest = GetAttributesRequest;

/// Payload for `VOLT_GET_CONFIG` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetConfigResponse {
    pub status: i32,
    pub config: u32,
}

/// `VOLT_SET_LEVEL`.
pub const SET_LEVEL: u8 = 0x07;

/// Payload for `VOLT_SET_LEVEL` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SetLevelRequest {
    pub domain_id: u32,
    pub voltage_level: i32,
}

/// Payload for `VOLT_SET_LEVEL` (response).
pub type SetLevelResponse = crate::message::StatusResponse;

/// `VOLT_GET_LEVEL`.
pub const GET_LEVEL: u8 = 0x08;

/// Payload for `VOLT_GET_LEVEL` (request).
pub type GetLevelRequest = GetAttributesRequest;

/// Payload for `VOLT_GET_LEVEL` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetLevelResponse {
    pub status: i32,
    pub voltage_level: i32,
}
