use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct FileDownload<'a> {
    pub name: &'a str,
    pub off: u64,
}

#[derive(Debug, Deserialize)]
pub struct FileDownloadResponse {
    pub off: u64,
    pub data: Vec<u8>,
    pub len: Option<u64>,
}

impl<'a> super::McuMgrRequest for FileDownload<'a> {
    type Response = FileDownloadResponse;

    const WRITE_OPERATION: bool = false;
    const GROUP_ID: u16 = 8;
    const COMMAND_ID: u8 = 0;
}
