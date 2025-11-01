fn main() {
    println!("Hello world!");

    for port in serialport::available_ports().unwrap() {
        println!("{:#?}", port);
    }

    let serial = serialport::new("/dev/ttyACM0", 115200).open().unwrap();
    let mut connection = zephyr_mcumgr::Connection::new(serial);

    connection.execute();
}
