use clap::{Parser, Subcommand};

/// Command line client for Zephyr's MCUmgr SMP protocol
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(disable_help_subcommand = true)]
pub struct App {
    /// Use the given serial port as backend
    #[arg(short, long)]
    pub serial: Option<String>,

    /// Serial port baud rate
    #[arg(short, long, default_value_t = 115200)]
    pub baud: u32,

    /// Communication timeout (in ms)
    #[arg(short, long, default_value_t = 500)]
    pub timeout: u64,

    /// Command group
    #[command(subcommand)]
    pub group: Group,
}

#[derive(Debug, Subcommand)]
pub enum OsCommand {
    /// Executes an echo command on the device
    Echo {
        /// The message to echo
        msg: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum FsCommand {}

#[derive(Debug, Subcommand)]
pub enum Group {
    /// Default/OS Management
    Os {
        #[command(subcommand)]
        command: OsCommand,
    },
    /// File Management
    Fs {
        #[command(subcommand)]
        command: FsCommand,
    },
}
