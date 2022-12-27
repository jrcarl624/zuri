use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct ResourcePackChunkData {
    pub uuid: String,
    pub chunk_index: u32,
    pub data_offset: u64,
    pub data: Bytes,
}

impl PacketType for ResourcePackChunkData {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.u32(self.chunk_index);
        writer.u64(self.data_offset);
        writer.byte_slice(&self.data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            chunk_index: reader.u32(),
            data_offset: reader.u64(),
            data: reader.byte_slice(),
        }
    }
}
