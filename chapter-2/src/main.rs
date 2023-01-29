use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{self, ThreadId};
use std::time::Duration;

// use std::io::{self, Write};
use indicatif::ProgressBar;

fn gen_key() -> u64 {
    42
}

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);

    if key == 0 {
        let new_key = gen_key();
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}

fn main() {
    let num_done = &AtomicU64::new(0);

    thread::scope(|s| {
        // four bg threads to process 100 items, 25 each
        for t in 0..4 {
            s.spawn(move || {
                let tid = thread::current().id();
                let mut thread_keys = vec![];
                for i in 0..25 {
                    thread_keys.push(t * 25 + get_key() + i);
                    num_done.fetch_add(1, Relaxed);
                    thread::sleep(Duration::from_millis(100));
                }
                println!("{tid:?}: {thread_keys:?}");
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
            thread::sleep(Duration::from_millis(10));
        }
        bar.finish();
    });
    println!("Done!");
}
