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

    let (endpoint, _, _, _) = peer_peer.new_endpoint().await?;
    println!("Listening on: {:?}", endpoint.socket_addr());

    let server: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port));
    let (mut send_stream, mut receive_stream) = endpoint.open_bidirectional_stream(&server).await?;
    println!("Connected with server");

    let mut handles = Vec::new();

    handles.push(tokio::spawn(async move {
        println!("Send PING");
        let msg = Bytes::from(PING);
        send_stream.send_user_msg(msg.clone()).await.unwrap();
        send_stream.finish().await.unwrap();
        
        // println!("Send QUIT");
        // send_stream.send_user_msg(Bytes::from(QUIT).clone()).await.unwrap();
    }));

    handles.push(tokio::spawn(async move {
        loop {
            match receive_stream.next().await {
                Ok(value) => {
                    if value == Bytes::from(PONG) {
                        println!("Pong received");
                    } else if value == Bytes::from(QUIT) {
                        println!("Quit!");
                        return;
                    } else {
                        println!("Received --> {:?}", value);
                    }
                },
                Err(_) => {}
            };
            
        }
    }));


    for handle in handles.drain(..) {
        handle.await.unwrap();
    }

    Ok(())
}