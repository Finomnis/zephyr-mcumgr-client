use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Echo<'a> {
    pub d: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct EchoResponse {
    pub r: String,
}

impl<'a> super::McuMgrRequest for Echo<'a> {
    type Response = EchoResponse;

    const WRITE_OPERATION: bool = true;
    const GROUP_ID: u16 = 0;
    const COMMAND_ID: u8 = 0;
}
