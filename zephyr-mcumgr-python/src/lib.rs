#![forbid(unsafe_code)]

use pyo3::prelude::*;

use ::zephyr_mcumgr::commands;
use pyo3::exceptions::PyRuntimeError;
use pyo3_stub_gen::{
    define_stub_info_gatherer,
    derive::{gen_stub_pyclass, gen_stub_pymethods},
};
use std::error::Error;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

/// A high level client for Zephyr's MCUmgr SMP functionality
#[gen_stub_pyclass]
#[pyclass(frozen)]
struct MCUmgrClient {
    client: Mutex<::zephyr_mcumgr::MCUmgrClient>,
}

fn convert_error<T, E: Error>(res: Result<T, E>) -> PyResult<T> {
    res.map_err(|e| PyRuntimeError::new_err(format!("{e}")))
}

impl MCUmgrClient {
    fn lock(&self) -> PyResult<MutexGuard<'_, ::zephyr_mcumgr::MCUmgrClient>> {
        let res = self.client.lock();
        convert_error(res)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl MCUmgrClient {
    /// Creates a new serial port based Zephyr MCUmgr SMP client.
    ///
    ///  # Arguments
    ///
    /// * `serial` - The identifier of the serial device. (Windows: `COMxx`, Linux: `/dev/ttyXX`)
    /// * `baud_rate` - The baud rate of the serial port.
    /// * `timeout_ms` - The communication timeout, in ms.
    ///
    #[staticmethod]
    #[pyo3(signature = (serial, baud_rate=115200, timeout_ms=500))]
    fn new_from_serial(serial: &str, baud_rate: u32, timeout_ms: u64) -> PyResult<Self> {
        let serial = serialport::new(serial, baud_rate)
            .timeout(Duration::from_millis(timeout_ms))
            .open();
        let serial = convert_error(serial)?;
        let client = ::zephyr_mcumgr::MCUmgrClient::new_from_serial(serial);
        Ok(MCUmgrClient {
            client: Mutex::new(client),
        })
    }

    /// Configures the maximum SMP frame size that we can send to the device.
    ///
    /// Must not exceed [`MCUMGR_TRANSPORT_NETBUF_SIZE`](https://github.com/zephyrproject-rtos/zephyr/blob/v4.2.1/subsys/mgmt/mcumgr/transport/Kconfig#L40),
    /// otherwise we might crash the device.
    fn set_frame_size(&self, smp_frame_size: usize) -> PyResult<()> {
        Ok(self.lock()?.set_frame_size(smp_frame_size))
    }

    /// Configures the maximum SMP frame size that we can send to the device automatically
    /// by reading the value of [`MCUMGR_TRANSPORT_NETBUF_SIZE`](https://github.com/zephyrproject-rtos/zephyr/blob/v4.2.1/subsys/mgmt/mcumgr/transport/Kconfig#L40)
    /// from the device.
    pub fn use_auto_frame_size(&self) -> PyResult<()> {
        let res = self.lock()?.use_auto_frame_size();
        convert_error(res)
    }

    /// Changes the communication timeout.
    ///
    /// When the device does not respond to packets within the set
    /// duration, an error will be raised.
    pub fn set_timeout(&self, timeout: Duration) -> PyResult<()> {
        let res = self.lock()?.set_timeout(timeout);
        // Chenanigans because Box<dyn Error> does not implement Error
        let res = match &res {
            Ok(()) => Ok(()),
            Err(e) => Err(&**e),
        };
        convert_error(res)
    }

    /// Sends a message to the device and expects the same message back as response.
    ///
    /// This can be used as a sanity check for whether the device is connected and responsive.
    fn os_echo(&self, msg: &str) -> PyResult<String> {
        let res = self.lock()?.os_echo(msg);
        convert_error(res)
    }

    // TODO: file download
    // TODO: file upload

    /// Run a shell command.
    ///
    ///  # Arguments
    ///
    /// * `argv` - The shell command to be executed.
    ///
    /// # Return
    ///
    /// A tuple of (returncode, stdout) produced by the command execution.
    pub fn shell_execute(&self, argv: Vec<String>) -> PyResult<(i32, String)> {
        let res = self.lock()?.shell_execute(&argv);
        convert_error(res)
    }

    /// Execute a raw [`commands::McuMgrCommand`].
    ///
    /// Only returns if no error happened, so the
    /// user does not need to check for an `rc` or `err`
    /// field in the response.
    pub fn raw_command(&self, command: i32) -> PyResult<i32> {
        //self.connection.execute_command(command)
        Ok(42)
    }
}

#[pymodule]
mod zephyr_mcumgr {
    #[pymodule_export]
    use super::MCUmgrClient;
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
