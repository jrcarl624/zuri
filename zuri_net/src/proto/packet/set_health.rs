use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server. It sets the health of the player it is sent to. The SetHealth packet should no longer be used.
/// Instead, the health attribute should be used so that the health and maximum health may be changed directly.
#[derive(Debug, Clone)]
pub struct SetHealth {
    /// The new health of the player.
    pub health: i32,
}

impl PacketType for SetHealth {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.health);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { health: reader.var_i32() }
    }
}
