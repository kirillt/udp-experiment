use std::time::{Duration, Instant};
use futures::{Future, Stream};
use tokio::timer::Interval;

pub fn with_periodical<F>(work: F, period: Duration, callback: fn()) -> impl Future<Item = (), Error = ()>
where F: Future<Item = (), Error = ()> {
    let periodic = {
        let callback = callback.clone();

        Interval::new(Instant::now(), period)
            .map_err(|e| println!("Error: {}", e))
            .for_each(move |_| {
                callback();
                Ok(())
            })
    };

    let done = work.map(move|_| callback());
    periodic.select(done)
        .map_err(|_| {})
        .map(|_| {})
}