use crate::transport::Transport;

pub struct Connection<T: Transport> {
    transport: T,
}

impl<T: Transport> Connection<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn execute(&mut self) {
        self.transport
            .send_frame(true, 0, group_id, command_id, data)
            .unwrap();
    }
}
