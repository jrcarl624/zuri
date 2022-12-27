use bytes::Bytes;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::resource_pack::ResourcePackType;

#[derive(Debug, Clone)]
pub struct ResourcePackDataInfo {
    pub uuid: String,
    pub data_chunk_size: u32,
    pub chunk_count: u32,
    pub size: u64,
    pub hash: Bytes,
    pub premium: bool,
    pub pack_type: ResourcePackType,
}

impl PacketType for ResourcePackDataInfo {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.u32(self.data_chunk_size);
        writer.u32(self.chunk_count);
        writer.u64(self.size);
        writer.byte_slice(&self.hash);
        writer.bool(self.premium);
        writer.u8(self.pack_type.to_u8().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            data_chunk_size: reader.u32(),
            chunk_count: reader.u32(),
            size: reader.u64(),
            hash: reader.byte_slice(),
            premium: reader.bool(),
            pack_type: ResourcePackType::from_u8(reader.u8()).unwrap(),
        }
    }
}
