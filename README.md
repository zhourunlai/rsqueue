# Rsqueue

A task queue using redis as backend.

## Usage

```rust
extern crate rsqueue;

use rsqueue::{Job, Queue};

let client = redis::Client::open("redis://127.0.0.1/").unwrap();
let conn = client.get_connection().unwrap();
let name = "q";
let queue = Queue::new(name.to_string(), conn);
let job = Job::new();

// publish
let _ = queue.publish(job);

// subscribe
while let Some(job) = queue.subscribe::<Job>() {
    if job.is_err() {
        continue;
    }
    let job = job.unwrap();

    // Job: d55f3966-1253-43e5-aef0-cef84d7f0637
    println!("Job: {}", job.id);
}
```
