use num_traits::{ToPrimitive, FromPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::inventory::Window;
use crate::proto::types::container::ContainerDataKey;

/// Sent by the server to update specific data of a single container, meaning a block such as a furnace or a brewing
/// stand. This data is usually used by the client to display certain features client-side.
#[derive(Debug, Clone)]
pub struct ContainerSetData {
    /// The window that the packet modifies. It must point to one of the windows that the client currently has opened.
    pub window: Window,
    /// The key of the property. It is one of the constants that can be found above. Multiple properties share the same
    /// key, but the functionality depends on the type of the container that the data is set to.
    pub key: ContainerDataKey,
    /// The value of the property. Its use differs per property.
    pub value: i32,
}

impl PacketType for ContainerSetData {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.window.to_u8().unwrap());
        writer.var_i32(self.key.0);
        writer.var_i32(self.value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: Window::from_u8(reader.u8()).unwrap(),
            key: ContainerDataKey(reader.var_i32()),
            value: reader.var_i32(),
        }
    }
}
