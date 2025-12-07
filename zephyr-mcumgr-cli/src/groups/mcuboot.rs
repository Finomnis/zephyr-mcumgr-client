use crate::{args::CommonArgs, client::Client, errors::CliError, file_read_write::read_input_file};

#[derive(Debug, clap::Subcommand)]
pub enum MCUbootCommand {
    /// Shows information about an MCUboot image file
    GetImageInfo {
        /// The image file to analyze. '-' for stdin.
        file: String,
    },
}

pub fn run(_client: &Client, _args: CommonArgs, command: MCUbootCommand) -> Result<(), CliError> {
    match command {
        MCUbootCommand::GetImageInfo { file } => {
            let (image_data, _source_filename) = read_input_file(&file)?;
            let image_info =
                zephyr_mcumgr::mcuboot::image::parse(std::io::Cursor::new(image_data.as_ref()))?;
            println!("{:?}", image_info);
        }
    }

    Ok(())
}
