use std::net::{IpAddr, Ipv4Addr};

use anyhow::Result;
use bytes::Bytes;
use qp2p::{Config, Endpoint, IncomingMessages, QuicP2p};

const PING: &str = "/ping";
const PONG: &str = "/pong";
const QUIT: &str = "/quit";

async fn process_client(endpoint: Endpoint, mut incoming_messages: IncomingMessages) -> Result<()> {
    while let Some((socket_addr, bytes)) = incoming_messages.next().await {
        if bytes == Bytes::from(PING) {
            endpoint.send_message(Bytes::from(PONG).clone(), &socket_addr).await?;
        } else if bytes == Bytes::from(QUIT) {
            println!("Quit!");
            endpoint.send_message(Bytes::from(QUIT).clone(), &socket_addr).await?;
            break;
        } else {
            println!("Received from {:?} --> {:?}", socket_addr, bytes);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = 5000;

    let peer_peer = QuicP2p::with_config(
        Some(Config {
            local_ip: Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
            local_port: Some(port),
            idle_timeout_msec: Some(1000 * 3600), // 1 hour idle timeout.
            ..Default::default()
        }), 
        Default::default(),
        true
    )?;

    let (endpoint, mut incoming_connections, incoming_messages, mut _disconnection_events) = peer_peer.new_endpoint().await?;
    println!("Listening on: {:?}", endpoint.socket_addr());

    let socket_addr = incoming_connections.next().await.unwrap();
    println!("Client '{:?}' connected", socket_addr);
    process_client(endpoint, incoming_messages).await?;

    let disconnected = _disconnection_events.next().await.unwrap();
    println!("Client '{:?}' disconnected", disconnected);

    Ok(())
}