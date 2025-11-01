use crate::transport::{ReceiveError, SendError, Transport};

use miette::Diagnostic;
use thiserror::Error;

pub struct Connection<T: Transport> {
    transport: T,
    next_seqnum: u8,
    transport_buffer: [u8; u16::MAX as usize],
}

#[derive(Error, Debug, Diagnostic)]
pub enum ExecuteError {
    #[error("sending failed")]
    #[diagnostic(code(zephyr_mcumgr::connection::send::execute::send))]
    SendError(#[from] SendError),
    #[error("receiving failed")]
    #[diagnostic(code(zephyr_mcumgr::connection::send::execute::receive))]
    ReceiveError(#[from] ReceiveError),
}

impl<T: Transport> Connection<T> {
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            next_seqnum: rand::random(),
            transport_buffer: [0; u16::MAX as usize],
        }
    }

    pub fn execute_cbor(&mut self) {
        //self.transport.send_frame(true, 0, 0, 0, data).unwrap();
    }

    pub fn execute_raw(
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
