#[macro_use]
extern crate criterion;

use async_std::task;
use blackhole_bin::server::serve;
use criterion::Criterion;
use std::thread;
use std::time::Duration;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

fn c100(c: &mut Criterion) {
    bench_serve(c, 10)
}

fn bench_serve(c: &mut Criterion, concurrency: u32) {
    thread::spawn(move || {
        let addr = ("127.0.0.1", 8080);
        let _ = serve(addr);
    });

    thread::sleep(Duration::from_secs(1));

    let client = surf::Client::new();
    c.bench_function("bench_serve", |b| {
        b.iter(|| {
            task::block_on(async {
                let mut parallel: Vec<_> = Vec::new();
                for _i in 0..concurrency {
                    let res = client.get("http://127.0.0.1/path?param=1").recv_string();
                    parallel.push(res)
                }
                let result: std::result::Result<Vec<_>, _> =
                    futures::future::try_join_all(parallel).await;

                result
            })
        })
    });
}

criterion_group!(
  name = benches;
  config = custom_criterion();
  targets = c100,
);
criterion_main!(benches);
