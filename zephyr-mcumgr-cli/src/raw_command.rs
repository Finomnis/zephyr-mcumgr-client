use crate::args::RawCommandOp;

impl zephyr_mcumgr::commands::McuMgrCommand for crate::args::RawCommand {
    type Payload = ciborium::Value;
    type Response = ciborium::Value;

    fn is_write_operation(&self) -> bool {
        match self.op {
            RawCommandOp::Read => false,
            RawCommandOp::Write => true,
        }
    }

    fn group_id(&self) -> u16 {
        self.group_id
    }

    fn command_id(&self) -> u8 {
        self.command_id
    }

    fn data(&self) -> &ciborium::Value {
        &self.data
    }
}
