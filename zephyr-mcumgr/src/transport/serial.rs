use base64::prelude::*;

use super::{SMP_HEADER_SIZE, SMP_TRANSFER_BUFFER_SIZE, SendError, Transport};

pub struct SerialTransport<T> {
    transfer_buffer: Box<[u8]>,
    body_buffer: Box<[u8]>,
    serial: T,
    crc_algo: crc::Crc<u16>,
}

impl<T> SerialTransport<T> {
    pub fn new(serial: T, mtu: usize) -> Self {
        Self {
            serial,
            transfer_buffer: vec![0u8; mtu].into_boxed_slice(),
            body_buffer: vec![0u8; ((mtu - 3) / 4) * 3].into_boxed_slice(),
            crc_algo: crc::Crc::<u16>::new(&crc::CRC_16_XMODEM),
        }
    }
}

// TODO refactor body assembly into separate testable struct

impl<T> Transport for SerialTransport<T>
where
    T: std::io::Write + std::io::Read,
{
    fn send_raw_frame(
        &mut self,
        header: [u8; SMP_HEADER_SIZE],
        mut data: &[u8],
    ) -> Result<(), SendError> {
        let checksum = {
            let mut digest = self.crc_algo.digest();
            digest.update(&header);
            digest.update(data);
            digest.finalize()
        };

        let mut is_first = true;
        while is_first || !data.is_empty() {
            let body_header;
            let mut body;
            let mut body_footer;

            if is_first {
                let size: u16 = (SMP_HEADER_SIZE + data.len() + 2)
                    .try_into()
                    .map_err(|_| SendError::DataTooBig)?;

                (body_header, body) = self.body_buffer.split_at_mut(2 + SMP_HEADER_SIZE);
                let (body_header_1, body_header_2) = body_header.split_at_mut(2);
                body_header_1.copy_from_slice(&size.to_be_bytes());
                body_header_2.copy_from_slice(&header);
            } else {
                body_header = &mut [];
                body = &mut self.body_buffer;
            }

            let body_len = body.len().min(data.len());
            (body, body_footer) = body.split_at_mut(body_len);
            if body_footer.len() >= 2 {
                (body_footer, _) = body_footer.split_at_mut(2);
                body_footer.copy_from_slice(&checksum.to_be_bytes())
            } else {
                body_footer = &mut [];
            }

            let chunk;
            (chunk, data) = data.split_at(body_len);
            body.copy_from_slice(chunk);

            let assembled_body_len = body_header.len() + body.len() + body_footer.len();

            let assembled_body = &self.body_buffer[..assembled_body_len];

            if is_first {
                self.transfer_buffer[0] = 6;
                self.transfer_buffer[1] = 9;
            } else {
                self.transfer_buffer[0] = 4;
                self.transfer_buffer[1] = 20;
            }
            let base64_body_len = BASE64_STANDARD
                .encode_slice(assembled_body, &mut self.transfer_buffer[2..])
                .unwrap();
            self.transfer_buffer[base64_body_len + 2] = 0x0a;

            self.serial
                .write_all(&self.transfer_buffer[..base64_body_len + 3])?;

            is_first = false;
        }

        Ok(())
    }

    fn recv_raw_frame(&mut self, buffer: &[u8; SMP_TRANSFER_BUFFER_SIZE]) {
        todo!()
    }
}
