use tokio::timer::Interval;
use tokio::runtime::current_thread;
use futures::stream::iter_ok;
use futures::Stream;

use clap::Clap;

use std::time::{Duration, Instant};
use std::net::UdpSocket;

mod batches;
mod stats;
mod units;

type DPS = usize;
type Delay = usize;

static mut DATAGRAMS_SENT: usize = 0;

#[derive(Clap)]
#[clap(version = "1.0")]
struct Options {
    #[clap(short, long)]
    url: String,
    #[clap(short, long)]
    delay: Option<String>,
    #[clap(short, long)]
    frequency: Option<usize>,
    #[clap(short, long)]
    total_amount: usize
}

fn main() {
    let options: Options = Options::parse();

    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").unwrap();

    if options.delay.is_none() || options.frequency.is_none() {
        println!("Please, specify timing using either delay between datagrams or their frequency.");
    }
    if options.delay.is_some() && options.frequency.is_some() {
        println!("You can specify only one timing parameter (either delay or frequency).");
    }

    let (delay_ns, dps) = if let Some(delay) = &options.delay {
        let delay = units::parse_duration(&delay).unwrap();
        timing_from_delay(delay)
    } else {
        timing_from_dps(options.frequency.unwrap())
    };

    println!("Stressing the receiver with UDP datagrams!");
    println!("Delay between datagrams: {}ns", delay_ns);
    println!("DPS to test: {}", dps);

    let (delay_ns, batch_size) = if delay_ns < 1_000_000 {
        println!("[delays less 1ms are simulated with batching]");
        let k = (1e6 / delay_ns as f64).ceil() as usize;
        (delay_ns * k, k as usize)
    } else {
        (delay_ns, 1)
    };
    println!("Real delay: {}ms", delay_ns as f64 / 1e6);
    println!("Batch size: {}", batch_size);

    let samples: Vec<String> = vec![
        "heey", "what's up?", "me too",
        "how was your weekend?", "awesome, dude!",
        "weather sucks", "when is old-wife's summer?",
        "nevermind", "alright, it's late",
        "see you", "bye"
    ].into_iter().map(|s| bloat_message(s, 420)).collect();

    let batches_amount = options.total_amount / batch_size;
    let batches = batches::prepare(samples, batch_size, true)
        .take(batches_amount);

    let send = move |data: &[u8]| {
        match socket.send_to(data, &options.url) {
            Ok(bytes) => Some(bytes as u64),
            Err(e) => {
                println!("Error: {}", e);
                None
            },
        }
    };

    let sending = Interval::new(Instant::now(), Duration::from_nanos(delay_ns as u64))
        .zip(iter_ok(batches))
        .map_err(|e| println!("Error: {}", e))
        .for_each(move |(_time, batch): (Instant, Vec<String>)| {
            for data in batch.iter() {
                send(data.as_bytes());
                unsafe {
                    DATAGRAMS_SENT += 1;
                }
            }
            Ok(())
        });

    current_thread::Runtime::new().unwrap()
        .block_on(stats::with_periodical(sending, Duration::from_secs(1), || unsafe {
            println!("Sent {} datagrams", DATAGRAMS_SENT);
        }))
        .unwrap();
}

fn bloat_message(msg: &str, size: usize) -> String {
    let times: f32 = (size as f32) / (msg.as_bytes().len() as f32);
    if times > 1.0 {
        let times = times.ceil() as usize;
        vec![msg.chars(); times]
            .into_iter()
            .flatten()
            .collect()
    } else {
        msg.into()
    }
}

fn timing_from_delay(delay_ns: usize) -> (Delay, DPS) {
    (delay_ns, dps_from_delay(delay_ns))
}

fn timing_from_dps(dps: usize) -> (Delay, DPS) {
    (delay_from_dps(dps), dps)
}

fn delay_from_dps(dps: usize) -> usize {
    1e9 as usize / dps
}

fn dps_from_delay(delay_ns: usize) -> usize {
    1e9 as usize / delay_ns
}