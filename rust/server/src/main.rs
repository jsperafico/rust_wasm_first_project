use std::net::{IpAddr, Ipv4Addr};

use anyhow::Result;
use bytes::Bytes;
use qp2p::{Config, QuicP2p};

const PING: &str = "/ping";
const PONG: &str = "/pong";
const QUIT: &str = "/quit";

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

    let (endpoint, mut incoming_connections, mut incoming_messages, mut _disconnection_events) = peer_peer.new_endpoint().await?;
    println!("Listening on: {:?}", endpoint.socket_addr());

    let socket_addr = incoming_connections.next().await.unwrap();
    println!("Client '{:?}' connected", socket_addr);

    let mut handles = Vec::new();

    handles.push(tokio::spawn(async move {
        while let Some((socket_addr, bytes)) = incoming_messages.next().await {
            if bytes == Bytes::from(PING) {
                println!("Ping received!");
                endpoint.send_message(Bytes::from(PONG).clone(), &socket_addr).await.unwrap();
            } else if bytes == Bytes::from(QUIT) {
                println!("Quit!");
                endpoint.send_message(Bytes::from(QUIT).clone(), &socket_addr).await.unwrap();
                break;
            } else {
                println!("Received from {:?} --> {:?}", socket_addr, bytes);
            }
        }
    }));

    
    handles.push(tokio::spawn(async move {
        let disconnected = _disconnection_events.next().await.unwrap();
        println!("Client '{:?}' disconnected", disconnected);
    }));

    for handle in handles.drain(..) {
        handle.await.unwrap();
    }

    Ok(())
}