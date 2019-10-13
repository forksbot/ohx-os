#![cfg_attr(feature = "external_doc", feature(external_doc))]
#![cfg_attr(feature = "external_doc", doc(include = "../readme.md"))]
#![feature(drain_filter)]

#[macro_use]
extern crate log;

mod config;
mod errors;
mod nm;
mod state_machine;
mod state_machine_portal_helper;
mod utils;

mod dhcp_server;
mod dns_server;
mod http_server;

pub use errors::*;
use structopt::StructOpt;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

// Test if binding to the given address and port works
pub async fn test_udp(server_addr: SocketAddrV4) -> Result<(), CaptivePortalError> {
    let socket = tokio::net::UdpSocket::bind(SocketAddr::V4(server_addr.clone())).await
        .map_err(|_| CaptivePortalError::GenericO(format!("Could not bind to {:?}", server_addr)))?;
    socket.set_broadcast(true)
        .map_err(|_| CaptivePortalError::GenericO(format!("Could not set broadcast flag for {:?}", server_addr)))?;
    Ok(())
}
pub async fn test_tcp(server_addr: SocketAddrV4) -> Result<(), CaptivePortalError> {
    let socket = tokio::net::TcpListener::bind(SocketAddr::V4(server_addr.clone())).await
        .map_err(|_| CaptivePortalError::GenericO(format!("Could not bind to {:?}", server_addr)))?;
    drop(socket);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config: config::Config = config::Config::from_args();

    test_udp(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), config.dns_port)).await?;
    test_udp(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), config.dhcp_port)).await?;
    test_tcp(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), config.listening_port)).await?;

    let mut sm = state_machine::StateMachine::StartUp(config.clone());

    loop {
        sm = if let Some(sm) = sm.progress().await? {
            sm
        } else {
            break;
        }
    }

    Ok(())
}
