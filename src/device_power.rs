//! RPMI 1.0 DEVICE_POWER service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-device-power.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x0009;

/// `DPWR_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `DPWR_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `DPWR_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `DPWR_GET_NUM_DOMAINS`.
pub const GET_NUM_DOMAINS: u8 = 0x02;

/// Payload for `DPWR_GET_NUM_DOMAINS` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetNumDomainsResponse {
    pub status: i32,
    pub num_domains: u32,
}

/// `DPWR_GET_ATTRIBUTES`.
pub const GET_ATTRIBUTES: u8 = 0x03;

/// Payload for `DPWR_GET_ATTRIBUTES` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesRequest {
    pub domain_id: u32,
}

/// Payload for `DPWR_GET_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesResponse {
    pub status: i32,
    pub flags: u32,
    pub transition_latency: u32,
    pub domain_name: [u8; 16],
}

/// `DPWR_SET_STATE`.
pub const SET_STATE: u8 = 0x04;

/// Payload for `DPWR_SET_STATE` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SetStateRequest {
    pub domain_id: u32,
    pub power_state: u32,
}

/// Payload for `DPWR_SET_STATE` (response).
pub type SetStateResponse = crate::message::StatusResponse;

/// `DPWR_GET_STATE`.
pub const GET_STATE: u8 = 0x05;

/// Payload for `DPWR_GET_STATE` (request).
pub type GetStateRequest = GetAttributesRequest;

/// Payload for `DPWR_GET_STATE` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetStateResponse {
    pub status: i32,
    pub power_state: u32,
}

/// Standard power-state value in `power_state[15:0]`; context is encoded separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PowerState {
    On = 0,
    Off = 3,
}

/// Power-state bit 16: context is lost.
pub const STATE_CONTEXT_LOST: u32 = 65536;
