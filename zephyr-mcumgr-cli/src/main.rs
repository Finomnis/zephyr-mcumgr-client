fn main() {
    println!("Hello world!");

    for port in serialport::available_ports().unwrap() {
        println!("{:#?}", port);
    }

    zephyr_mcumgr::Connection::new(serialport::new("/dev/ttyACM0", 115200).open().unwrap());
}
