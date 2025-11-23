use serde::Serialize;

/// Information about the bootloader
#[derive(Serialize)]
pub enum BootloaderInfo {
    /// MCUboot bootloader
    MCUboot {
        /// Bootloader mode
        ///
        /// See [TODO] for more information.
        mode: i32,
        /// Bootloader has downgrade prevention enabled
        no_downgrade: bool,
    },
    /// Other bootloader
    Other {
        /// Name of the bootloader
        name: String,
    },
}

/// MCUboot modes
///
/// See [`enum mcuboot_mode`](https://github.com/mcu-tools/mcuboot/blob/main/boot/bootutil/include/bootutil/boot_status.h).
#[derive(
    strum::FromRepr, strum::IntoStaticStr, strum::Display, Debug, Copy, Clone, PartialEq, Eq,
)]
#[repr(i32)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub enum MCUbootMode {
    MCUBOOT_MODE_SINGLE_SLOT = 0,
    MCUBOOT_MODE_SWAP_USING_SCRATCH,
    MCUBOOT_MODE_UPGRADE_ONLY,
    MCUBOOT_MODE_SWAP_USING_MOVE,
    MCUBOOT_MODE_DIRECT_XIP,
    MCUBOOT_MODE_DIRECT_XIP_WITH_REVERT,
    MCUBOOT_MODE_RAM_LOAD,
    MCUBOOT_MODE_FIRMWARE_LOADER,
    MCUBOOT_MODE_SINGLE_SLOT_RAM_LOAD,
    MCUBOOT_MODE_SWAP_USING_OFFSET,
}
