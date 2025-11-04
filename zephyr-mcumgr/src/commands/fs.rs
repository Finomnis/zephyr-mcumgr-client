use serde::{Deserialize, Serialize};

/// [File Download](https://docs.zephyrproject.org/latest/services/device_mgmt/smp_groups/smp_group_8.html#file-download) command
#[derive(Debug, Serialize)]
pub struct FileDownload<'a> {
    /// offset to start download at
    pub off: u64,
    /// absolute path to a file
    pub name: &'a str,
}

/// Response for [`FileDownload`] command
#[derive(Debug, Deserialize)]
pub struct FileDownloadResponse {
    /// offset the response is for
    pub off: u64,
    /// chunk of data read from file; it is CBOR encoded stream of bytes with embedded size
    pub data: Vec<u8>,
    /// length of file, this field is only mandatory when “off” is 0
    pub len: Option<u64>,
}

impl<'a> super::McuMgrCommand for FileDownload<'a> {
    type Response = FileDownloadResponse;

    const WRITE_OPERATION: bool = false;
    const GROUP_ID: u16 = 8;
    const COMMAND_ID: u8 = 0;
}

/// Computes how large [`FileUpload::data`] is allowed to be.
///
/// Taken from Zephyr's [MCUMGR_GRP_FS_DL_CHUNK_SIZE](https://github.com/zephyrproject-rtos/zephyr/blob/v4.2.1/subsys/mgmt/mcumgr/grp/fs_mgmt/include/mgmt/mcumgr/grp/fs_mgmt/fs_mgmt_config.h#L45).
///
/// # Arguments
///
/// * `smp_frame_size` - The max allowed size of an SMP frame.
pub const fn file_upload_max_data_chunk_size(smp_frame_size: usize) -> usize {
    const MCUMGR_GRP_FS_MAX_OFFSET_LEN: usize = std::mem::size_of::<u64>();
    const MGMT_HDR_SIZE: usize = 8; // Size of SMP header
    const CBOR_AND_OTHER_HDR: usize = MGMT_HDR_SIZE
        + (9 + 1)
        + (1 + 3 + MCUMGR_GRP_FS_MAX_OFFSET_LEN)
        + (1 + 4 + MCUMGR_GRP_FS_MAX_OFFSET_LEN)
        + (1 + 2 + 1)
        + (1 + 3 + MCUMGR_GRP_FS_MAX_OFFSET_LEN);

    smp_frame_size - CBOR_AND_OTHER_HDR
}

/// [File Upload](https://docs.zephyrproject.org/latest/services/device_mgmt/smp_groups/smp_group_8.html#file-upload) command
#[derive(Debug, Serialize)]
pub struct FileUpload<'a, 'b> {
    /// offset to start/continue upload at
    pub off: u64,
    /// chunk of data to write to the file; it is CBOR encoded with length embedded
    #[serde(with = "serde_bytes")]
    pub data: &'a [u8],
    /// absolute path to a file
    pub name: &'b str,
    /// length of file, this field is only mandatory when “off” is 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<u64>,
}

/// Response for [`FileUpload`] command
#[derive(Debug, Deserialize)]
pub struct FileUploadResponse {
    /// offset of last successfully written data
    pub off: u64,
}

impl<'a, 'b> super::McuMgrCommand for FileUpload<'a, 'b> {
    type Response = FileUploadResponse;

    const WRITE_OPERATION: bool = true;
    const GROUP_ID: u16 = 8;
    const COMMAND_ID: u8 = 0;
}
