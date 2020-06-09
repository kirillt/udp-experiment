extern crate tokio_threadpool;
extern crate tokio_codec;
extern crate futures;
extern crate tokio;
extern crate bytes;

use clap::Clap;

use tokio_codec::BytesCodec;
use tokio::net::{UdpSocket, UdpFramed};
use tokio::runtime::current_thread;
use tokio_threadpool::ThreadPool;

use futures::stream::Stream;
use futures::future;

use std::time::{Duration, Instant};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use bytes::BytesMut;

mod stats;
mod units;

#[derive(Clap)]
#[clap(version = "1.0")]
struct Options {
    #[clap(short, long)]
    port: u16,
    #[clap(short, long)]
    report_interval: String
}

static mut DATAGRAMS_RECEIVED: usize = 0;

fn main() {
    let options: Options = Options::parse();

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), options.port);
    let socket = UdpSocket::bind(&address).unwrap();

    let pool = ThreadPool::new();
    let pool = pool.sender();

    let report_interval = units::parse_duration(&options.report_interval).unwrap();

    println!("Collecting UDP datagrams");

    let receiving = UdpFramed::new(socket, BytesCodec::new()).split().1
        .map_err(|e| println!("error = {:?}", e))
        .for_each(move |(_buffer, _from): (BytesMut, SocketAddr)| Ok(pool.spawn(future::lazy(|| {
            let start = Instant::now();
            loop {
                let elapsed = Instant::now().duration_since(start);
                if elapsed.ge(&Duration::new(0, 33_000)) {
                    break;
                }
            }
            unsafe {
                DATAGRAMS_RECEIVED += 1;
            }
            Ok(())
        })).unwrap()));

    current_thread::Runtime::new().unwrap()
        .block_on(stats::with_periodical(receiving, Duration::from_nanos(report_interval as u64), || unsafe {
            println!("Received {} datagrams", DATAGRAMS_RECEIVED)
        })).unwrap();
}
