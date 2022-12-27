use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct ScriptCustomEvent {
    pub event_name: String,
    pub event_data: u8,
}

impl PacketType for ScriptCustomEvent {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.event_name.as_str());
        writer.u8(self.event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            event_name: reader.string(),
            event_data: reader.u8(),
        }
    }
}
