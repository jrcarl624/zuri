use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct ServerSettingsResponse {
    pub form_id: u32,
    pub form_data: Bytes,
}

impl PacketType for ServerSettingsResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.form_id);
        writer.byte_slice(&self.form_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            form_id: reader.var_u32(),
            form_data: reader.byte_slice(),
        }
    }
}
