use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use anyhow::Result;
use bytes::Bytes;
use qp2p::{Config, QuicP2p};

const PING: &str = "/ping";
const PONG: &str = "/pong";
const QUIT: &str = "/quit";

#[tokio::main]
async fn main() -> Result<()> {
    let (a, b, c, d, port) = (127, 0, 0 , 1, 5000);
    let peer_peer = QuicP2p::with_config(
        Some(Config {
            local_ip: Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
            idle_timeout_msec: Some(10 * 1000),
            ..Default::default()
        }), 
        Default::default(),
        true
    )?;

    let (endpoint, _, mut incoming_messages, _) = peer_peer.new_endpoint().await?;
    println!("Listening on: {:?}", endpoint.socket_addr());

    let server: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port));
    println!("Connected with server");

    let mut handles = Vec::new();

    handles.push(tokio::spawn(async move {
        println!("Send PING");
        endpoint.send_message(Bytes::from(PING).clone(), &server).await.unwrap();
        
        println!("Send QUIT");
        endpoint.send_message(Bytes::from(QUIT).clone(), &server).await.unwrap();
    }));

    handles.push(tokio::spawn(async move {
        while let Some((_, bytes)) = incoming_messages.next().await {
            if bytes == Bytes::from(PONG) {
                println!("Pong received");
            } else if bytes == Bytes::from(QUIT) {
                println!("Quit!");
                return;
            } else {
                println!("Received --> {:?}", bytes);
            }
        }
    }));

    for handle in handles.drain(..) {
        handle.await.unwrap();
    }

    Ok(())
}