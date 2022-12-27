use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct CodeBuilder {
    pub url: String,
    pub should_open_code_builder: bool,
}

impl PacketType for CodeBuilder {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.url.as_str());
        writer.bool(self.should_open_code_builder);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            url: reader.string(),
            should_open_code_builder: reader.bool(),
        }
    }
}
