use std::time::Duration;

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
        zephyr_mcumgr::Connection::new(zephyr_mcumgr::transport::SerialTransport::new(serial, 127));

    connection.execute_raw(true, 0, 0, &[])?;

    Ok(())
}
