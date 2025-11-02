use std::time::Duration;

use zephyr_mcumgr::transport::{SERIAL_TRANSPORT_DEFAULT_MTU, SerialTransport};

fn main() -> miette::Result<()> {
    println!("Hello world!");

    for port in serialport::available_ports().unwrap() {
        //println!("{port:#?}");
    }

    let serial = serialport::new("/tmp/interceptty", 115200)
        .timeout(Duration::from_millis(500))
        .open()
        .unwrap();

    let mut connection =
        zephyr_mcumgr::Connection::new(SerialTransport::new(serial, SERIAL_TRANSPORT_DEFAULT_MTU));

    let result = connection.execute_raw(true, 0, 0, &[])?;

    println!("{result:0x?}");

    Ok(())
}
