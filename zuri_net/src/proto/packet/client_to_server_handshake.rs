use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client in response to a ServerToClientHandshake packet sent by the server. It is the first encrypted
/// packet in the login handshake and serves as a confirmation that encryption is correctly initialised client side.
#[derive(Debug, Clone)]
pub struct ClientToServerHandshake;

impl PacketType for ClientToServerHandshake {
    fn write(&self, _: &mut Writer) {}

    fn read(_: &mut Reader) -> Self {
        Self {}
    }
}
