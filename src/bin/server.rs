#![feature(impl_trait_in_assoc_type)]
use miniredis::LogLayer;
use std::net::SocketAddr;

use miniredis::{S};

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::miniredis::ItemServiceServer::new(S::new())
        .layer_front(LogLayer)
        .run(addr)
        .await
        .unwrap();
}
