extern crate tokio_threadpool;
extern crate tokio_codec;
extern crate futures;
extern crate tokio;
extern crate bytes;

mod stats;

use tokio_codec::BytesCodec;
use tokio::net::{UdpSocket, UdpFramed};
use tokio::runtime::current_thread;
use tokio_threadpool::ThreadPool;

use futures::stream::Stream;
use futures::future;

use std::time::{Duration, Instant};
use std::net::SocketAddr;
use bytes::BytesMut;

static mut DATAGRAMS_RECEIVED: usize = 0;

fn main() {
    println!("Collecting UDP datagrams");

    let socket = UdpSocket::bind(&"127.0.0.1:60002".parse().unwrap()).unwrap();

    let pool = ThreadPool::new();
    let pool = pool.sender();

    let receiving = UdpFramed::new(socket, BytesCodec::new()).split().1
        .map_err(|e| println!("error = {:?}", e))
        .for_each(move |(_buffer, _from): (BytesMut, SocketAddr)| Ok(pool.spawn(future::lazy(|| {
            //println!("Received: {:?}", str::from_utf8(&buffer[..]));
            let start = Instant::now();
            loop {
                let elapsed = Instant::now().duration_since(start);
                if elapsed.ge(&Duration::new(0, 33_000)) {
                    //println!("Elapsed {:?}", elapsed);
                    break;
                }
            }
            unsafe {
                DATAGRAMS_RECEIVED += 1;
            }
            Ok(())
        })).unwrap()));

    current_thread::Runtime::new().unwrap()
        .block_on(stats::with_periodical(receiving, Duration::from_secs(3), || unsafe {
            println!("Received {} datagrams", DATAGRAMS_RECEIVED)
        })).unwrap();
}
