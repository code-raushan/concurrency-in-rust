use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
fn main() {
    let counter = AtomicU32::new(0);
    thread::scope(|scope| {
        for _ in 0..10 {
            scope.spawn(|| {
                for _ in 0..1_000 {
                    counter.fetch_add(1, Relaxed);
                }
            });
        }
    });
    println!("Counter: {}", counter.load(Relaxed));
}
