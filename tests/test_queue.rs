extern crate redis;
extern crate rsqueue;

use rsqueue::{Job, Queue};

#[test]
fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    let name = "q";
    let queue = Queue::new(name.to_string(), conn);
    let job = Job::new();

    let _ = queue.publish(job);

    while let Some(job) = queue.subscribe::<Job>() {
        if job.is_err() {
            continue;
        }

        let job = job.unwrap();

        println!("Job: {}", job.id);
    }
}
