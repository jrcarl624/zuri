use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::{Dimension, SubChunkOffset};

#[derive(Debug, Clone)]
pub struct SubChunkRequest {
    pub dimension: Dimension,
    pub position: IVec3,
    pub offsets: Vec<SubChunkOffset>,
}

impl PacketType for SubChunkRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.dimension.to_i32().unwrap());
        writer.block_pos(self.position);

        writer.u32(self.offsets.len() as u32);
        self.offsets.iter().for_each(|offset| offset.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            dimension: Dimension::from_i32(reader.var_i32()).unwrap(),
            position: reader.block_pos(),
            offsets: (0..reader.u32()).map(|_| SubChunkOffset::read(reader)).collect(),
        }
    }
}
