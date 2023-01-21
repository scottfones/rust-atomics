use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{self, ThreadId};
use std::time::Duration;

// use std::io::{self, Write};
use indicatif::ProgressBar;

fn do_work(num: i32, tid: ThreadId) {
    if num < 0 {
        println!("Processing: {num}, Thread: {tid:?}");
    }
}

fn main() {
    let num_done = &AtomicU64::new(0);

    thread::scope(|s| {
        // four bg threads to process 100 items, 25 each
        for t in 0..4 {
            s.spawn(move || {
                let tid = thread::current().id();
                for i in 0..25 {
                    do_work(t * 25 + i, tid);
                    num_done.fetch_add(1, Relaxed);
                    thread::sleep(Duration::from_millis(100));
                }
            });
        }

        // main thread for status updates, updates every second
        let bar = ProgressBar::new(100);
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            bar.set_position(n);
            thread::sleep(Duration::from_millis(100));
        }
        bar.finish();
    });
    println!("Done!");
}
