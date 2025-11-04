use std::{fmt::Display, io::Cursor};

use crate::{
    commands::{ErrResponse, ErrResponseV2, McuMgrCommand},
    transport::{ReceiveError, SendError, Transport},
};

use miette::Diagnostic;
use thiserror::Error;

pub struct Connection {
    transport: Box<dyn Transport + Send>,
    next_seqnum: u8,
    transport_buffer: [u8; u16::MAX as usize],
}

#[derive(Debug)]
pub enum DeviceError {
    V1 { rc: i32 },
    V2 { group: u32, rc: u32 },
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceError::V1 { rc } => {
                let err_str = match *rc {
                    0 => "MGMT_ERR_EOK".to_string(),
                    1 => "MGMT_ERR_EUNKNOWN".to_string(),
                    2 => "MGMT_ERR_ENOMEM".to_string(),
                    3 => "MGMT_ERR_EINVAL".to_string(),
                    4 => "MGMT_ERR_ETIMEOUT".to_string(),
                    5 => "MGMT_ERR_ENOENT".to_string(),
                    6 => "MGMT_ERR_EBADSTATE".to_string(),
                    7 => "MGMT_ERR_EMSGSIZE".to_string(),
                    8 => "MGMT_ERR_ENOTSUP".to_string(),
                    9 => "MGMT_ERR_ECORRUPT".to_string(),
                    10 => "MGMT_ERR_EBUSY".to_string(),
                    11 => "MGMT_ERR_EACCESSDENIED".to_string(),
                    12 => "MGMT_ERR_UNSUPPORTED_TOO_OLD".to_string(),
                    13 => "MGMT_ERR_UNSUPPORTED_TOO_NEW".to_string(),
                    256.. => format!("MGMT_ERR_EPERUSER({rc})"),
                    _ => format!("Unknown({rc})"),
                };
                write!(f, "V1({err_str})")
            }
            DeviceError::V2 { group, rc } => write!(f, "V2(group={group},rc={rc}"),
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum ExecuteError {
    #[error("sending failed")]
    #[diagnostic(code(zephyr_mcumgr::connection::execute::send))]
    SendFailed(#[from] SendError),
    #[error("receiving failed")]
    #[diagnostic(code(zephyr_mcumgr::connection::execute::receive))]
    ReceiveFailed(#[from] ReceiveError),
    #[error("cbor encoding failed")]
    #[diagnostic(code(zephyr_mcumgr::connection::execute::encode))]
    EncodeFailed,
    #[error("cbor decoding failed")]
    #[diagnostic(code(zephyr_mcumgr::connection::execute::decode))]
    DecodeFailed,
    #[error("device returned error {0}")]
    #[diagnostic(code(zephyr_mcumgr::connection::execute::device_error))]
    ErrorResponse(DeviceError),
}

impl Connection {
    pub fn new<T: Transport + Send + 'static>(transport: T) -> Self {
        Self {
            transport: Box::new(transport),
            next_seqnum: rand::random(),
            transport_buffer: [0; u16::MAX as usize],
        }
    }

    pub fn execute_command<R: McuMgrCommand>(
        &mut self,
        request: &R,
    ) -> Result<R::Response, ExecuteError> {
        let mut cursor = Cursor::new(self.transport_buffer.as_mut_slice());
        ciborium::into_writer(request, &mut cursor).map_err(|_| ExecuteError::EncodeFailed)?;
        let data_size = cursor.position() as usize;
        let data = &self.transport_buffer[..data_size];

        log::debug!(
            "TX data: {}",
            data.iter().map(|e| format!("{e:02x}")).collect::<String>()
        );

        let sequence_num = self.next_seqnum;
        self.next_seqnum = self.next_seqnum.wrapping_add(1);

        self.transport.send_frame(
            R::WRITE_OPERATION,
            sequence_num,
            R::GROUP_ID,
            R::COMMAND_ID,
            data,
        )?;

        let response = self.transport.receive_frame(
            &mut self.transport_buffer,
            R::WRITE_OPERATION,
            sequence_num,
            R::GROUP_ID,
            R::COMMAND_ID,
        )?;

        log::debug!(
            "RX data: {}",
            response
                .iter()
                .map(|e| format!("{e:02x}"))
                .collect::<String>()
        );

        let err: ErrResponse =
            ciborium::from_reader(Cursor::new(response)).map_err(|_| ExecuteError::DecodeFailed)?;

        if let Some(ErrResponseV2 { rc, group }) = err.err {
            return Err(ExecuteError::ErrorResponse(DeviceError::V2 { group, rc }));
        }

        if let Some(rc) = err.rc {
            return Err(ExecuteError::ErrorResponse(DeviceError::V1 { rc }));
        }

        let decoded_response: R::Response =
            ciborium::from_reader(Cursor::new(response)).map_err(|_| ExecuteError::DecodeFailed)?;

        Ok(decoded_response)
    }

    pub fn execute_raw_command(
        &mut self,
        write_operation: bool,
        group_id: u16,
        command_id: u8,
        data: &[u8],
    ) -> Result<&[u8], ExecuteError> {
        let sequence_num = self.next_seqnum;
        self.next_seqnum = self.next_seqnum.wrapping_add(1);

        self.transport
            .send_frame(write_operation, sequence_num, group_id, command_id, data)?;

        self.transport
            .receive_frame(
                &mut self.transport_buffer,
                write_operation,
                sequence_num,
                group_id,
                command_id,
            )
            .map_err(Into::into)
    }
}
