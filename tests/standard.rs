// RPMI v1.0: https://github.com/riscv-non-isa/riscv-rpmi/tree/v1.0/src
// Golden sizes and byte offsets from the specification, including variable tails.
use core::mem::{offset_of, size_of};
use rpmi::*;

#[test]
fn signed_fields_and_message_encodings() {
    assert_eq!(size_of::<message::MessageType>(), 1);
    assert_eq!(size_of::<message::NotificationRequestState>(), 4);
    assert_eq!(size_of::<status::Status>(), 4);
    assert_eq!(size_of::<device_power::PowerState>(), 4);
    assert_eq!(size_of::<base::EventId>(), 1);
    assert_eq!(size_of::<performance::EventId>(), 1);
    assert_eq!(size_of::<request_forward::EventId>(), 1);
    // These type checks distinguish signed fields even when sizes match.
    let _: fn(&message::StatusResponse) -> &i32 = |p| &p.status;
    let _: fn(&voltage::SetLevelRequest) -> &i32 = |p| &p.voltage_level;
    let _: fn(&voltage::GetLevelResponse) -> &i32 = |p| &p.voltage_level;
    let _: fn(&voltage::GetSupportedLevelsResponse) -> &[u32; 0] = |p| &p.voltage_level;
    let _: fn(&clock::GetSupportedRatesResponse) -> &[[u32; 2]; 0] = |p| &p.clock_rate;
    let _: fn(&performance::GetSupportedLevelsResponse) -> &[[u32; 4]; 0] = |p| &p.level;
    assert_eq!(
        [
            message::MessageType::NormalRequest as u8,
            message::MessageType::PostedRequest as u8,
            message::MessageType::Acknowledgement as u8,
            message::MessageType::Notification as u8,
            message::TYPE_MASK,
            message::DOORBELL,
            message::NOTIFICATION_SERVICE_ID,
        ],
        [0, 1, 2, 3, 7, 8, 0]
    );
    assert_eq!(
        [
            message::NotificationRequestState::Disable as u32,
            message::NotificationRequestState::Enable as u32,
            message::NotificationRequestState::Query as u32,
        ],
        [0, 1, 2]
    );
    assert_eq!(
        [
            status::Status::Success as i32,
            status::Status::Failed as i32,
            status::Status::NotSupported as i32,
            status::Status::InvalidParam as i32,
            status::Status::Denied as i32,
            status::Status::InvalidAddr as i32,
            status::Status::Already as i32,
            status::Status::Extension as i32,
            status::Status::HwFault as i32,
            status::Status::Busy as i32,
            status::Status::InvalidState as i32,
            status::Status::BadRange as i32,
            status::Status::Timeout as i32,
            status::Status::Io as i32,
            status::Status::NoData as i32,
        ],
        [0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14]
    );
}

macro_rules! layout {
    ($ty:ty, $size:expr; $($field:ident: $offset:expr),* $(,)?) => {
        assert_eq!(size_of::<$ty>(), $size, stringify!($ty));
        $(assert_eq!(offset_of!($ty, $field), $offset,
            concat!(stringify!($ty), "::", stringify!($field)));)*
    };
}

