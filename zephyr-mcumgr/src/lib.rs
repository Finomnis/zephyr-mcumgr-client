#![deny(unreachable_pub)]
#![deny(missing_docs)]
#![doc(
    issue_tracker_base_url = "https://github.com/Finomnis/zephyr-mcumgr-client/issues",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code)))
)]

mod client;

pub mod commands;
pub mod connection;
pub mod transport;

pub use client::MCUmgrClient;
