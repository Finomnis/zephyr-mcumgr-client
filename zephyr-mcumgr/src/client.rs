use std::io::{self, Read, Write};

use miette::Diagnostic;
use thiserror::Error;

use crate::{
    commands,
    connection::{Connection, ExecuteError},
    transport::{SERIAL_TRANSPORT_DEFAULT_MTU, SerialTransport},
};

pub struct MCUmgrClient {
    connection: Connection,
}

#[derive(Error, Debug, Diagnostic)]
pub enum FileDownloadError {
    #[error("transport error")]
    #[diagnostic(code(zephyr_mcumgr::client::file_download::execute))]
    ExecuteError(#[from] ExecuteError),
    #[error("received offset does not match requested offset")]
    #[diagnostic(code(zephyr_mcumgr::client::file_download::offset_mismatch))]
    UnexpectedOffset,
    #[error("writer returned an error")]
    #[diagnostic(code(zephyr_mcumgr::client::file_download::writer))]
    WriterError(#[from] io::Error),
    #[error("received data does not match reported size")]
    #[diagnostic(code(zephyr_mcumgr::client::file_download::size_mismatch))]
    SizeMismatch,
    #[error("received data is missing file size information")]
    #[diagnostic(code(zephyr_mcumgr::client::file_download::missing_size))]
    MissingSize,
}

impl MCUmgrClient {
    pub fn from_serial<T: Read + Write + 'static>(serial: T) -> Self {
        Self {
            connection: Connection::new(SerialTransport::new(serial, SERIAL_TRANSPORT_DEFAULT_MTU)),
        }
    }

    pub fn os_echo(&mut self, msg: impl AsRef<str>) -> Result<String, ExecuteError> {
        self.connection
            .execute_cbor(&commands::os::Echo { d: msg.as_ref() })
            .map(|resp| resp.r)
    }

    pub fn fs_file_download<T: Write>(
        &mut self,
        name: impl AsRef<str>,
        mut writer: T,
    ) -> Result<(), FileDownloadError> {
        let name = name.as_ref();
        let response = self
            .connection
            .execute_cbor(&commands::fs::FileDownload { name, off: 0 })?;

        let file_len = response.len.ok_or(FileDownloadError::MissingSize)?;
        if response.off != 0 {
            return Err(FileDownloadError::UnexpectedOffset);
        }

        let mut offset = 0;

        writer.write_all(&response.data)?;
        offset += response.data.len() as u64;

        while offset < file_len {
            let response = self
                .connection
                .execute_cbor(&commands::fs::FileDownload { name, off: offset })?;

            if response.off != offset {
                return Err(FileDownloadError::UnexpectedOffset);
            }

            writer.write_all(&response.data)?;
            offset += response.data.len() as u64;
        }

        if offset != file_len {
            return Err(FileDownloadError::SizeMismatch);
        }

        Ok(())
    }
}
