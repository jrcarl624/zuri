use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::item_stack::ItemStackRequestEntry;

#[derive(Debug, Clone)]
pub struct ItemStackRequest {
    pub requests: Vec<ItemStackRequestEntry>,
}

impl PacketType for ItemStackRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.requests.len() as u32);
        self.requests.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { requests: (0..reader.var_u32()).map(|_| ItemStackRequestEntry::read(reader)).collect() }
    }
}
