use base64::prelude::*;

use super::{SMP_HEADER_SIZE, SMP_TRANSFER_BUFFER_SIZE, SendError, Transport};

pub struct SerialTransport<T> {
    transfer_buffer: Box<[u8]>,
    body_buffer: Box<[u8]>,
    serial: T,
    crc_algo: crc::Crc<u16>,
}

fn fill_buffer_with_data<'a, I: Iterator<Item = u8>>(
    buffer: &'a mut [u8],
    data_iter: &mut I,
) -> &'a [u8] {
    for (pos, val) in buffer.iter_mut().enumerate() {
        if let Some(next) = data_iter.next() {
            *val = next;
        } else {
            return &buffer[..pos];
        }
    }

    buffer
}

impl<T> SerialTransport<T>
where
    T: std::io::Write + std::io::Read,
{
    pub fn new(serial: T, mtu: usize) -> Self {
        Self {
            serial,
            transfer_buffer: vec![0u8; mtu].into_boxed_slice(),
            body_buffer: vec![0u8; ((mtu - 3) / 4) * 3].into_boxed_slice(),
            crc_algo: crc::Crc::<u16>::new(&crc::CRC_16_XMODEM),
        }
    }

    fn send_chunked<I: Iterator<Item = u8>>(&mut self, mut data_iter: I) -> Result<(), SendError> {
        self.transfer_buffer[0] = 6;
        self.transfer_buffer[1] = 9;

        loop {
            let body = fill_buffer_with_data(&mut self.body_buffer, &mut data_iter);

            if body.is_empty() {
                break Ok(());
            }

            let base64_len = BASE64_STANDARD
                .encode_slice(body, &mut self.transfer_buffer[2..])
                .expect("Transfer buffer overflow; this is a bug. Please report.");

            self.transfer_buffer[base64_len + 2] = 0x0a;

            self.serial
                .write_all(&self.transfer_buffer[..base64_len + 3])?;

            self.transfer_buffer[0] = 4;
            self.transfer_buffer[1] = 20;
        }
    }
}

impl<T> Transport for SerialTransport<T>
where
    T: std::io::Write + std::io::Read,
{
    fn send_raw_frame(
        &mut self,
        header: [u8; SMP_HEADER_SIZE],
        data: &[u8],
    ) -> Result<(), SendError> {
        let checksum = {
            let mut digest = self.crc_algo.digest();
            digest.update(&header);
            digest.update(data);
            digest.finalize().to_be_bytes()
        };

        let size = u16::try_from(header.len() + data.len() + checksum.len())
            .map_err(|_| SendError::DataTooBig)?
            .to_be_bytes();

        self.send_chunked(
            size.into_iter()
                .chain(header)
                .chain(data.iter().copied())
                .chain(checksum),
        )
    }

    fn recv_raw_frame(&mut self, buffer: &[u8; SMP_TRANSFER_BUFFER_SIZE]) {
        todo!()
    }
}
