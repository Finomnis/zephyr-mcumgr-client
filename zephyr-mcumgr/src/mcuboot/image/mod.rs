/// The firmware version
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ImageVersion {
    /// Major version
    pub major: u8,
    /// Minor version
    pub minor: u8,
    /// Revision
    pub revision: u16,
    /// Build number
    pub build_num: u32,
}
impl std::fmt::Display for ImageVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revision)?;
        if self.build_num != 0 {
            write!(f, ".{}", self.build_num)?;
        }
        Ok(())
    }
}

/// Information about an MCUboot firmware image
#[derive(Debug, Clone)]
pub struct ImageInfo {
    /// Firmware version
    pub version: ImageVersion,
}

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

struct OffsetTrackingReader<'a> {
    read: &'a mut dyn std::io::Read,
    offset: usize,
}
impl<'a> OffsetTrackingReader<'a> {
    fn new(read: &'a mut dyn std::io::Read) -> Self {
        Self { read, offset: 0 }
    }
}
impl std::io::Read for OffsetTrackingReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = std::io::Read::read(self.read, buf)?;
        self.offset += size;
        Ok(size)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        std::io::Read::read_exact(self.read, buf)?;
        self.offset += buf.len();
        Ok(())
    }
}

fn read_u32(data: &mut dyn std::io::Read) -> Result<u32, std::io::Error> {
    let mut bytes = [0u8; 4];
    data.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

fn read_u16(data: &mut dyn std::io::Read) -> Result<u16, std::io::Error> {
    let mut bytes = [0u8; 2];
    data.read_exact(&mut bytes)?;
    Ok(u16::from_le_bytes(bytes))
}

fn read_u8(data: &mut dyn std::io::Read) -> Result<u8, std::io::Error> {
    let mut byte = 0u8;
    data.read_exact(std::slice::from_mut(&mut byte))?;
    Ok(byte)
}

/// The identifying header of an MCUboot image
pub const IMAGE_MAGIC: u32 = 0x96f3b83d;

/// Parses an MCUboot image
pub fn parse(mut image_data: impl std::io::Read) -> Result<ImageInfo, ImageParseError> {
    let image_data = &mut OffsetTrackingReader::new(&mut image_data);

    let ih_magic = read_u32(image_data)?;
    log::debug!("ih_magic: 0x{ih_magic:08x}");
    if ih_magic != IMAGE_MAGIC {
        return Err(ImageParseError::UnknownImageType);
    }

    let ih_load_addr = read_u32(image_data)?;
    log::debug!("ih_load_addr: 0x{ih_load_addr:08x}");

    let ih_hdr_size = read_u16(image_data)?;
    log::debug!("ih_hdr_size: 0x{ih_hdr_size:04x}");

    let ih_protect_tlv_size = read_u16(image_data)?;
    log::debug!("ih_protect_tlv_size: 0x{ih_protect_tlv_size:04x}");

    let ih_img_size = read_u32(image_data)?;
    log::debug!("ih_img_size: 0x{ih_img_size:08x}");

    let ih_flags = read_u32(image_data)?;
    log::debug!("ih_flags: 0x{ih_flags:08x}");

    let ih_ver = ImageVersion {
        major: read_u8(image_data)?,
        minor: read_u8(image_data)?,
        revision: read_u16(image_data)?,
        build_num: read_u32(image_data)?,
    };
    log::debug!("ih_ver: {ih_ver:?}");

    Ok(ImageInfo { version: ih_ver })
}
