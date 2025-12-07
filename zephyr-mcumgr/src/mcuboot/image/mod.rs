/// Information about an MCUboot firmware image
#[derive(Debug, Clone)]
pub struct ImageInfo {}

/// Possible error values of [`image::parse`](parse).
#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum ImageParseError {
    /// The given image file does is not an MCUboot image.
    #[error("The given image is not an an MCUboot image")]
    #[diagnostic(code(zephyr_mcumgr::mcuboot::image::unknown_type))]
    UnknownImageType,
    /// Failed to read from the image
    #[error("Image read failed")]
    #[diagnostic(code(zephyr_mcumgr::mcuboot::image::read))]
    ReadFailed(#[from] std::io::Error),
}

fn read_u32(data: &mut dyn std::io::Read) -> Result<u32, std::io::Error> {
    let mut bytes = [0u8; 4];
    data.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

/// The identifying header of an MCUboot image
pub const IMAGE_MAGIC: u32 = 0x96f3b83d;

/// Parses an MCUboot image
pub fn parse(mut image_data: impl std::io::Read) -> Result<ImageInfo, ImageParseError> {
    let image_data = &mut image_data;

    let magic = read_u32(image_data)?;
    if magic != IMAGE_MAGIC {
        println!("{:x?}", magic);
        return Err(ImageParseError::UnknownImageType);
    }

    Ok(ImageInfo {})
}
