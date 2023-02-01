use std::sync::atomic::{AtomicU64, AtomicBool};
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;
// use std::thread::ThreadId;
use std::time::Duration;

// use std::io::{self, Write};
// use indicatif::ProgressBar;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release); // Everything before this store ..
    });

    while !READY.load(Acquire) {  // ... is visible after this loads true 
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}", DATA.load(Relaxed));
}
