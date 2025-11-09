/// [File management](https://docs.zephyrproject.org/latest/services/device_mgmt/smp_groups/smp_group_8.html) group commands
pub mod fs;
/// [Default/OS management](https://docs.zephyrproject.org/latest/services/device_mgmt/smp_groups/smp_group_0.html) group commands
pub mod os;
/// [Shell management](https://docs.zephyrproject.org/latest/services/device_mgmt/smp_groups/smp_group_9.html) group commands
pub mod shell;

use serde::{Deserialize, Serialize};

/// SMP version 2 group based error message
#[derive(Debug, Deserialize)]
pub struct ErrResponseV2 {
    /// group of the group-based error code
    pub group: u16,
    /// contains the index of the group-based error code
    pub rc: i32,
}

/// [SMP error message](https://docs.zephyrproject.org/latest/services/device_mgmt/smp_protocol.html#minimal-response-smp-data)
#[derive(Debug, Deserialize)]
pub struct ErrResponse {
    /// SMP version 1 error code
    pub rc: Option<i32>,
    /// SMP version 2 error message
    pub err: Option<ErrResponseV2>,
}

/// An MCUmgr command that can be executed through [`Connection::execute_command`](crate::connection::Connection::execute_command).
pub trait McuMgrCommand {
    /// the data payload type
    type Payload: Serialize;
    /// the response type of the command
    type Response: for<'a> Deserialize<'a>;
    /// whether this command is a read or write operation
    fn is_write_operation(&self) -> bool;
    /// the group ID of the command
    fn group_id(&self) -> u16;
    /// the command ID
    fn command_id(&self) -> u8;
    /// the data
    fn data(&self) -> &Self::Payload;
}

/// Implements the [`McuMgrCommand`] trait for a request/response pair.
///
/// # Parameters
/// - `$request`: The request type implementing the command
/// - `$response`: The response type for this command
/// - `$iswrite`: Boolean literal indicating if this is a write operation
/// - `$groupid`: The MCUmgr group ID (u16)
/// - `$commandid`: The MCUmgr command ID (u8)
macro_rules! impl_mcumgr_command {
    ($request:ty, $response:ty, $iswrite:literal, $groupid:literal, $commandid:literal) => {
        impl McuMgrCommand for $request {
            type Payload = Self;
            type Response = $response;
            fn is_write_operation(&self) -> bool {
                $iswrite
            }
            fn group_id(&self) -> u16 {
                $groupid
            }
            fn command_id(&self) -> u8 {
                $commandid
            }
            fn data(&self) -> &Self {
                self
            }
        }
    };
}

impl_mcumgr_command!(os::Echo<'_>, os::EchoResponse, true, 0, 0);
impl_mcumgr_command!(os::TaskStatistics, os::TaskStatisticsResponse, false, 0, 2);
impl_mcumgr_command!(
    os::MCUmgrParameters,
    os::MCUmgrParametersResponse,
    false,
    0,
    6
);

impl_mcumgr_command!(fs::FileUpload<'_, '_>, fs::FileUploadResponse, true, 8, 0);
impl_mcumgr_command!(fs::FileDownload<'_>, fs::FileDownloadResponse, false, 8, 0);

impl_mcumgr_command!(
    shell::ShellCommandLineExecute<'_>,
    shell::ShellCommandLineExecuteResponse,
    true,
    9,
    0
);