#[test]
fn standard_layouts() {
    layout!(base::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(base::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(base::GetImplementationVersionResponse, 8; status: 0, impl_version: 4);
    layout!(base::GetImplementationIdResponse, 8; status: 0, impl_id: 4);
    layout!(base::GetSpecVersionResponse, 8; status: 0, spec_version: 4);
    layout!(base::GetPlatformInfoResponse, 8; status: 0, platform_id_len: 4, platform_id: 8);
    layout!(base::ProbeServiceGroupRequest, 4; servicegroup_id: 0);
    layout!(base::ProbeServiceGroupResponse, 8; status: 0, service_group_version: 4);
    layout!(base::GetAttributesResponse, 20; status: 0, flags0: 4, flags1: 8, flags2: 12, flags3: 16);
    layout!(system_msi::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(system_msi::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(system_msi::GetAttributesResponse, 16; status: 0, sys_num_msi: 4, flags0: 8, flags1: 12);
    layout!(system_msi::GetMsiAttributesRequest, 4; sys_msi_index: 0);
    layout!(system_msi::GetMsiAttributesResponse, 28; status: 0, flags0: 4, flags1: 8, sys_msi_name: 12);
    layout!(system_msi::SetMsiStateRequest, 8; sys_msi_index: 0, sys_msi_state: 4);
    layout!(system_msi::SetMsiStateResponse, 4; status: 0);
    layout!(system_msi::GetMsiStateRequest, 4; sys_msi_index: 0);
    layout!(system_msi::GetMsiStateResponse, 8; status: 0, sys_msi_state: 4);
    layout!(system_msi::SetMsiTargetRequest, 16; sys_msi_index: 0, sys_msi_address_low: 4, sys_msi_address_high: 8, sys_msi_data: 12);
    layout!(system_msi::SetMsiTargetResponse, 4; status: 0);
    layout!(system_msi::GetMsiTargetRequest, 4; sys_msi_index: 0);
    layout!(system_msi::GetMsiTargetResponse, 16; status: 0, sys_msi_address_low: 4, sys_msi_address_high: 8, sys_msi_data: 12);
    layout!(system_reset::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(system_reset::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(system_reset::GetAttributesRequest, 4; reset_type: 0);
    layout!(system_reset::GetAttributesResponse, 8; status: 0, flags: 4);
    layout!(system_reset::ResetRequest, 4; reset_type: 0);
    layout!(system_suspend::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(system_suspend::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(system_suspend::GetAttributesRequest, 4; suspend_type: 0);
    layout!(system_suspend::GetAttributesResponse, 8; status: 0, flags: 4);
    layout!(system_suspend::SuspendRequest, 16; hart_id: 0, suspend_type: 4, resume_addr_low: 8, resume_addr_high: 12);
    layout!(system_suspend::SuspendResponse, 4; status: 0);
    layout!(hsm::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(hsm::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(hsm::GetHartStatusRequest, 4; hart_id: 0);
    layout!(hsm::GetHartStatusResponse, 8; status: 0, hart_state: 4);
    layout!(hsm::GetHartListRequest, 4; start_index: 0);
    layout!(hsm::GetHartListResponse, 12; status: 0, remaining: 4, returned: 8, hart_id: 12);
    layout!(hsm::GetSuspendTypesRequest, 4; start_index: 0);
    layout!(hsm::GetSuspendTypesResponse, 12; status: 0, remaining: 4, returned: 8, suspend_type: 12);
    layout!(hsm::GetSuspendInfoRequest, 4; suspend_type: 0);
    layout!(hsm::GetSuspendInfoResponse, 24; status: 0, flags: 4, entry_latency: 8, exit_latency: 12, wakeup_latency: 16, min_residency: 20);
    layout!(hsm::HartStartRequest, 12; hart_id: 0, start_addr_low: 4, start_addr_high: 8);
    layout!(hsm::HartStartResponse, 4; status: 0);
    layout!(hsm::HartStopRequest, 4; hart_id: 0);
    layout!(hsm::HartStopResponse, 4; status: 0);
    layout!(hsm::HartSuspendRequest, 16; hart_id: 0, suspend_type: 4, resume_addr_low: 8, resume_addr_high: 12);
    layout!(hsm::HartSuspendResponse, 4; status: 0);
    layout!(cppc::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(cppc::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(cppc::ProbeRegRequest, 8; reg_id: 0, hart_id: 4);
    layout!(cppc::ProbeRegResponse, 8; status: 0, reg_length: 4);
    layout!(cppc::ReadRegRequest, 8; reg_id: 0, hart_id: 4);
    layout!(cppc::ReadRegResponse, 12; status: 0, data_low: 4, data_high: 8);
    layout!(cppc::WriteRegRequest, 16; reg_id: 0, hart_id: 4, data_low: 8, data_high: 12);
    layout!(cppc::WriteRegResponse, 4; status: 0);
    layout!(cppc::GetFastChannelRegionResponse, 36; status: 0, flags: 4, region_addr_low: 8, region_addr_high: 12, region_size_low: 16, region_size_high: 20, db_addr_low: 24, db_addr_high: 28, db_write_value: 32);
    layout!(cppc::GetFastChannelOffsetRequest, 4; hart_id: 0);
    layout!(cppc::GetFastChannelOffsetResponse, 20; status: 0, perf_request_offset_low: 4, perf_request_offset_high: 8, perf_feedback_offset_low: 12, perf_feedback_offset_high: 16);
    layout!(cppc::GetHartListRequest, 4; start_index: 0);
    layout!(cppc::GetHartListResponse, 12; status: 0, remaining: 4, returned: 8, hart_id: 12);
    layout!(voltage::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(voltage::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(voltage::GetNumDomainsResponse, 8; status: 0, num_domains: 4);
    layout!(voltage::GetAttributesRequest, 4; domain_id: 0);
    layout!(voltage::GetAttributesResponse, 32; status: 0, flags: 4, num_levels: 8, trans_latency: 12, domain_name: 16);
    layout!(voltage::GetSupportedLevelsRequest, 8; domain_id: 0, voltage_level_index: 4);
    layout!(voltage::GetSupportedLevelsResponse, 16; status: 0, flags: 4, remaining: 8, returned: 12, voltage_level: 16);
    layout!(voltage::SetConfigRequest, 8; domain_id: 0, config: 4);
    layout!(voltage::SetConfigResponse, 4; status: 0);
    layout!(voltage::GetConfigRequest, 4; domain_id: 0);
    layout!(voltage::GetConfigResponse, 8; status: 0, config: 4);
    layout!(voltage::SetLevelRequest, 8; domain_id: 0, voltage_level: 4);
    layout!(voltage::SetLevelResponse, 4; status: 0);
    layout!(voltage::GetLevelRequest, 4; domain_id: 0);
    layout!(voltage::GetLevelResponse, 8; status: 0, voltage_level: 4);
    layout!(clock::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(clock::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(clock::GetNumClocksResponse, 8; status: 0, num_clocks: 4);
    layout!(clock::GetAttributesRequest, 4; clock_id: 0);
    layout!(clock::GetAttributesResponse, 32; status: 0, flags: 4, num_rates: 8, transition_latency: 12, clock_name: 16);
    layout!(clock::GetSupportedRatesRequest, 8; clock_id: 0, clock_rate_index: 4);
    layout!(clock::GetSupportedRatesResponse, 16; status: 0, flags: 4, remaining: 8, returned: 12, clock_rate: 16);
    layout!(clock::SetConfigRequest, 8; clock_id: 0, config: 4);
    layout!(clock::SetConfigResponse, 4; status: 0);
    layout!(clock::GetConfigRequest, 4; clock_id: 0);
    layout!(clock::GetConfigResponse, 8; status: 0, config: 4);
    layout!(clock::SetRateRequest, 16; clock_id: 0, flags: 4, clock_rate_low: 8, clock_rate_high: 12);
    layout!(clock::SetRateResponse, 4; status: 0);
    layout!(clock::GetRateRequest, 4; clock_id: 0);
    layout!(clock::GetRateResponse, 12; status: 0, clock_rate_low: 4, clock_rate_high: 8);
    layout!(device_power::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(device_power::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(device_power::GetNumDomainsResponse, 8; status: 0, num_domains: 4);
    layout!(device_power::GetAttributesRequest, 4; domain_id: 0);
    layout!(device_power::GetAttributesResponse, 28; status: 0, flags: 4, transition_latency: 8, domain_name: 12);
    layout!(device_power::SetStateRequest, 8; domain_id: 0, power_state: 4);
    layout!(device_power::SetStateResponse, 4; status: 0);
    layout!(device_power::GetStateRequest, 4; domain_id: 0);
    layout!(device_power::GetStateResponse, 8; status: 0, power_state: 4);
    layout!(performance::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(performance::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(performance::GetNumDomainsResponse, 8; status: 0, num_domains: 4);
    layout!(performance::GetAttributesRequest, 4; domain_id: 0);
    layout!(performance::GetAttributesResponse, 32; status: 0, flags: 4, num_levels: 8, transition_latency: 12, domain_name: 16);
    layout!(performance::GetSupportedLevelsRequest, 8; domain_id: 0, perf_level_index: 4);
    layout!(performance::GetSupportedLevelsResponse, 16; status: 0, flags: 4, remaining: 8, returned: 12, level: 16);
    layout!(performance::GetLevelRequest, 4; domain_id: 0);
    layout!(performance::GetLevelResponse, 8; status: 0, level: 4);
    layout!(performance::SetLevelRequest, 8; domain_id: 0, level: 4);
    layout!(performance::SetLevelResponse, 4; status: 0);
    layout!(performance::GetLimitRequest, 4; domain_id: 0);
    layout!(performance::GetLimitResponse, 12; status: 0, max_perf_level: 4, min_perf_level: 8);
    layout!(performance::SetLimitRequest, 12; domain_id: 0, max_perf_level: 4, min_perf_level: 8);
    layout!(performance::SetLimitResponse, 4; status: 0);
    layout!(performance::GetFastChannelRegionResponse, 20; status: 0, region_phys_addr_low: 4, region_phys_addr_high: 8, region_size_low: 12, region_size_high: 16);
    layout!(performance::GetFastChannelAttributesRequest, 8; domain_id: 0, service_id: 4);
    layout!(performance::GetFastChannelAttributesResponse, 32; status: 0, flags: 4, fastchan_offset_low: 8, fastchan_offset_high: 12, fastchan_size: 16, db_addr_low: 20, db_addr_high: 24, db_write_value: 28);
    layout!(management_mode::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(management_mode::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(management_mode::GetAttributesResponse, 20; status: 0, mm_version: 4, mm_shmem_addr_low: 8, mm_shmem_addr_high: 12, mm_shmem_size: 16);
    layout!(management_mode::CommunicateRequest, 16; mm_comm_input_data_offset: 0, mm_comm_input_data_size: 4, mm_comm_output_data_offset: 8, mm_comm_output_data_size: 12);
    layout!(management_mode::CommunicateResponse, 8; status: 0, mm_comm_return_data_size: 4);
    layout!(ras_agent::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(ras_agent::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(ras_agent::GetNumErrSrcsResponse, 8; status: 0, num_err_srcs: 4);
    layout!(ras_agent::GetErrSrcsIdListRequest, 4; start_index: 0);
    layout!(ras_agent::GetErrSrcsIdListResponse, 16; status: 0, flags: 4, remaining: 8, returned: 12, ras_err_src_id: 16);
    layout!(ras_agent::GetErrSrcDescRequest, 8; ras_err_src_id: 0, byte_offset: 4);
    layout!(ras_agent::GetErrSrcDescResponse, 16; status: 0, flags: 4, remaining: 8, returned: 12, err_src_desc: 16);
    layout!(request_forward::EnableNotificationRequest, 8; event_id: 0, req_state: 4);
    layout!(request_forward::EnableNotificationResponse, 8; status: 0, current_state: 4);
    layout!(request_forward::RetrieveCurrentMessageRequest, 4; start_index: 0);
    layout!(request_forward::RetrieveCurrentMessageResponse, 12; status: 0, remaining: 4, returned: 8, request_message: 12);
    layout!(request_forward::CompleteCurrentMessageRequest, 0; response_data: 0);
    layout!(request_forward::CompleteCurrentMessageResponse, 8; status: 0, num_messages: 4);
    layout!(cppc::PerformanceRequest, 8; desired_performance: 0, reserved: 4);
    layout!(cppc::AutonomousPerformanceRequest, 8; minimum_performance: 0, maximum_performance: 4);
    layout!(cppc::PerformanceFeedback, 8; frequency_low: 0, frequency_high: 4);
    layout!(performance::PowerChangeEvent, 8; domain_id: 0, power: 4);
    layout!(performance::LevelAttribute, 16; index: 0, clock_freq: 4, power_cost: 8, transition_latency: 12);
    layout!(performance::FastLevel, 4; level: 0);
    layout!(performance::FastLimit, 8; max_perf_level: 0, min_perf_level: 4);
    layout!(performance::LimitChangeEvent, 12; domain_id: 0, max_perf_level: 4, min_perf_level: 8);
    layout!(performance::LevelChangeEvent, 8; domain_id: 0, level: 4);
    layout!(message::Header, 8; service_group_id: 0, service_id: 2, flags: 3, data_len: 4, token: 6);
    layout!(message::EventHeader, 4; data_len: 0, event_id: 2, reserved: 3);
    layout!(message::Message<14>, 64; header: 0, data: 8);
    layout!(message::Event<3>, 16; header: 0, data: 4);
}

#[test]
fn standard_ids() {
    assert_eq!(base::SERVICE_GROUP_ID, 1, "base::SERVICE_GROUP_ID");
    assert_eq!(base::ENABLE_NOTIFICATION, 1, "base::ENABLE_NOTIFICATION");
    assert_eq!(
        base::GET_IMPLEMENTATION_VERSION,
        2,
        "base::GET_IMPLEMENTATION_VERSION"
    );
    assert_eq!(
        base::GET_IMPLEMENTATION_ID,
        3,
        "base::GET_IMPLEMENTATION_ID"
    );
    assert_eq!(base::GET_SPEC_VERSION, 4, "base::GET_SPEC_VERSION");
    assert_eq!(base::GET_PLATFORM_INFO, 5, "base::GET_PLATFORM_INFO");
    assert_eq!(base::PROBE_SERVICE_GROUP, 6, "base::PROBE_SERVICE_GROUP");
    assert_eq!(base::GET_ATTRIBUTES, 7, "base::GET_ATTRIBUTES");
    assert_eq!(
        system_msi::SERVICE_GROUP_ID,
        2,
        "system_msi::SERVICE_GROUP_ID"
    );
    assert_eq!(
        system_msi::ENABLE_NOTIFICATION,
        1,
        "system_msi::ENABLE_NOTIFICATION"
    );
    assert_eq!(system_msi::GET_ATTRIBUTES, 2, "system_msi::GET_ATTRIBUTES");
    assert_eq!(
        system_msi::GET_MSI_ATTRIBUTES,
        3,
        "system_msi::GET_MSI_ATTRIBUTES"
    );
    assert_eq!(system_msi::SET_MSI_STATE, 4, "system_msi::SET_MSI_STATE");
    assert_eq!(system_msi::GET_MSI_STATE, 5, "system_msi::GET_MSI_STATE");
    assert_eq!(system_msi::SET_MSI_TARGET, 6, "system_msi::SET_MSI_TARGET");
    assert_eq!(system_msi::GET_MSI_TARGET, 7, "system_msi::GET_MSI_TARGET");
    assert_eq!(
        system_reset::SERVICE_GROUP_ID,
        3,
        "system_reset::SERVICE_GROUP_ID"
    );
    assert_eq!(
        system_reset::ENABLE_NOTIFICATION,
        1,
        "system_reset::ENABLE_NOTIFICATION"
    );
    assert_eq!(
        system_reset::GET_ATTRIBUTES,
        2,
        "system_reset::GET_ATTRIBUTES"
    );
    assert_eq!(system_reset::RESET, 3, "system_reset::RESET");
    assert_eq!(
        system_suspend::SERVICE_GROUP_ID,
        4,
        "system_suspend::SERVICE_GROUP_ID"
    );
    assert_eq!(
        system_suspend::ENABLE_NOTIFICATION,
        1,
        "system_suspend::ENABLE_NOTIFICATION"
    );
    assert_eq!(
        system_suspend::GET_ATTRIBUTES,
        2,
        "system_suspend::GET_ATTRIBUTES"
    );
    assert_eq!(system_suspend::SUSPEND, 3, "system_suspend::SUSPEND");
    assert_eq!(hsm::SERVICE_GROUP_ID, 5, "hsm::SERVICE_GROUP_ID");
    assert_eq!(hsm::ENABLE_NOTIFICATION, 1, "hsm::ENABLE_NOTIFICATION");
    assert_eq!(hsm::GET_HART_STATUS, 2, "hsm::GET_HART_STATUS");
    assert_eq!(hsm::GET_HART_LIST, 3, "hsm::GET_HART_LIST");
    assert_eq!(hsm::GET_SUSPEND_TYPES, 4, "hsm::GET_SUSPEND_TYPES");
    assert_eq!(hsm::GET_SUSPEND_INFO, 5, "hsm::GET_SUSPEND_INFO");
    assert_eq!(hsm::HART_START, 6, "hsm::HART_START");
    assert_eq!(hsm::HART_STOP, 7, "hsm::HART_STOP");
    assert_eq!(hsm::HART_SUSPEND, 8, "hsm::HART_SUSPEND");
    assert_eq!(cppc::SERVICE_GROUP_ID, 6, "cppc::SERVICE_GROUP_ID");
    assert_eq!(cppc::ENABLE_NOTIFICATION, 1, "cppc::ENABLE_NOTIFICATION");
    assert_eq!(cppc::PROBE_REG, 2, "cppc::PROBE_REG");
    assert_eq!(cppc::READ_REG, 3, "cppc::READ_REG");
    assert_eq!(cppc::WRITE_REG, 4, "cppc::WRITE_REG");
    assert_eq!(
        cppc::GET_FAST_CHANNEL_REGION,
        5,
        "cppc::GET_FAST_CHANNEL_REGION"
    );
    assert_eq!(
        cppc::GET_FAST_CHANNEL_OFFSET,
        6,
        "cppc::GET_FAST_CHANNEL_OFFSET"
    );
    assert_eq!(cppc::GET_HART_LIST, 7, "cppc::GET_HART_LIST");
    assert_eq!(voltage::SERVICE_GROUP_ID, 7, "voltage::SERVICE_GROUP_ID");
    assert_eq!(
        voltage::ENABLE_NOTIFICATION,
        1,
        "voltage::ENABLE_NOTIFICATION"
    );
    assert_eq!(voltage::GET_NUM_DOMAINS, 2, "voltage::GET_NUM_DOMAINS");
    assert_eq!(voltage::GET_ATTRIBUTES, 3, "voltage::GET_ATTRIBUTES");
    assert_eq!(
        voltage::GET_SUPPORTED_LEVELS,
        4,
        "voltage::GET_SUPPORTED_LEVELS"
    );
    assert_eq!(voltage::SET_CONFIG, 5, "voltage::SET_CONFIG");
    assert_eq!(voltage::GET_CONFIG, 6, "voltage::GET_CONFIG");
    assert_eq!(voltage::SET_LEVEL, 7, "voltage::SET_LEVEL");
    assert_eq!(voltage::GET_LEVEL, 8, "voltage::GET_LEVEL");
    assert_eq!(clock::SERVICE_GROUP_ID, 8, "clock::SERVICE_GROUP_ID");
    assert_eq!(clock::ENABLE_NOTIFICATION, 1, "clock::ENABLE_NOTIFICATION");
    assert_eq!(clock::GET_NUM_CLOCKS, 2, "clock::GET_NUM_CLOCKS");
    assert_eq!(clock::GET_ATTRIBUTES, 3, "clock::GET_ATTRIBUTES");
    assert_eq!(clock::GET_SUPPORTED_RATES, 4, "clock::GET_SUPPORTED_RATES");
    assert_eq!(clock::SET_CONFIG, 5, "clock::SET_CONFIG");
    assert_eq!(clock::GET_CONFIG, 6, "clock::GET_CONFIG");
    assert_eq!(clock::SET_RATE, 7, "clock::SET_RATE");
    assert_eq!(clock::GET_RATE, 8, "clock::GET_RATE");
    assert_eq!(
        device_power::SERVICE_GROUP_ID,
        9,
        "device_power::SERVICE_GROUP_ID"
    );
    assert_eq!(
        device_power::ENABLE_NOTIFICATION,
        1,
        "device_power::ENABLE_NOTIFICATION"
    );
    assert_eq!(
        device_power::GET_NUM_DOMAINS,
        2,
        "device_power::GET_NUM_DOMAINS"
    );
    assert_eq!(
        device_power::GET_ATTRIBUTES,
        3,
        "device_power::GET_ATTRIBUTES"
    );
    assert_eq!(device_power::SET_STATE, 4, "device_power::SET_STATE");
    assert_eq!(device_power::GET_STATE, 5, "device_power::GET_STATE");
    assert_eq!(
        performance::SERVICE_GROUP_ID,
        10,
        "performance::SERVICE_GROUP_ID"
    );
    assert_eq!(
        performance::ENABLE_NOTIFICATION,
        1,
        "performance::ENABLE_NOTIFICATION"
    );
    assert_eq!(
        performance::GET_NUM_DOMAINS,
        2,
        "performance::GET_NUM_DOMAINS"
    );
    assert_eq!(
        performance::GET_ATTRIBUTES,
        3,
        "performance::GET_ATTRIBUTES"
    );
    assert_eq!(
        performance::GET_SUPPORTED_LEVELS,
        4,
        "performance::GET_SUPPORTED_LEVELS"
    );
    assert_eq!(performance::GET_LEVEL, 5, "performance::GET_LEVEL");
    assert_eq!(performance::SET_LEVEL, 6, "performance::SET_LEVEL");
    assert_eq!(performance::GET_LIMIT, 7, "performance::GET_LIMIT");
    assert_eq!(performance::SET_LIMIT, 8, "performance::SET_LIMIT");
    assert_eq!(
        performance::GET_FAST_CHANNEL_REGION,
        9,
        "performance::GET_FAST_CHANNEL_REGION"
    );
    assert_eq!(
        performance::GET_FAST_CHANNEL_ATTRIBUTES,
        10,
        "performance::GET_FAST_CHANNEL_ATTRIBUTES"
    );
    assert_eq!(
        management_mode::SERVICE_GROUP_ID,
        11,
        "management_mode::SERVICE_GROUP_ID"
    );
    assert_eq!(
        management_mode::ENABLE_NOTIFICATION,
        1,
        "management_mode::ENABLE_NOTIFICATION"
    );
    assert_eq!(
        management_mode::GET_ATTRIBUTES,
        2,
        "management_mode::GET_ATTRIBUTES"
    );
    assert_eq!(
        management_mode::COMMUNICATE,
        3,
        "management_mode::COMMUNICATE"
    );
    assert_eq!(
        ras_agent::SERVICE_GROUP_ID,
        12,
        "ras_agent::SERVICE_GROUP_ID"
    );
    assert_eq!(
        ras_agent::ENABLE_NOTIFICATION,
        1,
        "ras_agent::ENABLE_NOTIFICATION"
    );
    assert_eq!(
        ras_agent::GET_NUM_ERR_SRCS,
        2,
        "ras_agent::GET_NUM_ERR_SRCS"
    );
    assert_eq!(
        ras_agent::GET_ERR_SRCS_ID_LIST,
        3,
        "ras_agent::GET_ERR_SRCS_ID_LIST"
    );
    assert_eq!(
        ras_agent::GET_ERR_SRC_DESC,
        4,
        "ras_agent::GET_ERR_SRC_DESC"
    );
    assert_eq!(
        request_forward::SERVICE_GROUP_ID,
        13,
        "request_forward::SERVICE_GROUP_ID"
    );
    assert_eq!(
        request_forward::ENABLE_NOTIFICATION,
        1,
        "request_forward::ENABLE_NOTIFICATION"
    );
    assert_eq!(
        request_forward::RETRIEVE_CURRENT_MESSAGE,
        2,
        "request_forward::RETRIEVE_CURRENT_MESSAGE"
    );
    assert_eq!(
        request_forward::COMPLETE_CURRENT_MESSAGE,
        3,
        "request_forward::COMPLETE_CURRENT_MESSAGE"
    );
    assert_eq!(
        base::EventId::RequestHandleError as u8,
        1,
        "base::EventId::RequestHandleError as u8"
    );
    assert_eq!(
        request_forward::EventId::NewMessage as u8,
        1,
        "request_forward::EventId::NewMessage as u8"
    );
    assert_eq!(
        performance::EventId::PowerChange as u8,
        1,
        "performance::EventId::PowerChange as u8"
    );
    assert_eq!(
        performance::EventId::LimitChange as u8,
        2,
        "performance::EventId::LimitChange as u8"
    );
    assert_eq!(
        performance::EventId::LevelChange as u8,
        3,
        "performance::EventId::LevelChange as u8"
    );
    assert_eq!(size_of::<clock::RoundingMode>(), 4);
    assert_eq!(clock::RoundingMode::Down as u32, 0);
    assert_eq!(clock::RoundingMode::Up as u32, 1);
    assert_eq!(clock::RoundingMode::Auto as u32, 2);
    assert_eq!(
        device_power::PowerState::On as u32,
        0,
        "device_power::PowerState::On as u32"
    );
    assert_eq!(
        device_power::PowerState::Off as u32,
        3,
        "device_power::PowerState::Off as u32"
    );
    assert_eq!(
        device_power::STATE_CONTEXT_LOST,
        65536,
        "device_power::STATE_CONTEXT_LOST"
    );
}
