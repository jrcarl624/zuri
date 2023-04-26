use std::any::TypeId;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use p384::ecdsa::SigningKey;
use rust_raknet::error::RaknetError;
use rust_raknet::{RaknetSocket, Reliability};
use tokio::sync::Mutex;

use crate::chan::PkReceiver;
use crate::compression::Compression;
use crate::encode::Encoder;
use crate::encryption::Encryption;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::Packet;

pub struct Connection {
    socket: RaknetSocket,

    buffered_batch: Mutex<Vec<Vec<u8>>>,
    queued_packets: Mutex<VecDeque<Packet>>,

    signing_key: SigningKey,

    encoder: Mutex<Encoder>,
}

impl Connection {
    pub fn new(socket: RaknetSocket) -> Self {
        Self {
            socket,

            buffered_batch: Mutex::new(Vec::new()),
            queued_packets: Mutex::new(VecDeque::new()),

            signing_key: SigningKey::random(&mut rand::thread_rng()),

            encoder: Mutex::new(Encoder::default()),
        }
    }

    pub fn peer_addr(&self) -> SocketAddr {
        // Unwrap can be done safely here: peer_addr() always returns Ok().
        self.socket.peer_addr().unwrap()
    }

    pub async fn close(&self) -> Result<(), ConnError> {
        self.socket.close().await?;
        Ok(())
    }

    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    pub async fn set_compression(&self, compression: Compression) {
        self.encoder.lock().await.set_compression(compression);
    }

    pub async fn set_encryption(&self, encryption: Encryption) {
        self.encoder.lock().await.set_encryption(encryption);
    }

    pub async fn flush(&self) -> Result<(), ConnError> {
        let mut batch_mu = self.buffered_batch.lock().await;
        if batch_mu.is_empty() {
            return Ok(());
        }
        let batch = self
            .encoder
            .lock()
            .await
            .encode(&mut *batch_mu)
            .map_err(|s| ConnError::EncodeError(s))?;
        batch_mu.clear();
        drop(batch_mu);

        Ok(self
            .socket
            .send(&batch, Reliability::ReliableOrdered)
            .await?)
    }

    pub async fn write_packet(&self, packet: &Packet) {
        let mut writer = Writer::new(0); // TODO: Shield ID
        packet.write(&mut writer);

        self.buffered_batch.lock().await.push(writer.into());
    }

    pub async fn read_next_packet(&self) -> Result<Packet, ConnError> {
        loop {
            let mut queue = self.queued_packets.lock().await;
            if let Some(packet) = queue.pop_front() {
                return Ok(packet);
            }
            *queue = self.read_next_batch().await?.into();
        }
    }

    async fn read_next_batch(&self) -> Result<Vec<Packet>, ConnError> {
        let encoded = self.socket.recv().await?;
        let batch = self
            .encoder
            .lock()
            .await
            .decode(&mut encoded.into())
            .map_err(|e| ConnError::DecodeError(e))?;

        let mut packets = Vec::with_capacity(batch.len());
        for buf in batch {
            let mut reader = Reader::from_buf(Bytes::from(buf), 0);
            packets.push(Packet::read(&mut reader));
        }
        Ok(packets)
    }
}

#[async_trait]
pub trait Sequence<T> {
    async fn execute(
        self,
        reader: PkReceiver,
        conn: Arc<Connection>,
        expecter: Arc<ExpectedPackets>,
    ) -> T;
}

/// Keeps track of which packets are currently expected by a [Sequence].
#[derive(Default, Debug)]
pub struct ExpectedPackets {
    /// A queue of expected packet types.
    packets: Mutex<Vec<TypeId>>,
}

impl ExpectedPackets {
    /// Marks a packet of type `T` as 'expected'.
    ///
    /// Packets expected by a sequence will be passed onto
    /// that sequence instead of [Connection::read_next_batch] or [Connection::read_next_packet].
    ///
    /// If multiple instances of the same packet type are expected, this method can be called
    /// multiple times.
    pub async fn queue<T: TryFrom<Packet> + 'static>(&self) {
        self.packets.lock().await.push(TypeId::of::<T>());
    }

    /// Mark one packets of type `T` as no longer expected.
    ///
    /// Returns true if at least one packet of type `T` was expected before calling this method and
    /// thus was successfully retracted.
    /// Returns false when no such packet was currently in the expected queue.
    pub async fn retract<T: TryFrom<Packet> + 'static>(&self) -> bool {
        let mut packets = self.packets.lock().await;
        packets
            .iter()
            // Find the index of the first match.
            .position(|t| *t == TypeId::of::<T>())
            // Remove the match only if it exists.
            .map(|index| packets.remove(index))
            // Return true if an index was found.
            .is_some()
    }

    /// Returns true if any packets are currently in the expected packets queue.
    pub async fn expecting_any(&self) -> bool {
        !self.packets.lock().await.is_empty()
    }

    // Internal methods
    // ----------------

    /// Check whether a packet is expected.
    ///
    /// The type of packet `pk` is compared against all currently expected packet types, and if at
    /// least one match is found, true is returned.
    pub(crate) async fn expected(&self, pk: &Packet) -> bool {
        self.packets.lock().await.contains(&pk.inner_type_id())
    }

    /// Remove a single packet type from the expected queue.
    ///
    /// This is a 'dynamic' version of [ExpectedPackets::retract] that panics when no packet of
    /// `pk`'s type is present.
    pub(crate) async fn remove(&self, pk: &Packet) {
        let mut packets = self.packets.lock().await;
        let index = packets
            .iter()
            .position(|t| *t == pk.inner_type_id())
            .unwrap();
        packets.remove(index);
    }
}

#[derive(Debug)]
pub enum ConnError {
    EncodeError(String),
    DecodeError(String),
    RakNetError(RaknetError),
}

impl Display for ConnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnError::EncodeError(s) => f.write_str(&format!("Error encoding packet: {}", s)),
            ConnError::DecodeError(s) => f.write_str(&format!("Error decoding packet: {}", s)),
            ConnError::RakNetError(err) => f.write_str(&format!("RakNet error: {:?}", err)),
        }
    }
}

impl Error for ConnError {}

impl From<RaknetError> for ConnError {
    fn from(value: RaknetError) -> Self {
        Self::RakNetError(value)
    }
}
