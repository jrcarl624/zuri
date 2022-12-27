use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct Camera {
    pub camera_entity_unique_id: i64,
    pub target_player_unique_id: i64,
}

impl PacketType for Camera {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.camera_entity_unique_id);
        writer.var_i64(self.target_player_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            camera_entity_unique_id: reader.var_i64(),
            target_player_unique_id: reader.var_i64(),
        }
    }
}
