use std::time::Duration;

use zephyr_mcumgr::{
    commands,
    transport::{SERIAL_TRANSPORT_DEFAULT_MTU, SerialTransport},
};

fn main() -> miette::Result<()> {
    let serial = serialport::new("/tmp/interceptty", 115200)
        .timeout(Duration::from_millis(500))
        .open()
        .unwrap();

    let mut connection =
        zephyr_mcumgr::Connection::new(SerialTransport::new(serial, SERIAL_TRANSPORT_DEFAULT_MTU));

    let result = connection.execute_cbor(&commands::os::Echo { d: "Hello world!" })?;
    println!("{result:?}");

    Ok(())
}
