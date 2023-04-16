use std::thread;
use std::time::Duration;

use anyhow::Result;
use notify_rust::Notification;
use rand::Rng;

#[derive(Debug)]
struct SmallRemainder<'a> {
    min_sec: u64,
    max_sec: u64,
    text: &'a str
}

fn run_notification_small(text: &str) -> Result<()> {
    run_notification(text, "")
}

fn run_notification(summary: &str, body: &str) -> Result<()> {
    Notification::new().summary(summary).body(body).show()?;
    Ok(())
}

static SMALL_REMAINDERS: [SmallRemainder; 3] = [
    SmallRemainder {
        min_sec: 15*60,
        max_sec: 60*60,
        text: "Remember to have good posture.",
    },
    SmallRemainder {
        min_sec: 15*60,
        max_sec: 60*60,
        text: "Remember to breathe through your nose.",
    },
    SmallRemainder {
        min_sec: 60*60,
        max_sec: 2*60*60,
        text: "Remember to drink more water.",
    },
];



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut threads = Vec::new();
    for rem in SMALL_REMAINDERS.iter() {
        let t = thread::spawn(|| {
            let mut rng = rand::thread_rng();
           loop {
            let secs = rng.gen_range(rem.min_sec..rem.max_sec);
            thread::sleep(Duration::from_secs(secs));
            run_notification_small(&rem.text).unwrap(); // TODO
           } 
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap(); // TODO
    }
    Ok(())
}
