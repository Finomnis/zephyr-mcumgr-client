use strum_macros::{Display, FromRepr};

/// See [`enum mcumgr_group_t`](https://docs.zephyrproject.org/latest/doxygen/html/mgmt__defines_8h.html).
#[derive(FromRepr, Display, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub enum MCUmgrGroup {
    MGMT_GROUP_ID_OS = 0,
    MGMT_GROUP_ID_IMAGE,
    MGMT_GROUP_ID_STAT,
    MGMT_GROUP_ID_SETTINGS,
    MGMT_GROUP_ID_LOG,
    MGMT_GROUP_ID_CRASH,
    MGMT_GROUP_ID_SPLIT,
    MGMT_GROUP_ID_RUN,
    MGMT_GROUP_ID_FS,
    MGMT_GROUP_ID_SHELL,
    MGMT_GROUP_ID_ENUM,
    ZEPHYR_MGMT_GRP_BASIC = 63,
    MGMT_GROUP_ID_PERUSER = 64,
}

/// See [`enum mcumgr_err_t`](https://docs.zephyrproject.org/latest/doxygen/html/mgmt__defines_8h.html).
#[derive(FromRepr, Display, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub enum MCUmgrErr {
    MGMT_ERR_EOK = 0,
    MGMT_ERR_EUNKNOWN,
    MGMT_ERR_ENOMEM,
    MGMT_ERR_EINVAL,
    MGMT_ERR_ETIMEOUT,
    MGMT_ERR_ENOENT,
    MGMT_ERR_EBADSTATE,
    MGMT_ERR_EMSGSIZE,
    MGMT_ERR_ENOTSUP,
    MGMT_ERR_ECORRUPT,
    MGMT_ERR_EBUSY,
    MGMT_ERR_EACCESSDENIED,
    MGMT_ERR_UNSUPPORTED_TOO_OLD,
    MGMT_ERR_UNSUPPORTED_TOO_NEW,
    MGMT_ERR_EPERUSER = 256,
}

impl MCUmgrErr {
    /// Converts a raw error code to a string
    pub fn err_to_string(err: i32) -> String {
        const PERUSER: MCUmgrErr = MCUmgrErr::MGMT_ERR_EPERUSER;
        if err < PERUSER as i32 {
            if let Some(err_enum) = Self::from_repr(err) {
                format!("{err_enum}")
            } else {
                format!("MGMT_ERR_UNKNOWN({err})")
            }
        } else {
            format!("{PERUSER}({err})")
        }
    }
}

impl MCUmgrGroup {
    /// Converts a raw group id to a string
    pub fn group_id_to_string(group_id: u16) -> String {
        const PERUSER: MCUmgrGroup = MCUmgrGroup::MGMT_GROUP_ID_PERUSER;
        if group_id < PERUSER as u16 {
            if let Some(group_enum) = Self::from_repr(group_id) {
                format!("{group_enum}")
            } else {
                format!("MGMT_GROUP_ID_UNKNOWN({group_id})")
            }
        } else {
            format!("{PERUSER}({group_id})")
        }
    }
}
