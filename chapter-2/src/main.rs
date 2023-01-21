use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{self, ThreadId};
use std::time::Duration;
// use std::io::{self, Write};

fn do_work(num: i32, tid: ThreadId) {
    println!("Processing: {num}, Thread: {tid:?}");
}

fn main() {
    let num_done = &AtomicUsize::new(0);

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
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_millis(100));
        }
    });
    println!("Done!");
}
