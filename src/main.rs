use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use human_panic::setup_panic;
use notify_rust::Notification;
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SmallRemainder {
    min_sec: u64,
    max_sec: u64,
    text: String,
}

fn run_notification_small(text: &str) -> Result<()> {
    run_notification(text, "")
}

fn run_notification(summary: &str, body: &str) -> Result<()> {
    Notification::new().summary(summary).body(body).show()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic!();

    let file = File::open("example.json")?;
    let reader = BufReader::new(file);
    let small_remainders: Vec<SmallRemainder> = serde_json::from_reader(reader)?;

    let mut threads = Vec::new();
    for rem in small_remainders.into_iter() {
        let t = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let secs = rng.gen_range(rem.min_sec..rem.max_sec);
                thread::sleep(Duration::from_secs(secs));
                run_notification_small(&rem.text).unwrap();
            }
        });
        threads.push(t);
    }
    println!("Press CTRL+C to stop");
    for t in threads {
        t.join().unwrap();
    }
    Ok(())
}
