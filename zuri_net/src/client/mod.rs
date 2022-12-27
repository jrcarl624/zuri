use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc::sync_channel;
use async_trait::async_trait;
use oauth2::basic::BasicTokenResponse;
use rust_raknet::RaknetSocket;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use crate::proto::packet::Packet;

use crate::client::login::LoginSequence;
use crate::client::data::{ClientData, IdentityData};
use crate::connection::{Connection, ConnError, Sequence, SequenceConn};

mod login;
mod auth;
mod data;
#[cfg(feature = "bevy")]
pub mod plugin;

pub struct Client<H: Handler + Send + 'static> {
    conn: Arc<Mutex<Connection>>,
    handler: Arc<Mutex<H>>,
    seq_chan: Sender<Sender<Packet>>,

    client_data: ClientData,
    identity_data: IdentityData,
}

impl<H: Handler + Send + 'static> Client<H> {
    pub async fn connect(
        ip: SocketAddr,
        client_data: ClientData,
        identity_data: Option<IdentityData>,
        live_token: Option<BasicTokenResponse>,
        handler: H,
    ) -> Result<Self, String> {
        let socket = RaknetSocket::connect_with_version(&ip, 11).await.expect("TODO: panic message"); // TODO: panic message


        let (send, recv) = channel(1);
        let (seq_send, seq_recv) = channel(1);
        let client = Self {
            conn: Arc::new(Mutex::new(Connection::new(socket))),
            handler: Arc::new(Mutex::new(handler)),
            seq_chan: seq_send,

            client_data,
            identity_data: identity_data.unwrap_or(IdentityData {
                xuid: "".into(),
                identity: "".into(),
                display_name: "".into(),
                title_id: None,
            }), // TODO: Parse from live_token if present.
        };
        client.exec_sequence(LoginSequence::new(
            &client.client_data,
            &client.identity_data,
            live_token,
            false,
        )).await.unwrap();

        tokio::spawn(Self::read_loop(send, client.conn.clone(), seq_recv));
        tokio::spawn(Self::handle_loop(recv, client.handler.clone(), client.conn.clone()));
        Ok(client)
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        //self.conn.close()
        todo!()
    }

    pub async fn write_packet(&mut self, packet: &mut Packet) -> Result<(), ConnError> {
        let mut mu = self.handler.lock().await;
        mu.handle_outgoing(packet).await;
        drop(mu);

        self.conn.lock().await.write_packet(packet);
        Ok(())
    }

    pub async fn flush(&mut self) -> Result<(), ConnError> {
        self.conn.lock().await.flush().await
    }

    pub async fn exec_sequence<E>(&self, seq: impl Sequence<E>) -> Result<(), E> {
        let (send, recv) = channel(1);
        self.seq_chan.send(send).await.unwrap();
        seq.execute(recv, SequenceConn::new(self.conn.clone())).await
    }

    async fn read_loop(chan: Sender<Packet>, conn: Arc<Mutex<Connection>>, mut seq_chan: Receiver<Sender<Packet>>) {
        let mut seq = None;
        loop {
            let result = conn.lock().await.read_next_packet().await;

            if seq.is_none() {
                if let Ok(s) = seq_chan.try_recv() {
                    seq = Some(s);
                }
            }
            match result {
                Ok(pk) => {
                    // If there is an ongoing sequence, send it a clone of the packet first.
                    let mut complete_seq = false;
                    if let Some(seq_sender) = &mut seq {
                        if let Err(_) = seq_sender.send(pk.clone()).await {
                            complete_seq = true;
                        }
                    }
                    if complete_seq {
                        seq = None;
                    }
                    // We can call expect here: the handler stops only if the read loop stops first.
                    chan.send(pk).await.expect("Could not send packet to handler");
                }
                Err(_) => {
                    return;
                }
            };
        }
    }

    async fn handle_loop<T: Handler + Send>(mut chan: Receiver<Packet>, handler: Arc<Mutex<T>>, conn: Arc<Mutex<Connection>>) {
        loop {
            if let Some(pk) = chan.recv().await {
                let mut response = handler.lock().await.handle_incoming(pk).await;

                let mut mu = conn.lock().await;
                for pk in &mut response {
                    mu.write_packet(pk);
                }
                mu.flush().await.unwrap();
            } else {
                handler.lock().await.handle_disconnect(None).await; // todo: reason
                return;
            }
        }
    }
}

/// Handles events such as incoming packets from the connection.
#[async_trait]
pub trait Handler {
    async fn handle_incoming(&mut self, pk: Packet) -> Vec<Packet> { vec![] }
    async fn handle_outgoing(&mut self, pk: &mut Packet) {}

    async fn handle_disconnect(&mut self, reason: Option<String>) {}
}

// tests
#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::sleep;
    use uuid::Uuid;
    use super::*;

    #[tokio::test]
    async fn connect_test() {
        let client = Client::connect(
            "127.0.0.1:19131".parse().unwrap(),
            ClientData::default(),
            Some(IdentityData {
                identity: Uuid::new_v4().to_string(),
                display_name: "Zuri".into(),
                xuid: String::new(),
                title_id: None,
            }),
            None,
            TestHandler,
        ).await.unwrap();
        sleep(Duration::from_secs(3)).await;
    }

    struct TestHandler;

    #[async_trait]
    impl Handler for TestHandler {
        async fn handle_incoming(&mut self, pk: Packet) -> Vec<Packet> {
            println!("Incoming packet: {:?}", pk);
            vec![]
        }

        async fn handle_outgoing(&mut self, pk: &mut Packet) {
            println!("Outgoing packet: {:?}", pk);
        }
    }
}
